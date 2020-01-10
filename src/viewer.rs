extern crate piston_window;
extern crate termion;

use crate::world::*;
use piston_window::*;
use std::{thread, time};

pub struct Viewer {
    window: PistonWindow,
    width: u32,
    height: u32,
}

impl Viewer {
    pub fn new(width: u32, height: u32) -> Self {
        let window: PistonWindow = WindowSettings::new("", [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap();

        Self {
            window,
            width,
            height,
        }
    }

    pub fn draw(&mut self, mut world: World) {
        while let Some(event) = self.window.next() {
            self.window.draw_2d(&event, |context, graphics, _device| {
                clear([0.0; 4], graphics);
                for h in 0..world.height {
                    for w in 0..world.width {
                        if world.terrain[world.get_index(h, w)] == Cell::Alive {
                            Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
                                [10.0 * w as f64, 10.0 * h as f64, 5.0, 5.0],
                                &context.draw_state,
                                context.transform,
                                graphics,
                            );
                        }
                    }
                }
            });
            thread::sleep(time::Duration::from_millis(40));
            world.iterate();
        }
    }
}
