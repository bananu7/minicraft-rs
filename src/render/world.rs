use std::time::{Instant};

use crate::render::util::pipeline::Pipeline;

use crate::world::coord::{OuterChunkCoord};
use crate::world::{Field};

use self::display_chunk::{DisplayChunk};
use self::traits::{DisplayChunkGen};

mod block_atlas;
pub mod display_chunk;
pub mod displaychunk_gen_cold;
pub mod displaychunk_gen_hot;
pub mod traits;

pub struct DisplayField {
    display_chunks: Vec<DisplayChunk>,
    gen_cold: displaychunk_gen_cold::DisplayChunkGenCold,
    gen_hot: displaychunk_gen_hot::DisplayChunkGenHot,

    normal_map: glium::texture::Texture2d,
    depth_map: glium::texture::Texture2d,
    color_map: glium::texture::CompressedSrgbTexture2d,

    time: Instant,
}

fn load_image(path: &str) -> glium::texture::RawImage2d<u8> {
    let image = image::open(path)
        .map(|i| i.to_rgba() )
        .map_err(|_| () ).unwrap();

    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    return image;
}

impl DisplayField {
    pub fn new(display: &glium::Display) -> Self {
        // TEXTURE ----------------------------
        let atlas = block_atlas::load_blocks("data/blocks.json").unwrap();
        let block = &atlas.blocks[1];

        let normal_map = glium::texture::Texture2d::new(display, load_image(&block.normal)).unwrap();
        let color_map = glium::texture::CompressedSrgbTexture2d::new(display, load_image(&block.color)).unwrap();
        let depth_map = glium::texture::Texture2d::new(display, load_image(&block.depth)).unwrap();
        // --------------------------------------

        DisplayField {
            display_chunks: Vec::new(),
            gen_cold: displaychunk_gen_cold::DisplayChunkGenCold::new(display),
            gen_hot: displaychunk_gen_hot::DisplayChunkGenHot::new(display),

            normal_map: normal_map,
            color_map: color_map,
            depth_map: depth_map,

            time: Instant::now(),
        }
    }

    pub fn update(self: &mut Self, field: &Field, display: &glium::Display) {
        let chunks = field.get_chunks();
        self.display_chunks.clear();

        for (coord, chunk) in chunks.get_map() {
            let hot = coord.eq(&OuterChunkCoord::new(0,0,1));

            if hot {
                let dc = self.gen_hot.generate((*coord).clone(), chunk, display);
                self.display_chunks.push(dc);
            } else {
                let dc = self.gen_cold.generate((*coord).clone(), chunk, display);
                self.display_chunks.push(dc);
            }
        }
    }

    pub fn draw(self: &Self, target: &mut glium::Frame, _display: &glium::Display, pip: &Pipeline) {
        for dc in &self.display_chunks {        
            // Temp
            dc.draw(target, pip, &self.normal_map, &self.color_map, &self.depth_map, self.time.elapsed().as_millis() as f32 / 1000.0);
        }
    }
}