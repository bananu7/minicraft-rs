use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::Borrow;

pub struct DefaultHashMap<K, V> {
    default: V,
    map: HashMap<K, V>,
}

impl<K, V> DefaultHashMap<K, V>
    where K: Hash + Eq,
          V: Clone,
{
    pub fn new(default: V) -> Self {
        DefaultHashMap {
            default: default,
            map: HashMap::new(),
        }
    }

    pub fn get_mut(&mut self, v: K) -> &mut V {
        let def = &self.default;
        self.map.entry(v).or_insert_with(|| def.clone())
    }

    pub fn get<B>(&self, v: B) -> &V
        where B: Borrow<K>,
    {
        self.map.get(v.borrow()).unwrap_or(&self.default)
    }

    pub fn get_map(&self) -> &HashMap<K, V> {
        return &self.map
    }
}
