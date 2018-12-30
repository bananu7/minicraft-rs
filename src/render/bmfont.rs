use std::collections::HashMap;
use std::io::Read;
use roxmltree::*;

#[derive(Debug, Clone, Copy)]
pub struct CharDescriptor {
    id: i64,
    x: i64, y: i64,
    width: i64, height: i64,

    x_offset: i64, y_offset: i64,
    x_advance: i64,

    num: i64,
}

pub struct FontDescriptor {
    data: HashMap<i64, CharDescriptor>,
    x_size: i64, y_size: i64,
}

fn load_file(path: &std::path::Path) -> String {
    let mut file = std::fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn get_attrib<T>(n: Node, attrib_name: &str) -> Result<T, String> 
where T: std::str::FromStr
{
    let attrib: &str = n.attribute(attrib_name).ok_or(format!("Not existing attribute {}", attrib_name))?;
    attrib.parse::<T>().map_err(|_| format!("Failed to parse attribute {}", attrib_name))
}

impl FontDescriptor {
    pub fn load(path_to_xml: &std::path::Path) -> Result<Self, String> {   
        let text = load_file(path_to_xml);
        let doc = Document::parse(&text).unwrap();

        let root = doc.root_element();

        let common = root.children().filter(|c| c.has_tag_name("common")).nth(0).unwrap();
        let x_size: i64 = get_attrib(common, "scaleW")?;
        let y_size: i64 = get_attrib(common, "scaleH")?;

        let mut num = 0i64;
        let chars = root.children().filter(|c| c.has_tag_name("chars")).nth(0).unwrap();

        let mut data = HashMap::new();

        for character in chars.children().filter(|c| c.has_tag_name("char")) {
            let cd = CharDescriptor {
                id: get_attrib(character, "id")?,
                x: get_attrib(character, "x")?,
                y: get_attrib(character, "y")?,
                width: get_attrib(character, "width")?,
                height: get_attrib(character, "height")?,

                x_offset: get_attrib(character, "xoffset")?,
                y_offset: get_attrib(character, "yoffset")?,

                x_advance: get_attrib(character, "xadvance")?,

                num: num,
            };
            num += 1;

            data.insert(cd.id, cd);
        }

        Ok(FontDescriptor {
            data: data,
            x_size: x_size,
            y_size: y_size,
        })
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }
}
