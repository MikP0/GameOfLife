extern crate rand;
extern crate termion;

use std::{thread, time};

pub mod world;

fn main() {
    let mut world = world::World::new(128, 36);

    loop {
        world.show();
        thread::sleep(time::Duration::from_millis(200));
        world.iterate();
        thread::sleep(time::Duration::from_millis(200));
        print!("{}", termion::clear::All);
    }
}
