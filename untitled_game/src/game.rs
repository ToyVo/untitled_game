use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use crate::{despawn_screen, GameState, Player};

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Name::new("Plane"),
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        OnGameScreen,
    ));
    // cube
    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        OnGameScreen,
    ));
    // Player
    commands.spawn((
        Name::new("Player"),
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 144, 124))),
        Transform::from_xyz(0.0, 0.3, 3.0),
        Player,
        OnGameScreen,
    ));
    // light
    commands.spawn((
        Name::new("Light"),
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        OnGameScreen,
    ));
}
