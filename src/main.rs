extern crate rand;
extern crate termion;

use rand::Rng;
use std::ops::Not;
use std::{thread, time};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Alive = 1,
    Dead = 0,
}

impl Not for Cell {
    type Output = Cell;

    fn not(self) -> Self::Output {
        match self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}

struct World {
    width: u32,
    height: u32,
    terrain: Vec<Cell>,
}

impl World {
    fn new(width: u32, height: u32) -> Self {
        let mut terrain: Vec<Cell> = Vec::new();
        for _h in 0..height {
            for _w in 0..width {
                let mut rng = rand::thread_rng();
                let num = rng.gen_range(0, 2);
                println!("{}", num);
                if num == 1 {
                    terrain.push(Cell::Alive);
                } else {
                    terrain.push(Cell::Dead);
                }
            }
        }
        Self {
            width,
            height,
            terrain,
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn show(&self) {
        for h in 0..self.height {
            for w in 0..self.width {
                match self.terrain.get(self.get_index(h, w)) {
                    Some(Cell::Dead) => print!(" "),
                    Some(Cell::Alive) => print!("x"),
                    None => print!("x"),
                }
            }
            println!();
        }
    }

    fn alive_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.terrain[nw] as u8;

        let n = self.get_index(north, column);
        count += self.terrain[n] as u8;

        let ne = self.get_index(north, east);
        count += self.terrain[ne] as u8;

        let w = self.get_index(row, west);
        count += self.terrain[w] as u8;

        let e = self.get_index(row, east);
        count += self.terrain[e] as u8;

        let sw = self.get_index(south, west);
        count += self.terrain[sw] as u8;

        let s = self.get_index(south, column);
        count += self.terrain[s] as u8;

        let se = self.get_index(south, east);
        count += self.terrain[se] as u8;

        count
    }

    fn iterate(&mut self) {
        let mut new_terrain: Vec<Cell> = Vec::new();

        for h in 0..self.height {
            for w in 0..self.width {
                let alive_neighbours = self.alive_neighbor_count(h, w);
                let index = self.get_index(h, w);

                if self.terrain[index] == Cell::Alive {
                    if alive_neighbours > 3 {
                        new_terrain.push(!(self.terrain[index]));
                    } else if alive_neighbours < 2 {
                        new_terrain.push(!(self.terrain[index]));
                    } else {
                        new_terrain.push(self.terrain[index]);
                    }
                } else {
                    if alive_neighbours == 3 {
                        new_terrain.push(!self.terrain[index]);
                    } else {
                        new_terrain.push(self.terrain[index]);
                    }
                }
            }
        }
        self.terrain = new_terrain;
    }
}

fn main() {
    let mut world = World::new(128, 36);

    loop {
        world.show();
        thread::sleep(time::Duration::from_millis(200));
        world.iterate();
        thread::sleep(time::Duration::from_millis(200));
        print!("{}", termion::clear::All);
    }
}
