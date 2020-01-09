extern crate piston_window;

use piston_window::*;

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

    pub fn draw(&mut self) {
        while let Some(event) = self.window.next() {
            self.window.draw_2d(&event, |context, graphics, _device| {
                clear([1.0; 4], graphics);

                for x in 1..4 {
                    Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
                        [100.0 * x as f64, 0.0, 50.0, 50.0],
                        &context.draw_state,
                        context.transform,
                        graphics,
                    );
                }
            });
        }
    }
}
