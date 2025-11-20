use bevy::prelude::*;


#[derive(Event, Deref)]
pub struct WindowGrabEvent(pub bool);

#[derive(Event, Deref)]
pub struct DisplayMenu(pub bool);
