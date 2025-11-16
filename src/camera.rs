mod input;
mod grab;

use bevy::{
    prelude::*,
    input::common_conditions::input_just_released,
};


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_camera,
        ));
        app.add_systems(Update, (
            input::orbit,
            input::zoom.after(input::orbit),
            grab::focus_events,
            grab::toggle_grab.run_if(input_just_released(KeyCode::Escape)),
        ));
        app.add_observer(grab::apply_grab);
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

// tmp
// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Plane"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            // Turning off culling keeps the plane visible when viewed from beneath.
            cull_mode: None,
            ..default()
        })),
    ));

    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(1.5, 0.51, 1.5),
    ));

    commands.spawn((
        Name::new("Light"),
        PointLight::default(),
        Transform::from_xyz(3.0, 8.0, 5.0),
    ));
}
