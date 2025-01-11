mod crs;
pub use crs::*;
pub mod menu;
pub mod cameras;
pub mod splash;
pub mod game;

use bevy::prelude::*;

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn not_in_state<S: States>(state: S) -> impl FnMut(Option<Res<State<S>>>) -> bool + Clone {
    move |current_state: Option<Res<State<S>>>| match current_state {
        Some(current_state) => *current_state != state,
        None => false,
    }
}
