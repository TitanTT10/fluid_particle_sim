mod events;
mod resources;
mod camera;
mod simulation;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            camera::CameraPlugin,
            simulation::SimPlugin,
        ))
        .run();
}

