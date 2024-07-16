// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at <http://mozilla.org/MPL/2.0/>.

use crate::App;
use std::any::Any;

/// A collection of application logic and configuration steps that can be added to an [`App`] to
/// customize its startup and runtime behavior.  Instances of types that implement the [`Plugin`]
/// trait may be registered with an [`App`] instance.  When a plugin is registered, the plugin's
/// [`Plugin::build`] method is called, allowing the plugin to configure the [`App`] instance as
/// needed.
///
/// By default, a given type of plugin can only be added once to an [`App`], and this is enforced at
/// runtime.  This is a safety check as plugins may contain non-idempotent operations.  For plugins
/// which explicitly expect to be added multiple times, such plugins will need to override the
/// [`Plugin::is_unique`] method to return `false`.  This will allow the plugin to be added multiple
/// times to the same [`App`] instance (presumably with different parameters, e.g. a
/// `ConfigurationFilePlugin` that specifies a file to check on startup for runtime options, and
/// more than one file location is to be checked).
pub trait Plugin: Any {
    /// Configure the [`App`] instance to which this plugin was added.
    fn build(&self, app: &mut App);

    /// Whether a given [`App`] instance may include more than one instance of this plugin type, as
    /// identified by [`std::any::TypeId`].  The default implementation of this method returns
    /// `true`, as a safety check against plugins that perform non-idempotent steps at runtime. This
    /// behavior only needs to be overridden for plugin implementations with internal state where it
    /// explicitly makes sense to add multiple instances to the same [`App`].  For example:
    ///
    /// ```
    /// # use atomcad_app::{App, Plugin};
    /// pub struct ConfigurationFilePlugin(std::path::PathBuf);
    ///
    /// impl Plugin for ConfigurationFilePlugin {
    ///     fn build(&self, app: &mut App) {
    ///         // Read the configuration file and apply the settings to the app
    ///     }
    ///     fn is_unique(&self) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// App::new("Duplicate Plugins".into())
    ///     .add_plugin(ConfigurationFilePlugin("~/.config/duplicate-plugins.toml".into()))
    ///     .add_plugin(ConfigurationFilePlugin("/etc/duplicate-plugins/config.toml".into())).run();
    /// ```
    fn is_unique(&self) -> bool {
        true
    }

    /// Return the [`TypeId`](std::any::TypeId) of the plugin type.  The default implementation is
    /// exactly what you would expect: the [`TypeId`](std::any::TypeId) of the plugin type itself,
    /// and most users should have no reason to change this.  One reason you might is if you wanted
    /// to have two separate plugin types that cannot both be added to the same [`App`] instance.
    /// For example:
    ///
    /// ```should_panic
    /// # use atomcad_app::{App, Plugin};
    /// pub struct ConflictingPluginTag;
    ///
    /// pub struct PluginA;
    /// impl Plugin for PluginA {
    ///     fn build(&self, app: &mut App) {}
    ///     fn id(&self) -> std::any::TypeId {
    ///         std::any::TypeId::of::<ConflictingPluginTag>()
    ///     }
    /// }
    ///
    /// pub struct PluginB;
    /// impl Plugin for PluginB {
    ///     fn build(&self, app: &mut App) {}
    ///     fn id(&self) -> std::any::TypeId {
    ///         std::any::TypeId::of::<ConflictingPluginTag>()
    ///     }
    /// }
    ///
    /// App::new("Conflicting Plugins".into())
    ///     .add_plugin(PluginA)
    ///     .add_plugin(PluginB);
    /// ```
    fn id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }
}

/// As it is commonly the case that a plugin only implements a single [`Plugin::build`] method that
/// configures the application runner, a convenience implementation is provided that allows any
/// closure to be used in place of a full plugin type.
impl<T: Fn(&mut App) + Any> Plugin for T {
    fn build(&self, app: &mut App) {
        self(app);
    }
}

// End of File
