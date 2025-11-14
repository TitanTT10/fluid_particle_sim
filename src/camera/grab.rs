use bevy::{
    prelude::*,
    window::{
        PrimaryWindow,
        CursorOptions,
        WindowFocused
    }
};

use crate::events::WindowGrabEvent;

pub fn apply_grab(
    grab: On<WindowGrabEvent>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    use bevy::window::CursorGrabMode;
    if **grab {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    } else {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}

pub fn focus_events(
    mut events: MessageReader<WindowFocused>,
    mut commands: Commands,
) {
    if let Some(event) = events.read().last() {
        commands.trigger(WindowGrabEvent(event.focused));
    }
}

pub fn toggle_grab(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    window.focused = !window.focused;
    commands.trigger(WindowGrabEvent(window.focused));
}

