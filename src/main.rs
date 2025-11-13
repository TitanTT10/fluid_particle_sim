mod events;
mod resources;
mod camera;
mod simulation;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            resources::ResourcesPlugin,
            camera::CameraPlugin,
//            simulation::SimPlugin,
        ))
        .run();
}

