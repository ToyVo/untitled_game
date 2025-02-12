use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::prelude::*;
use std::f32::consts::FRAC_PI_2;
use std::time::Duration;

const DIM: usize = 5;
const TILE_SIZE: f32 = 50.;

#[derive(Component)]
struct Collapsed;

struct Tile {
    pub image: Handle<Image>,
    /// 0-3 scaled by 90 degrees
    pub rotation: usize,
}

#[derive(Component)]
struct Cell {
    pub options: Vec<usize>,
}

#[derive(Resource)]
struct TileConfig {
    pub tiles: Vec<Tile>,
}

fn main() {
    App::new()
        .insert_resource(TileConfig { tiles: Vec::new() })
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            spawn_tiles.run_if(on_timer(Duration::from_secs(1))),
        )
        .add_systems(Update, analyze_tiles)
        .run();
}

fn spawn_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tiles: ResMut<TileConfig>,
) {
    commands.spawn(Camera2d);

    tiles.tiles = (0..4)
        .map(|i| Tile {
            image: asset_server.load("up.png"),
            rotation: i,
        })
        .collect();

    let offset = DIM as f32 * TILE_SIZE / 2.;
    for i in 0..DIM * DIM {
        commands.spawn((
            Sprite::from_image(asset_server.load("blank.png")),
            Transform::from_xyz(
                (i % DIM) as f32 * TILE_SIZE - offset,
                (i / DIM) as f32 * TILE_SIZE - offset,
                0.,
            ),
            Cell {
                options: (0..tiles.tiles.len()).collect(),
            },
        ));
    }
}

fn analyze_tiles(
    mut query: Query<(&mut Sprite, &mut Transform, Entity), (With<Cell>, Without<Collapsed>)>,
    collapsed: Query<&Collapsed, With<Cell>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if collapsed.is_empty() {
        if let Some((mut sprite, mut transform, entity)) =
            query.iter_mut().choose(&mut thread_rng())
        {
            sprite.image = asset_server.load("up.png");
            let index = thread_rng().gen_range(0..4);
            transform.rotate_local_z(FRAC_PI_2 * index as f32);
            commands.entity(entity).insert(Collapsed);
        }
    } else {
        // TODO: find entity with lowest entropy
        for (mut sprite, mut transform, entity) in &mut query {}
    }
}
