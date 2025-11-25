use bevy::prelude::*;


#[derive(Event, Deref)]
pub struct WindowGrabEvent(pub bool);
