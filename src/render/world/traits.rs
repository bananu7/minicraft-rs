
use crate::world::coord::{OuterChunkCoord};
use crate::world::{Chunk};
use crate::render::world::display_chunk::{DisplayChunk};

pub trait DisplayChunkGen {
    fn generate(&mut self, chunk_coord: OuterChunkCoord, chunk: &Chunk, display: &glium::Display) -> DisplayChunk;
}
