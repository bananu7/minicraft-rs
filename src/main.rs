#[macro_use]

mod render;
mod world;
mod tests;

fn main() {
    render::setup(&world::setup())
}