// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlatformDetails;

impl Plugin for PlatformDetails {
    fn build(&self, app: &mut App) {
        app.add_system(update_canvas_size);
    }
}

fn update_canvas_size(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    (|| {
        let mut window = window.get_single_mut().ok()?;
        let browser_window = web_sys::window()?;
        let width = browser_window.inner_width().ok()?.as_f64()?;
        let height = browser_window.inner_height().ok()?.as_f64()?;
        window.resolution.set(width as f32, height as f32);
        Some(())
    })();
}

// End of File
