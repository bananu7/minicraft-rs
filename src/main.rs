#[macro_use]

mod render;
mod world;
mod tests;

fn main() {
    world::setup();
    render::setup()
}