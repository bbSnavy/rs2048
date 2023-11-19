use std::cell::RefCell;
use raylib::prelude::*;

use crate::vector;

pub struct Board {
    size:  i32,
    tiles: Vec<RefCell<Tile>>,
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

                tiles.push(RefCell::new(tile));
            }
        }

        Board {
            size,
            tiles,
            tile_size: 64,
        }
    }

    pub fn tile_get(&self, x: i32, y: i32) -> Option<&RefCell<Tile>> {
        if x < 0 || y < 0 {
            return None
        }

        if x >= self.size || y >= self.size {
            return None
        }

        for (index, value) in self.tiles.iter().enumerate() {
            let tile = value.borrow();

            if tile.x == x && tile.y == y {
                return self.tiles.get(index);
            }
        }

        None
    }

    pub fn tile_get_vector(&self, vector: &vector::Vector2<i32>) -> Option<&RefCell<Tile>> {
        println!("GET {:?}", vector);
        self.tile_get(vector.x, vector.y)
    }

    pub fn tile_iterator_vec(&self) -> Vec<vector::Vector2<i32>> {
        let mut result = vec![];

        for y in 0..self.size {
            for x in 0..self.size {
                result.push(vector::Vector2::new(x, y));
            }
        }

        result
    }

    pub fn handle(&self, key: KeyboardKey) {
        println!("Key Pressed -> {:?}", key);

        let move_key: bool;
        let move_vec: vector::Vector2<i32>;

        match key {
            KeyboardKey::KEY_LEFT => {
                move_vec = vector::Vector2::new(-1, 0);
                move_key = true;
            }
            KeyboardKey::KEY_RIGHT => {
                move_vec = vector::Vector2::new(1, 0);
                move_key = true;
            }
            KeyboardKey::KEY_UP => {
                move_vec = vector::Vector2::new(0, -1);
                move_key = true;
            }
            KeyboardKey::KEY_DOWN => {
                move_vec = vector::Vector2::new(0, 1);
                move_key = true;
            }
            _ => {
                move_vec = vector::Vector2::new(0, 0);
                move_key = false;
            }
        }

        if move_key {
            for i in 0..4 {
                self.tile_iterator_vec().iter()
                    .map(|v| {
                        (
                            self.tile_get_vector(v),
                            self.tile_get_vector(&v.add(&move_vec))
                        )
                    })
                    .for_each(|v| {
                        let (curr, next) = v;

                        if curr.is_none() {
                            return;
                        }

                        let mut curr = curr.unwrap().borrow_mut();

                        match next {
                            None => {}
                            Some(next) => {
                                let mut next = next.borrow_mut();

                                next.add_value(curr.value());
                                curr.set_value(0);
                            }
                        }
                    });
            }
        }
    }

    pub fn render(&self, draw: &mut RaylibDrawHandle) {
        let tile_base = vector::Vector2::new(32, 32);

        for tile_y in 0..self.size {
            for tile_x in 0..self.size {
                let tile_pos = tile_base.copy().add(&vector::Vector2::new(
                    tile_x * self.tile_size, tile_y * self.tile_size));

                match self.tile_get(tile_x, tile_y) {
                    None => {}
                    Some(tile) => {
                        let mut tile_mut = tile.borrow_mut();

                        draw.draw_rectangle(
                            tile_pos.x,
                            tile_pos.y,
                            64,
                            64,
                            Color::DARKPURPLE,
                        );

                        draw.draw_text(
                            format!("{}", tile_mut.value()).as_str(),
                            tile_pos.x,
                            tile_pos.y,
                            32,
                            Color::RAYWHITE,
                        );
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
            value: (y * 4 + x) as i64,
        }
    }

    pub fn pos_x(&self) -> vector::Vector2<i32> {
        vector::Vector2::new(self.x, self.y)
    }

    pub fn value(&self) -> i64 { self.value }

    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }

    pub fn add_value(&mut self, value: i64) {
        self.value += value;
    }
}
