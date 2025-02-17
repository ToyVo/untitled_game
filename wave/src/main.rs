use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::FRAC_PI_2;

const DIM: usize = 16;
const TILE_SIZE: f32 = 50.;

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
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_tiles)
        .add_systems(Update, analyze_tiles)
        .run();
}

fn spawn_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tiles: ResMut<TileConfig>,
) {
    commands.spawn(Camera2d);

    let mut init_tiles = vec![];
    let up = Tile::new(
        asset_server.load("up.png"),
        vec![
            String::from("wbw"),
            String::from("wbw"),
            String::from("www"),
            String::from("wbw"),
        ],
    );
    for i in 0..4 {
        init_tiles.push(up.rotate(i));
    }
    let init_tiles_ref = &init_tiles.clone();
    for tile in &mut init_tiles {
        tile.analyze(init_tiles_ref)
    }
    tiles.tiles = init_tiles;

    let offset = DIM as f32 * TILE_SIZE / 2.;
    for x in 0..DIM {
        for y in 0..DIM {
            commands.spawn((
                Sprite::from_image(asset_server.load("blank.png")),
                Transform::from_xyz(
                    x as f32 * TILE_SIZE - offset,
                    y as f32 * TILE_SIZE - offset,
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
    mut query: Query<(&mut Sprite, &mut Transform, &mut Cell, Entity), Without<Collapsed>>,
    mut collapsed: Query<(&mut Sprite, &mut Transform, &mut Cell, Entity), With<Collapsed>>,
    mut commands: Commands,
    tiles: Res<TileConfig>,
    asset_server: Res<AssetServer>,
) {
    if query.is_empty() {
        return;
    }

    let mut sorted = query
        .iter()
        .map(|(_, _, cell, entity)| (entity, cell.options.len(), cell.coord))
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

    let (mut sprite, mut transform, mut cell, entity) = query.get_mut(entity).unwrap();

    if cell.options.is_empty() {
        let blank = asset_server.load("blank.png");
        for (_, _, mut cell, _) in &mut query {
            cell.options = (0..tiles.tiles.len()).collect();
        }
        for (mut sprite, mut transform, mut cell, entity) in &mut collapsed {
            cell.options = (0..tiles.tiles.len()).collect();
            commands.entity(entity).remove::<Collapsed>();
            transform.rotation = Quat::from_rotation_z(0.);
            sprite.image = blank.clone();
        }
        return;
    }

    let tile_index = cell.options[thread_rng().gen_range(0..cell.options.len())];
    let tile = &tiles.tiles[tile_index];

    // Mark as collapsed
    cell.options = vec![tile_index];
    sprite.image = tile.image.clone();
    transform.rotate_local_z(-FRAC_PI_2 * tile.rotation as f32);
    commands.entity(entity).insert(Collapsed);

    // check neighboring cells
    let [up, right, down, left] = get_neighbors(coords.0, coords.1, DIM - 1, DIM - 1);
    for (_, _, mut cell, _) in &mut query {
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
