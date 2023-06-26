// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use bevy::{
    prelude::*,
    window::{PresentMode, WindowPlugin},
    winit::WinitSettings,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};
use bevy_mod_picking::prelude::*;

use atomcad::camera::{pan_orbit_camera, PanOrbitCamera};
use atomcad::menubar::winit_menu_bar;
use atomcad::molecule_builder::{init_molecule, molecule_builder};
use atomcad::APP_NAME;

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: APP_NAME.into(),
                // Turn off vsync to maximize CPU/GPU usage
                present_mode: PresentMode::AutoNoVsync,
                // See index.html if this changes
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(InfiniteGridPlugin)
        .add_startup_system(winit_menu_bar)
        .add_startup_system(setup)
        .add_startup_system(init_molecule)
        .add_system(ui_hello_world)
        .add_system(pan_orbit_camera)
        .add_system(molecule_builder)
        .run();
}

fn setup(mut commands: Commands) {
    // camera
    let position = Vec3::new(0.0, 1.5, 6.0);
    let target = Vec3::ZERO;

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(position).looking_at(target, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            radius: (position - target).length(),
            ..Default::default()
        },
        RaycastPickCamera::default(),
    ));

    // infinite grid
    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    // light source
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, 4.0),
        ..default()
    });
}

fn ui_hello_world(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("Hello World!");
    });
}

// End of File
