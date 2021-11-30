
pub struct Object {
	pub document: gltf::Document,
	pub buffers: Vec<gltf::buffer::Data>,
	pub images: Vec<gltf::image::Data>,
}

impl Object {
	pub fn new(path: &str) -> Result<Object, ()> {
		let (document, buffers, images) = gltf::import(path).map_err(|_|())?;
		Ok(Object {
			document: document,
			buffers: buffers,
			images: images,
		})
	}
}
