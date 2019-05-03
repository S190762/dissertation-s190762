use crate::{
    eng::{
        ecs::{
            components::{
                sprite::SpriteComponent,
                transform::TransformComponent
            }
        }
    }
};
use serde::{
    Deserialize,
    Serialize
};
use specs::{
    prelude::*
};
use specs_derive::Component;
use glium::glutin;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, SpriteComponent>,
        ReadStorage<'a, TransformComponent>
    );
    fn run(&mut self, (sprites, transforms): Self::SystemData) {
        for(sprite, transform) in (&sprites, &transforms).join() {
            
        }
    }
}