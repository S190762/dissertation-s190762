use crate::{
    eng::{
        core::{
            input_manager::JustPressed,
            input_manager::RefInputManager
        },
        ecs::{
            components::{
                action::ActionComponent,
                action::Actions,
                input::InputComponent
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

pub struct InputSystem {
    pub input_manager: RefInputManager
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, InputComponent>,
        WriteStorage<'a, ActionComponent>
    );

    fn run(&mut self, (entities, inputs, mut actions): Self::SystemData) {
        for(ent, input) in (&*entities, &inputs).join() {
            for(key, just_pressed, action) in input.actions.iter() {
                if let Some(jp) = self.input_manager.lock().unwrap().is_key_pressed(*key) {
                    if jp == *just_pressed {
                        let mut inserted = false;
                        {
                            if let Some(a) = actions.get_mut(ent) {
                                a.remaining_actions.push(action.clone());
                                inserted = true;
                            }
                        }

                        if !inserted {
                            actions.insert(ent, ActionComponent { remaining_actions: vec![action.clone()]}).unwrap();
                        }
                    }
                }
            }
        }
    }
}