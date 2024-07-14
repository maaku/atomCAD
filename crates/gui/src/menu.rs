// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at <http://mozilla.org/MPL/2.0/>.

use crate::platform::menubar as platform_menubar;
use winit::event_loop::EventLoopBuilder;
use winit::window::Window;

pub use crate::platform::menubar::PlatformMenubar;

/// A menubar is a hierarchical list of actions with attached titles and/or keyboard shortcuts.  It
/// is attached to either the application instance (macOS), the main window (Windows/Linux), or
/// fully emulated (mobile/web).  On platforms that lack per-window menubars, the application must
/// switch the global menubar based on the active window.
///
/// Menus can also be contextual (e.g. a popup right-click menu) or accessed from the system tray.
pub struct Blueprint {
    pub title: String,
    pub items: Vec<Item>,
}

impl Blueprint {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
            items: Vec::new(),
        }
    }
}

/// A menu item is either an action (with an optional keyboard shortcut) or a submenu.  The
/// Separator is a visual divider between groups of related menu items.
pub enum Item {
    Separator,
    Entry {
        title: String,
        shortcut: Shortcut,
        action: Action,
    },
    SubMenu(Blueprint),
}

/// A keyboard shortcut is a combination of modifier keys (e.g. Shift, Option, Alt, etc.) and the
/// key to press (indicated by a unicode character).  Some shortcuts for common actions like copy,
/// paste, quit, etc. are system-wide and cannot be overridden by the application.
#[derive(Clone, Copy)]
pub enum Shortcut {
    None,
    System(SystemShortcut),
}

/// Common actions like copy-paste, file-open, and quit are usually bound to shortcuts that vary
/// from platform to platform, but are expected to remain consistent across all apps on that
/// platform.
#[derive(Clone, Copy)]
pub enum SystemShortcut {
    Preferences,
    HideApp,
    HideOthers,
    QuitApp,
}

/// A menu action is a callback that is invoked when the menu item is selected.  It can be either an
/// internal, application-defined action, or a system response implemented by the operating system.
pub enum Action {
    System(SystemAction),
}

/// System actions are predefined actions that are implemented by the operating system.  They are
/// usually used for common actions like showing the preferences window, hiding the app, etc.
pub enum SystemAction {
    LaunchAboutWindow,
    LaunchPreferences,
    ServicesMenu,
    HideApp,
    HideOthers,
    ShowAll,
    Terminate,
}

/// The platform-specific setup function is called during application initialization to configure
/// the event loop with the necessary platform-specific menu handling code, and returns a handle to
/// a platform-specific datastructure which contains the necessary information to create and attach
/// menus to windows.
pub fn platform_setup<T: Send>(event_loop_builder: &mut EventLoopBuilder<T>) -> PlatformMenubar {
    platform_menubar::configure_event_loop(event_loop_builder)
}

/// Attach the menubar to the window.  This function is called when the window is created and
/// attached to the event loop.  It is responsible for creating the platform-specific menu objects
/// from the blueprint spec, and attaching them to the window.
pub fn attach_menubar_to_window(
    window: &Window,
    platform_menubar: &PlatformMenubar,
    blueprint: &Blueprint,
) {
    // Do the platform-dependent work of constructing the menubar and
    // attaching it to the application object or main window.
    platform_menubar::attach_to_window(window, platform_menubar, blueprint);
}

// End of File
