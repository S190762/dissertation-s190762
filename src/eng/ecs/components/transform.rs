use specs::prelude::*;
use specs_derive::Component;
use serde::{
    Deserialize,
    Serialize
};

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct TransformComponent {
    pub x : f64,
    pub y : f64
}

impl TransformComponent {
    pub fn new(x: f64, y: f64) -> Self {
        TransformComponent { x, y }
    }
}