mod events;
mod general;
mod resources;
mod camera;
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

