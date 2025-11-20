use bevy::{
    prelude::*,
    window::{
        PrimaryWindow,
        CursorOptions,
        CursorGrabMode,
    },
    input::common_conditions::input_just_released,
};

use crate::{
    events::{
        WindowGrabEvent,
        DisplayMenu,
    },
    resources::GameState,
};


pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        // --- cursor grab ---
        app.add_systems(Update, (
            toggle_grab.run_if(input_just_released(KeyCode::Escape)),
        ));
        app.add_observer(apply_grab);

        // --- game state ---
        app.add_systems(Update, (
            toggle_state.run_if(input_just_released(KeyCode::Escape)),
        ));
    }
}


// --- tmp ---
// Set up a simple 3D scene
fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn((
        Name::new("Plane"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            // Turning off culling keeps the plane visible when viewed from beneath.
            cull_mode: None,
            ..default()
        })),
    ));

    cmd.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(1.5, 0.51, 1.5),
    ));

    cmd.spawn((
        Name::new("Light"),
        PointLight::default(),
        Transform::from_xyz(3.0, 8.0, 5.0),
    ));
}


// --- cursor grab ---
fn apply_grab(
    grab: On<WindowGrabEvent>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if **grab {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    } else {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}

fn toggle_grab(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut cmd: Commands,
) {
    window.focused = !window.focused;
    cmd.trigger(WindowGrabEvent(window.focused));
}


// --- game state ---
fn toggle_state(
    window: Single<&mut Window, With<PrimaryWindow>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut cmd: Commands,
) {
    if window.focused {
        next_state.set(GameState::Running);
    } else {
        next_state.set(GameState::Paused);
    }

    cmd.trigger(DisplayMenu(window.focused));
}
