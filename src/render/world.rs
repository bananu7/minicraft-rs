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

pub struct AtlasTextures {
    color: glium::texture::SrgbTexture2d,
    normal: glium::texture::Texture2d,
    depth: glium::texture::Texture2d,
}

pub fn build_atlas_textures(display: &glium::Display, atlas: &self::block_atlas::BlockAtlas) -> AtlasTextures {

    let atlas_size = 1024; // power-of-two
    let tex_size = 256; // TODO make sure images are 256x256

    let color_atlas = glium::texture::srgb_texture2d::SrgbTexture2d::empty_with_format(display,
                                           glium::texture::SrgbFormat::U8U8U8U8,
                                           glium::texture::MipmapsOption::EmptyMipmaps,
                                           atlas_size, atlas_size).unwrap();

    let normal_atlas = glium::Texture2d::empty_with_format(display,
                                           glium::texture::UncompressedFloatFormat::U8U8U8U8,
                                           glium::texture::MipmapsOption::EmptyMipmaps,
                                           atlas_size, atlas_size).unwrap();

    let depth_atlas = glium::Texture2d::empty_with_format(display,
                                           glium::texture::UncompressedFloatFormat::U8U8U8U8,
                                           glium::texture::MipmapsOption::EmptyMipmaps,
                                           atlas_size, atlas_size).unwrap();

    let mut count = 0;
    for block in &atlas.blocks {
        let atlas_num = atlas_size/tex_size;

        let dest_rect = glium::Rect {
            left: (count % atlas_num) * tex_size,
            bottom: (count / atlas_num) * tex_size,
            width: tex_size,
            height: tex_size,
        };

        let color_map = load_image(&block.color);        
        color_atlas.write(dest_rect, color_map);
        unsafe { color_atlas.generate_mipmaps() };

        let normal_map = load_image(&block.normal);
        normal_atlas.write(dest_rect, normal_map);
        unsafe { normal_atlas.generate_mipmaps() };

        let depth_map = load_image(&block.depth);
        depth_atlas.write(dest_rect, depth_map);
        unsafe { depth_atlas.generate_mipmaps() };

        count += 1;
    }

    AtlasTextures {
        color: color_atlas,
        normal: normal_atlas,
        depth: depth_atlas,
    }
}

pub struct DisplayField {
    display_chunks: Vec<DisplayChunk>,
    gen_cold: displaychunk_gen_cold::DisplayChunkGenCold,
    gen_hot: displaychunk_gen_hot::DisplayChunkGenHot,

    atlas_textures: AtlasTextures,

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
        let textures = build_atlas_textures(display, &atlas);

        // --------------------------------------

        DisplayField {
            display_chunks: Vec::new(),
            gen_cold: displaychunk_gen_cold::DisplayChunkGenCold::new(display),
            gen_hot: displaychunk_gen_hot::DisplayChunkGenHot::new(display),
            atlas_textures: textures,
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
            dc.draw(
                target,
                pip,
                &self.atlas_textures.normal,
                &self.atlas_textures.color,
                &self.atlas_textures.depth,
                self.time.elapsed().as_millis() as f32 / 1000.0
            );
        }
    }
}