#[macro_use]

mod render;
mod world;

fn main() {
    world::setup();
    render::setup()
}