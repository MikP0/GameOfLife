extern crate rand;
extern crate termion;

use std::{thread, time};

pub mod viewer;
pub mod world;

fn main() {
    let mut world = world::World::new(128, 36);

    let mut viewer = viewer::Viewer::new(620, 480);
    viewer.draw();

    loop {
        world.show();
        thread::sleep(time::Duration::from_millis(200));
        world.iterate();
        thread::sleep(time::Duration::from_millis(200));
        print!("{}", termion::clear::All);
    }
}
