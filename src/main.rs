#[derive(Debug)]
enum Cell {
    Alive,
    Dead,
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
                terrain.push(Cell::Dead) //TODO: Add more options to world creation
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

    fn show(self) {
        for h in 0..self.height {
            for w in 0..self.width {
                match self.terrain.get(self.get_index(h, w)) {
                    Some(Cell::Dead) => print!("0"),
                    Some(Cell::Alive) => print!("1"),
                    None => print!("x"),
                }
                //print!(" {:?} ", cell.state)
            }
            println!();
        }
    }
}

fn main() {
    let world = World::new(64, 32);
    world.show();
    //loop {}
}
