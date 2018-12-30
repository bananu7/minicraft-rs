use std::hash::Hasher;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Read;
use roxmltree::*;

#[derive(Debug, Clone, Copy)]
struct CharDescriptor {
    id: i64,
    x: i64, y: i64,
    width: i64, height: i64,

    x_offset: i64, y_offset: i64,
    x_advance: i64,

    num: i64,
}

impl Hash for CharDescriptor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

struct FontDescriptor {
    data: HashMap<i64, CharDescriptor>,
    x_size: i64, y_size: i64,
}

fn load_file(path: &std::path::Path) -> String {
    let mut file = std::fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

impl FontDescriptor {
    fn load(path_to_xml: &std::path::Path) -> Result<Self, &'static str> {   
        let text = load_file(path_to_xml);
        let doc = Document::parse(&text).unwrap();

        let root = doc.root_element();

        let common = root.children().filter(|c| c.has_tag_name("common")).nth(0).unwrap();
        let x_size: i64 = common.attribute("scaleW").unwrap().parse().unwrap();
        let y_size: i64 = common.attribute("scaleH").unwrap().parse().unwrap();

        let mut num = 0i64;
        let chars = root.children().filter(|c| c.has_tag_name("chars")).nth(0).unwrap();

        let mut data = HashMap::new();

        for character in chars.children() {
            let cd = CharDescriptor {
                id: character.attribute("id").unwrap().parse().unwrap(),
                x: character.attribute("x").unwrap().parse().unwrap(),
                y: character.attribute("y").unwrap().parse().unwrap(),
                width: character.attribute("width").unwrap().parse().unwrap(),
                height: character.attribute("height").unwrap().parse().unwrap(),

                x_offset: character.attribute("xoffset").unwrap().parse().unwrap(),
                y_offset: character.attribute("yoffset").unwrap().parse().unwrap(),

                x_advance: character.attribute("xadvance").unwrap().parse().unwrap(),

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
}
