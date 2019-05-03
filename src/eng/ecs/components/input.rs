use crate::{
    eng::{
        core::{
            input_manager::JustPressed
        },
        ecs::{
            components::{
                action::Actions
            }
        }
    }
};
use serde::{
    Deserialize,
    Serialize
};
use specs::prelude::*;
use specs_derive::Component;
use glium::glutin;

#[derive(Component, Serialize, Deserialize, Clone, Default)]
pub struct InputComponent {
    pub actions: Vec<(glutin::VirtualKeyCode, JustPressed, Actions)>
}

impl InputComponent {
    pub fn new(inputs: Vec<(glutin::VirtualKeyCode, JustPressed, Actions)>) -> Self {
        let actions = inputs.into_iter().map(|(code, just_pressed, action)| (code, just_pressed, action)).collect();
        InputComponent { actions }
    }
}