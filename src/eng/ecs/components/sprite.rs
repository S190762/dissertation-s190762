use specs::prelude::*;
use specs_derive::Component;
use serde::{
    Deserialize,
    Serialize
};

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct SpriteComponent {
    pub filename: String
}

impl SpriteComponent {
    pub fn new(filename: String) -> Self {
        SpriteComponent {filename}
    }
}