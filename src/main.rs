use std::fmt;

#[derive(Debug)]
enum CellState {
    Alive,
    Dead,
}

struct Point {
    x: u32,
    y: u32,
    state: CellState,
}

struct World {
    width: u32,
    height: u32,
    terrain: Vec<Point>,
}

impl World {
    fn new(width: u32, height: u32) -> Self {
        let mut terrain: Vec<Point> = Vec::new();
        for h in 0..height {
            for w in 0..width {
                terrain.push(Point {
                    x: h,
                    y: w,
                    state: CellState::Dead,
                })
            }
        }
        Self {
            width,
            height,
            terrain,
        }
    }

    fn show(self) {
        for cell in self.terrain {
            match cell.state {
                CellState::Dead => print!("0"),
                CellState::Alive => print!("1"),
            }
            //print!(" {:?} ", cell.state)
        }
    }
}

fn main() {
    let world = World::new(32, 32);
    world.show();
    loop {}
}
