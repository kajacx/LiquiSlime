use macroquad::prelude::Texture2D;
use std::collections::HashMap;

pub struct TextureAtlas {
    pub textures: HashMap<String, Texture2D>,
}

impl TextureAtlas {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, texture: Texture2D) {
        self.textures.insert(name, texture);
    }

    pub fn get(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }
}
