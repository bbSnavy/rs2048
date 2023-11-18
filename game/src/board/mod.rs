use raylib::drawing::RaylibDrawHandle;

use crate::vector;

pub struct Board {
    size:  i32,
    tiles: Vec<Tile>,
    tile_size: i32,
}

impl Board {
    pub fn new() -> Board {
        let size = 4;
        let mut tiles = Vec::new();

        for x in 0..size {
            for y in 0..size {
                let tile = Tile::new(
                    x, y,
                );

                tiles.push(tile);
            }
        }

        Board {
            size,
            tiles,
            tile_size: 64,
        }
    }

    pub fn tile_get(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles.get((y * self.size + x) as usize)
    }

    pub fn render(&self, draw: &RaylibDrawHandle) {
        let tile_base = vector::Vector2::new(32, 32);

        for tile_y in 0..self.size {
            for tile_x in 0..self.size {
                let tile_pos = tile_base.copy().add(vector::Vector2::new(
                    tile_x * self.tile_size, tile_y * self.tile_size));

                match self.tile_get(tile_x, tile_y) {
                    None => {}
                    Some(tile) => {

                    }
                }
            }
        }
    }
}

pub struct Tile {
    x:     i32,
    y:     i32,
    value: i64,
}

impl Tile {
    pub fn new(x: i32, y: i32) -> Tile {
        Tile {
            x,
            y,
            value: 0,
        }
    }

    pub fn pos_x(&self) -> vector::Vector2<i32> {
        vector::Vector2::new(self.x, self.y)
    }

    pub fn value(&self) -> i64 { self.value }

    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }
}
