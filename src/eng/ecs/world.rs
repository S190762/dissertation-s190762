use crate::{
    eng::{
        ecs::{
            components::{
                action::ActionComponent,
                input::InputComponent,
                sprite::SpriteComponent,
                transform::TransformComponent
            }
        }
    }
};
use specs::{
    Builder,
    Entity,
    Join,
    LazyUpdate,
    System,
    World,
    saveload::{U64Marker,U64MarkerAllocator,Marker,MarkerAllocator}
};

pub fn copy_world(copy_world: &World) -> World {
    let copy_entities = copy_world.entities();
    let mut world = create_default_world();
    for ent in copy_entities.join() {
        let new_ent = {
            let mut new_ent = world.create_entity();
            macro_rules! add_copy_comp {
                ($comp:ty) => {
                    match copy_world.read_storage::<$comp>().get(ent) {
                        Some(c) => {
                            new_ent = new_ent.with(c.clone());
                        },
                        None => {}
                    };
                };
            }

            add_copy_comp!(InputComponent);
            add_copy_comp!(SpriteComponent);
            add_copy_comp!(TransformComponent);
            new_ent.build()
        };
    }

    *world.write_resource::<U64MarkerAllocator>() = copy_world.read_resource::<U64MarkerAllocator>().clone();

    world
}

pub fn copy_entity(entity: Entity, world: &mut World) -> Entity {
    let copy_ent = world.entities().create();
    {
        let lazy_updater = world.read_resource::<LazyUpdate>();

        macro_rules! add_copy_comp {
            ($comp:ty) => {
                match world.read_storage::<$comp>().get(entity) {
                    Some(c) => lazy_updater.insert(copy_ent.clone(), c.clone()),
                    None => {}
                };
            };
        }

        add_copy_comp!(InputComponent);
        add_copy_comp!(SpriteComponent);
        add_copy_comp!(TransformComponent);
        let mut alloc = world.write_resource::<<U64Marker as Marker>::Allocator>();
    }

    world.maintain();

    copy_ent
}

pub fn create_default_world() -> World {
    let mut world = World::new();

    world.register::<InputComponent>();
    world.register::<ActionComponent>();
    world.register::<SpriteComponent>();
    world.register::<TransformComponent>();
    world.register::<U64Marker>();
    world.add_resource(U64MarkerAllocator::new());

    world
}