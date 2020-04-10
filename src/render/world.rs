use std::time::{Instant};

use crate::render::util::pipeline::Pipeline;
use crate::render::world::display_chunk::{DisplayChunk};

use crate::world::coord::{OuterChunkCoord};
use crate::world::{Field};

pub mod display_chunk;
pub mod displaychunk_gen_cold;
pub mod displaychunk_gen_hot;
pub mod traits;

pub struct DisplayField {
    display_chunks: Vec<DisplayChunk>,
    gen_cold: displaychunk_gen_cold::DisplayChunkGenCold,
    gen_hot: displaychunk_gen_hot::DisplayChunkGenHot,
}

impl DisplayField {
    pub fn new(display: &glium::Display) -> Self {
        DisplayField {
            display_chunks: Vec::new(),
            gen_cold: displaychunk_gen_cold::DisplayChunkGenCold::new(display),
            gen_hot: displaychunk_gen_hot::DisplayChunkGenHot::new(display),
        }
    }

    pub fn update(self: &mut Self, field: &Field, display: &glium::Display) {
        let chunks = field.get_chunks();
        self.display_chunks.clear();

        for (coord, chunk) in chunks.get_map() {
            let hot = coord.eq(&OuterChunkCoord::new(0,0,1));

            if hot {
                let start_time = Instant::now();

                let dc = self.gen_hot.generate((*coord).clone(), chunk, display);

                let delta = Instant::now() - start_time;
                println!("Gen time hot: {}", delta.as_micros());

                self.display_chunks.push(dc);
            } else {
                let start_time = Instant::now();

                let dc = self.gen_cold.generate((*coord).clone(), chunk, display);

                let delta = Instant::now() - start_time;
                println!("Gen time cold: {}", delta.as_micros());

                self.display_chunks.push(dc);
            }
        }
    }

    pub fn draw(self: &Self, target: &mut glium::Frame, _display: &glium::Display, pip: &Pipeline) {
        for dc in &self.display_chunks {        
            dc.draw(target, pip);
        }
    }
}