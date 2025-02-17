use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::FRAC_PI_2;
use bevy::window::WindowResolution;

const DIM: usize = 16;
const TILE_SIZE: f32 = 56.;

#[derive(Component)]
struct Collapsed;

#[derive(Clone)]
struct Tile {
    pub image: Handle<Image>,
    /// 0-3 scaled by 90 degrees
    pub rotation: usize,
    pub edges: Vec<String>,
    /// represents valid indices into tiles array
    pub up: Vec<usize>,
    pub left: Vec<usize>,
    pub down: Vec<usize>,
    pub right: Vec<usize>,
}

impl Tile {
    pub fn new(image: Handle<Image>, edges: Vec<String>) -> Self {
        Self {
            image,
            rotation: 0,
            edges,
            up: Vec::new(),
            right: Vec::new(),
            down: Vec::new(),
            left: Vec::new(),
        }
    }

    pub fn rotate(&self, n: usize) -> Self {
        let mut edges = self.edges.clone();
        edges.rotate_right(n);
        Self {
            image: self.image.clone(),
            rotation: self.rotation + n,
            edges,
            up: Vec::new(),
            right: Vec::new(),
            down: Vec::new(),
            left: Vec::new(),
        }
    }

    pub fn analyze(&mut self, tiles: &[Tile]) {
        for (i, tile) in tiles.iter().enumerate() {
            // Check if the current tile's bottom edge matches this tile's top edge
            if tile.edges[2].chars().rev().collect::<String>() == self.edges[0] {
                self.up.push(i);
            }
            // Check if the current tile's left edge matches this tile's right edge
            if tile.edges[3].chars().rev().collect::<String>() == self.edges[1] {
                self.right.push(i);
            }
            // Check if the current tile's top edge matches this tile's bottom edge
            if tile.edges[0].chars().rev().collect::<String>() == self.edges[2] {
                self.down.push(i);
            }
            // Check if the current tile's right edge matches this tile's left edge
            if tile.edges[1].chars().rev().collect::<String>() == self.edges[3] {
                self.left.push(i);
            }
        }
    }
}

#[derive(Component)]
struct Cell {
    pub options: Vec<usize>,
    pub coord: (usize, usize),
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
                resolution: WindowResolution::new(TILE_SIZE * DIM as f32, TILE_SIZE * DIM as f32),
                // fill the entire browser window
                fit_canvas_to_parent: true,
                // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, spawn_tiles)
        .add_systems(Update, analyze_tiles)
        .run();
}

fn spawn_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tiles: ResMut<TileConfig>,
    window: Single<&Window>,
) {
    commands.spawn((Camera2d, Transform::from_xyz(window.resolution.height() / 2., window.resolution.height() / 2., 10.)));
    
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
        tile.analyze(init_tiles_ref)
    }
    tiles.tiles = init_tiles;

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
                    coord: (x, y),
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
        .collect::<Vec<(Entity, usize, (usize, usize))>>();
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
            commands.entity(entity).remove::<Collapsed>().remove::<Sprite>();
            transform.rotation = Quat::from_rotation_z(0.);
        }
        return;
    }

    let tile_index = cell.options[thread_rng().gen_range(0..cell.options.len())];
    let tile = &tiles.tiles[tile_index];

    // Mark as collapsed
    cell.options = vec![tile_index];
    transform.rotate_local_z(-FRAC_PI_2 * tile.rotation as f32);
    commands.entity(entity).insert((
        Collapsed,
        Sprite::from_image(tile.image.clone()),
    ));

    // check neighboring cells
    let [up, right, down, left] = get_neighbors(coords.0, coords.1, DIM - 1, DIM - 1);
    for (_, mut cell, _) in &mut query {
        if cell.coord == up {
            cell.options = tile
                .up
                .iter()
                .filter_map(|opt| {
                    if cell.options.contains(opt) {
                        Some(*opt)
                    } else {
                        None
                    }
                })
                .collect();
        } else if cell.coord == right {
            cell.options = tile
                .right
                .iter()
                .filter_map(|opt| {
                    if cell.options.contains(opt) {
                        Some(*opt)
                    } else {
                        None
                    }
                })
                .collect();
        } else if cell.coord == down {
            cell.options = tile
                .down
                .iter()
                .filter_map(|opt| {
                    if cell.options.contains(opt) {
                        Some(*opt)
                    } else {
                        None
                    }
                })
                .collect();
        } else if cell.coord == left {
            cell.options = tile
                .left
                .iter()
                .filter_map(|opt| {
                    if cell.options.contains(opt) {
                        Some(*opt)
                    } else {
                        None
                    }
                })
                .collect();
        }
    }
}

fn get_neighbors(x: usize, y: usize, x_max: usize, y_max: usize) -> [(usize, usize); 4] {
    let x_sub = if x == 0 { x_max } else { x - 1 };
    let y_sub = if y == 0 { y_max } else { y - 1 };
    let x_add = if x == x_max { 0 } else { x + 1 };
    let y_add = if y == y_max { 0 } else { y + 1 };
    [(x, y_add), (x_add, y), (x, y_sub), (x_sub, y)]
}
