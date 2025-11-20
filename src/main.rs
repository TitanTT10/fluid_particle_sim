mod events;
mod resources;
mod general;
mod camera;
mod ui;
mod simulation;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            general::GeneralPlugin,
            resources::ResourcesPlugin,
            camera::CameraPlugin,
            simulation::SimPlugin,
        ))
        .run();
}

