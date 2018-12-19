#[macro_use]
extern crate glium;

mod render;
mod world;

fn main() {
    world::setup();
    render::setup()
}