use bevy::prelude::{Handle, Image};

#[derive(PartialEq, Copy, Clone)]
pub struct SnappedCoordinate {
    pub x: usize,
    pub y: usize,
}

pub struct CardinalDirections {
    pub up: Option<SnappedCoordinate>,
    pub right: Option<SnappedCoordinate>,
    pub down: Option<SnappedCoordinate>,
    pub left: Option<SnappedCoordinate>,
}

#[derive(Clone)]
pub struct Tile {
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

    pub fn generate_relationships(&mut self, tiles: &[Tile]) {
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

pub fn get_neighbors_wrap_xy(x: usize, y: usize, x_max: usize, y_max: usize) -> CardinalDirections {
    let x_sub = if x == 0 { x_max } else { x - 1 };
    let y_sub = if y == 0 { y_max } else { y - 1 };
    let x_add = if x == x_max { 0 } else { x + 1 };
    let y_add = if y == y_max { 0 } else { y + 1 };
    CardinalDirections {
        up: Some(SnappedCoordinate { x, y: y_add }),
        right: Some(SnappedCoordinate { x: x_add, y }),
        down: Some(SnappedCoordinate { x, y: y_sub }),
        left: Some(SnappedCoordinate { x: x_sub, y }),
    }
}

pub fn get_neighbors_wrap_x(x: usize, y: usize, x_max: usize, y_max: usize) -> CardinalDirections {
    let mut directions = get_neighbors_wrap_xy(x, y, x_max, y_max);
    if y == 0 {
        directions.down = None;
    } else if y == y_max {
        directions.up = None;
    }
    directions
}

pub fn get_neighbors_wrap_y(x: usize, y: usize, x_max: usize, y_max: usize) -> CardinalDirections {
    let mut directions = get_neighbors_wrap_xy(x, y, x_max, y_max);
    if x == 0 {
        directions.left = None;
    } else if x == x_max {
        directions.right = None;
    }
    directions
}

pub fn get_neighbors_no_wrap(x: usize, y: usize, x_max: usize, y_max: usize) -> CardinalDirections {
    let mut directions = get_neighbors_wrap_xy(x, y, x_max, y_max);
    if x == 0 {
        directions.left = None;
    } else if x == x_max {
        directions.right = None;
    }
    if y == 0 {
        directions.down = None;
    } else if y == y_max {
        directions.up = None;
    }
    directions
}
