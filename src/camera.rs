mod input;

use bevy::prelude::*;


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_camera,
        ));
        app.add_systems(Update, (
            input::orbit,
            input::zoom.after(input::orbit),
        ));
    }
}


#[derive(Component)]
struct MainCamera;

fn setup_camera(
    mut cmd: Commands
) {
    cmd.spawn((
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    cmd.spawn(DirectionalLight::default());
}
