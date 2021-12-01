use std::fs;
use glium::{program};

use crate::render::util::pipeline::*;

pub fn create_program(vs_path: &str, fs_path: &str, display : &glium::Display) -> glium::Program {
    let vertex_source = fs::read_to_string(vs_path).unwrap();
    let fragment_source = fs::read_to_string(fs_path).unwrap();

    let program = program!(display,
        410 => {
            vertex: vertex_source.as_str(),
            fragment: fragment_source.as_str(),
        },
    ).unwrap();
    return program
}

pub fn easy_pipeline_vsfs(vs_path: &str, fs_path: &str, display: &glium::Display)
    -> std::cell::RefCell<Pipeline>
{
    let program = create_program(vs_path, fs_path, &display);
    let pipeline = std::cell::RefCell::new(Pipeline::new(program));
    pipeline
}