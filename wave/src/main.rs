use bevy::prelude::*;
use bevy::window::WindowResolution;
use rand::prelude::*;
use std::f32::consts::FRAC_PI_2;

use wave::*;

const DIM: usize = 30;
const TILE_SIZE: f32 = 56.;

#[derive(Component)]
struct Collapsed;

#[derive(Component)]
struct Cell {
    pub options: Vec<usize>,
    pub coord: SnappedCoordinate,
    pub collapsed: bool,
}

#[derive(Resource)]
struct TileConfig {
    pub tiles: Vec<Tile>,
}

fn main() {
    App::new()
        .insert_resource(TileConfig { tiles: Vec::new() })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(TILE_SIZE * DIM as f32 / 5., TILE_SIZE * DIM as f32 / 5.),
                // fill the entire browser window
                fit_canvas_to_parent: true,
                // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (preload_tiles, spawn_cells).chain())
        .add_systems(Update, analyze_tiles)
        .run();
}

fn preload_tiles(asset_server: Res<AssetServer>, mut tiles: ResMut<TileConfig>) {
    let mut init_tiles = vec![];

    // Initialize tiles with images and edges
    init_tiles.push(Tile::new(
        asset_server.load("circuit/0.png"),
        vec![
            String::from("AAA"),
            String::from("AAA"),
            String::from("AAA"),
            String::from("AAA"),
        ],
    ));
    init_tiles.push(Tile::new(
        asset_server.load("circuit/1.png"),
        vec![
            String::from("BBB"),
            String::from("BBB"),
            String::from("BBB"),
            String::from("BBB"),
        ],
    ));
    let tile = Tile::new(
        asset_server.load("circuit/2.png"),
        vec![
            String::from("BBB"),
            String::from("BCB"),
            String::from("BBB"),
            String::from("BBB"),
        ],
    );
    for i in 1..4 {
        init_tiles.push(tile.rotate(i));
    }
    init_tiles.push(tile);
    let tile = Tile::new(
        asset_server.load("circuit/3.png"),
        vec![
            String::from("BBB"),
            String::from("BDB"),
            String::from("BBB"),
            String::from("BDB"),
        ],
    );
    init_tiles.push(tile.rotate(1));
    init_tiles.push(tile);
    let tile = Tile::new(
        asset_server.load("circuit/4.png"),
        vec![
            String::from("ABB"),
            String::from("BCB"),
            String::from("BBA"),
            String::from("AAA"),
        ],
    );
    for i in 1..4 {
        init_tiles.push(tile.rotate(i));
    }
    init_tiles.push(tile);
    let tile = Tile::new(
        asset_server.load("circuit/5.png"),
        vec![
            String::from("ABB"),
            String::from("BBB"),
            String::from("BBB"),
            String::from("BBA"),
        ],
    );
    for i in 1..4 {
        init_tiles.push(tile.rotate(i));
    }
    init_tiles.push(tile);
    let tile = Tile::new(
        asset_server.load("circuit/6.png"),
        vec![
            String::from("BBB"),
            String::from("BCB"),
            String::from("BBB"),
            String::from("BCB"),
        ],
    );
    init_tiles.push(tile.rotate(1));
    init_tiles.push(tile);
    let tile = Tile::new(
        asset_server.load("circuit/7.png"),
        vec![
            String::from("BDB"),
            String::from("BCB"),
            String::from("BDB"),
            String::from("BCB"),
        ],
    );
    init_tiles.push(tile.rotate(1));
    init_tiles.push(tile);
    let tile = Tile::new(
        asset_server.load("circuit/8.png"),
        vec![
            String::from("BDB"),
            String::from("BBB"),
            String::from("BCB"),
            String::from("BBB"),
        ],
    );
    for i in 1..4 {
        init_tiles.push(tile.rotate(i));
    }
    init_tiles.push(tile);
    let tile = Tile::new(
        asset_server.load("circuit/9.png"),
        vec![
            String::from("BCB"),
            String::from("BCB"),
            String::from("BBB"),
            String::from("BCB"),
        ],
    );
    for i in 1..4 {
        init_tiles.push(tile.rotate(i));
    }
    init_tiles.push(tile);
    let tile = Tile::new(
        asset_server.load("circuit/10.png"),
        vec![
            String::from("BCB"),
            String::from("BCB"),
            String::from("BCB"),
            String::from("BCB"),
        ],
    );
    init_tiles.push(tile.rotate(1));
    init_tiles.push(tile);
    let tile = Tile::new(
        asset_server.load("circuit/11.png"),
        vec![
            String::from("BCB"),
            String::from("BCB"),
            String::from("BBB"),
            String::from("BBB"),
        ],
    );
    for i in 1..4 {
        init_tiles.push(tile.rotate(i));
    }
    init_tiles.push(tile);
    let tile = Tile::new(
        asset_server.load("circuit/12.png"),
        vec![
            String::from("BBB"),
            String::from("BCB"),
            String::from("BBB"),
            String::from("BCB"),
        ],
    );
    init_tiles.push(tile.rotate(1));
    init_tiles.push(tile);

    // TODO: deduplicate based on edges
    let init_tiles_ref = &init_tiles.clone();
    for tile in &mut init_tiles {
        tile.generate_relationships(init_tiles_ref)
    }
    tiles.tiles = init_tiles;
}

