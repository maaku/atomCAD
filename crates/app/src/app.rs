// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at <http://mozilla.org/MPL/2.0/>.

use crate::Plugin;
use core::num::NonZero;
use std::collections::HashSet;

/// The status code to use when exiting the application.  It is the value returned by the
/// application runner, and passed back to the callee of [`App::run`].
pub enum AppExit {
    /// The application exited successfully.  This results in a status code of 0 on POSIX systems.
    Success,
    /// The application exited with an error.  The status code is captured in the `NonZero<u8>`
    /// value.  The status code must be non-zero because POSIX systems reserve a status code of 0 to
    /// indicate success, and although `std::process::exit` will accept an `i32` value, it is only
    /// guaranteed that the lowest 8 bits are passed to the calling shell.  A hypothetical
    /// `Error(0x100)` would thus be falsely interpreted as `Error(0)`.
    Error(NonZero<u8>),
}

type RunnerFn = Box<dyn FnOnce(&mut App) -> AppExit>;

/// The application object, containing a global repository of program state and the application
/// runner, the main loop of the program.  Typically no more than one application runner is
/// instantiated in the lifetime of a process.  On some platforms and configurations, such as
/// graphical apps on iOS or web, this is a hard requirement and the main loop of the application
/// runner will never return.
#[readonly::make]
pub struct App {
    /// The application name, as passed to [`App::new`] or [`App::set_name`], and used in user
    /// interface or logging/diagnostic text.  Read-only via the [`readonly`] crate; use
    /// [`App::set_name`] to change.
    #[readonly]
    pub name: String,
    /// The application runner, a closure run by [`App::run`] that processes the main loop of the
    /// application.  This is set by [`App::set_runner`], and defaults to [`run_once`].
    runner: RunnerFn,
    /// A set of plugin IDs that have already been registered with the application.  This set is
    /// checked on each call to [`App::add_plugin`] to ensure that a plugin is not added to the same
    /// application instance more than once, unless [`Plugin::is_unique`] returns `false`.
    plugins: HashSet<std::any::TypeId>,
}

/// The default application runner, which features no event loop.  This is useful for simple
/// headless applications that do not require a main loop, such as command-line utilities, and is
/// the default behavior of a newly initialized [`App`].
pub fn run_once(app: &mut App) -> AppExit {
    let _ = app;
    AppExit::Success
}

/// Associated functions for initializing and manipulating [`App`] instances.  You should use
/// [`App::new`] to initialize a new [`App`] instance, unless you really know what you are doing.
/// The closure to use to execute the main loop of the application can be configured with
/// [`App::set_runner`], and plugins can be added with [`App::add_plugin`].  Once an [`App`] is
/// fully configured, enter the main loop with [`App::run`].
///
/// ```
/// # use atomcad_app::prelude::*;
/// App::new("My App".into())
///     .set_runner(|app: &mut App| {
///         # let _ = app;
///         // Your application logic here.
///         AppExit::Success
///     })
///     .run();
/// ```
impl App {
    /// Creates a new application runner with the given name.  The name is used to identify the
    /// application in log messages and other diagnostic output, as well as user interface elements
    /// in window managers (default window title, application menu name on macOS, etc.).  The runner
    /// is initialized to [`run_once`], but can be changed with [`App::set_runner`].  The name must
    /// be specified, but can later be changed with [`App::set_name`].
    pub fn new(name: String) -> Self {
        Self {
            name,
            runner: Box::new(run_once),
            plugins: HashSet::new(),
        }
    }

    /// Change the name of the application runner, during configuration or at runtime.  To read the
    /// current application name, do so directly via the [`App::name`] field.
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    /// Change the runner / event loop function.  Runners are expected to be called only once, and
    /// are permitted to never return (although it is generally preferable to do so on platforms
    /// where that is a possibility).
    pub fn set_runner(&mut self, runner: impl FnOnce(&mut App) -> AppExit + 'static) -> &mut Self {
        self.runner = Box::new(runner);
        self
    }

    /// Register a plugin with the application.  Plugins are used to configure the application and
    /// provide additional functionality to maintain global state or service the application event
    /// loop.  A given type of plugin can only be added to the same application instance once,
    /// unless [`Plugin::is_unique`] returns `false`.
    ///
    /// # Panics
    ///
    /// * As must envisioned use cases involve separate plugin types for each configuration or
    ///   feature, adding two plugins of the same type to the same application instance is generally
    ///   disallowed, and will generally result in a panic.  See [`Plugin::is_unique`] for details.
    pub fn add_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
        // Panic if the plugin is unique and has already been added to the application.
        let id = plugin.id();
        if plugin.is_unique() && self.plugins.contains(&id) {
            panic!("Attempted to add a non-unique plugin to the same App instance twice");
        }
        self.plugins.insert(id);

        // Call the plugin's build method, which configures the application.
        plugin.build(self);

        self
    }

    /// Run the application's event processing loop by calling its [runner](App::set_runner).  This
    /// method consumes the application runner, which is reset to [`run_once`] unless it is further
    /// set from within the application runner itself.  On some platforms This *must* be called from
    /// the main thread of the application.
    ///
    /// # Caveats
    ///
    /// * Calls to [`App::run()`] will never return on iOS and Web, unless the headless [`run_once`]
    ///   runner is used, as running the user input event loop on these platforms requires giving up
    ///   control of the main thread.
    ///
    /// * Headless apps can generally expect this method to return control to the caller upon
    ///   completion.  Those that do not require interfacing with the operating system / window
    ///   manager's event loop may or may not return, and even if they do return, on some platforms
    ///   it is not possible to re-initialize the windowing event loop.
    ///
    /// # Panics
    ///
    /// * Panics if not called from the main thread on platforms where this is a requirement.
    pub fn run(&mut self) -> AppExit {
        // This is a bit of a hack to get around the borrow checker.  Calling the runner directly
        // from self.runner will consume the runner while mutably borrowing the application because
        // the runner is of type `FnOnce(&mut App)`.  But the app contains the runner!  The borrow
        // checker complains that the application is already moved when the runner is called, as the
        // compiler needs to consume app twice: once to extract and use the runner, and again as an
        // argument to the runner.  This obviously won't work.
        //
        // Rather than attempt some unsafe{} black magic, we swap out the runner, whatever it was,
        // for another boxed runner value.  We could then call the extracted runner, enabling the
        // App value to be returned to the calling context, just without its original runner.
        let runner = std::mem::replace(&mut self.runner, Box::new(run_once));

        // Returns an AppExit value from the runner.
        (runner)(self)
    }
}

// End of File
