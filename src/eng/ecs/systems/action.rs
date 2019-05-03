use crate::{
    eng::{
        ecs::{
            components::{
                action::ActionComponent,
                action::Actions,
                transform::TransformComponent
            }
        }
    }
};

use specs::{
  prelude::*
};
use specs::saveload::{U64Marker, U64MarkerAllocator, Marker, MarkerAllocator};

pub struct ActionSystem;

impl<'a> System<'a> for ActionSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, U64MarkerAllocator>,
        WriteStorage<'a, TransformComponent>,
        WriteStorage<'a, ActionComponent>
    );

    fn run(&mut self, (entities, u64_marker_allocator, mut transforms, mut actions): Self::SystemData) {
        fn perform_action<'a>(entity: Entity, action: &Actions, entities: &Entities<'a>, u64_marker_allocator: &U64MarkerAllocator, transform_storage: &mut WriteStorage<'a, TransformComponent>) {
            match action {
                Actions::MoveEntity(xy) => {
                    if let Some(transform) = transform_storage.get_mut(entity) {
                        transform.x += xy.x;
                        transform.y += xy.y;
                    }
                }
                Actions::PrintText(s) => {
                    println!("{}", s);
                }
                Actions::DeleteEntity => {
                    entities.delete(entity).unwrap()
                }
                Actions::ModifyEntity(u64_marker, action) => {
                    if let Some(u64_marker) = u64_marker {
                        if let Some(ent) = u64_marker_allocator.retrieve_entity_internal(u64_marker.id()) {
                            perform_action(ent, action, entities, u64_marker_allocator, transform_storage);
                        }
                    }
                }
                Actions::ModifyEntities(u64_marker, actions) => {
                    if let Some(u64_marker) = u64_marker {
                        if let Some(ent) = u64_marker_allocator.retrieve_entity_internal(u64_marker.id()) {
                            for action in actions.iter() {
                                perform_action(ent, action, entities, u64_marker_allocator, transform_storage);
                            }
                        }
                    }
                }
                Actions::MultipleActions(actions) => {
                    for action in actions.iter() {
                        perform_action(entity, action, entities, u64_marker_allocator, transform_storage);
                    }
                }
                _ => {}
            }
        }

        for(ent, action) in (&*entities, &mut actions).join() {
            for action in action.remaining_actions.iter() {
                perform_action(ent, action, &entities, &u64_marker_allocator, &mut transforms);
            }
            action.remaining_actions.clear();
        }
    }
}