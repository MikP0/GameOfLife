pub mod viewer;
pub mod world;

fn main() {
    let world = world::World::new(64, 64);
    let mut viewer = viewer::Viewer::new(620, 480);
    viewer.draw(world);
}
