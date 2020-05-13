use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub name: String,
    pub color: String,
    pub depth: String,
    pub normal: String,
}

#[derive(Serialize, Deserialize)]
pub struct BlockAtlas {
    pub version: u8,
    pub blocks: Vec<Block>,
}

pub fn load_blocks(path: &str) -> Result<BlockAtlas, String> {
    let data = fs::read_to_string(path).map_err(|_| "Failed to open blocks json file")?;
    let atlas: BlockAtlas = serde_json::from_str(&data).map_err(|x| x.to_string())?;

    Ok(atlas)
}

