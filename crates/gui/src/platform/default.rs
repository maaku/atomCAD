// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at <http://mozilla.org/MPL/2.0/>.

pub mod menubar {
    use crate::menu;
    use winit::{event_loop::EventLoopBuilder, window::Window};

    // Currently does nothing, and is present merely to ensure we compile on platforms, including
    // those that don't natively support any menubar functionality.

    // Platform-specific type that handles all menu allocations.
    #[derive(Default)]
    pub struct PlatformMenubar;

    pub fn configure_event_loop<T: 'static>(
        event_loop_builder: &mut EventLoopBuilder<T>,
    ) -> PlatformMenubar {
        let _ = event_loop_builder;
        PlatformMenubar
    }

    pub fn attach_to_window(
        // On some platforms, e.g. Windows and Linux, the menu bar is part of the window itself, and
        // we need to attach a copy of the menu to each individual window.
        window: &Window,
        // On platforms like macos, there is one menubar for the entire application (indeed, the
        // entire system).  Changing focus results in the menubar blueprint being swapped out.
        platform_menubar: &PlatformMenubar,
        // The layout of the menubar to be used when this window is in focus.
        blueprint: &menu::Blueprint,
    ) {
        let _ = (window, platform_menubar, blueprint);
    }
}

// End of File