fn spawn_cells(mut commands: Commands, tiles: Res<TileConfig>, window: Single<&Window>) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 5.;
    commands.spawn((
        Camera2d,
        projection,
        Transform::from_xyz(
            window.resolution.height() / 2. * 5.,
            window.resolution.height() / 2. * 5.,
            10.,
        ),
    ));

    for x in 0..DIM {
        for y in 0..DIM {
            commands.spawn((
                Transform::from_xyz(
                    x as f32 * TILE_SIZE + TILE_SIZE / 2.,
                    y as f32 * TILE_SIZE + TILE_SIZE / 2.,
                    0.,
                ),
                Cell {
                    options: (0..tiles.tiles.len()).collect(),
                    coord: SnappedCoordinate { x, y },
                    collapsed: false,
                },
            ));
        }
    }
}

fn analyze_tiles(
    mut query: Query<(&mut Transform, &mut Cell, Entity), Without<Collapsed>>,
    mut collapsed: Query<(&mut Transform, &mut Cell, Entity), With<Collapsed>>,
    mut commands: Commands,
    tiles: Res<TileConfig>,
) {
    if query.is_empty() {
        return;
    }

    let mut sorted = query
        .iter()
        .map(|(_, cell, entity)| (entity, cell.options.len(), cell.coord))
        .collect::<Vec<(Entity, usize, SnappedCoordinate)>>();
    sorted.sort_by(|a, b| a.1.cmp(&b.1));
    let end_index = if let Some((i, _)) = sorted
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, (_, options_len, _))| *options_len > sorted[0].1)
    {
        i
    } else {
        sorted.len()
    };
    let chosen_index = thread_rng().gen_range(0..end_index);
    let (entity, _, coords) = sorted[chosen_index];

    let (mut transform, mut cell, entity) = query.get_mut(entity).unwrap();

    if cell.options.is_empty() {
        for (_, mut cell, _) in &mut query {
            cell.options = (0..tiles.tiles.len()).collect();
        }
        for (mut transform, mut cell, entity) in &mut collapsed {
            cell.options = (0..tiles.tiles.len()).collect();
            cell.collapsed = false;
            commands
                .entity(entity)
                .remove::<Collapsed>()
                .remove::<Sprite>();
            transform.rotation = Quat::from_rotation_z(0.);
        }
        return;
    }

    let tile_index = cell.options[thread_rng().gen_range(0..cell.options.len())];
    let tile = &tiles.tiles[tile_index];

    // Mark as collapsed
    cell.options = vec![tile_index];
    cell.collapsed = true;
    transform.rotate_local_z(-FRAC_PI_2 * tile.rotation as f32);
    commands
        .entity(entity)
        .insert((Collapsed, Sprite::from_image(tile.image.clone())));

    // check neighboring cells
    let neighbors = get_neighbors_no_wrap(coords.x, coords.y, DIM - 1, DIM - 1);
    for (_, mut cell, _) in &mut query {
        let cb = |option| {
            if cell.options.contains(option) {
                Some(*option)
            } else {
                None
            }
        };
        if Some(cell.coord) == neighbors.up {
            cell.options = tile.up.iter().filter_map(cb).collect();
        } else if Some(cell.coord) == neighbors.right {
            cell.options = tile.right.iter().filter_map(cb).collect();
        } else if Some(cell.coord) == neighbors.down {
            cell.options = tile.down.iter().filter_map(cb).collect();
        } else if Some(cell.coord) == neighbors.left {
            cell.options = tile.left.iter().filter_map(cb).collect();
        }
    }
}
