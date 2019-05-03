use crate::{
    eng::{
        cfg::{
            level_cfg::LevelConfig
        },
        ecs::{
            systems::{
                serializer::DeserializeSystem,
                serializer::SerializeSystem
            },
            world
        },
        core::{
            resource_manager::ResourceManager,
            context::Context
        },
        utils::{
            constants
        }
    }
};
use specs::{
    Entity,
    Dispatcher,
    DispatcherBuilder,
    RunNow,
    World,
    saveload::{U64Marker, U64MarkerAllocator, MarkerAllocator, Marker}
};
use std::{
    fs::{
        self,
        File
    },
    io::{
        Write
    },
    path::{
        Path
    }
};

pub struct Level<'a, 'b> {
    config: LevelConfig,
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    resource_manager: ResourceManager
}


pub fn clone<'l, 'l2, F: FnMut(DispatcherBuilder<'l, 'l2>) -> DispatcherBuilder<'l, 'l2>>(level: &Level, mut build_dispatcher: F) -> Level<'l, 'l2> {
    let mut world = crate::eng::ecs::world::copy_world(&level.world);
    let dispatcher_builder = DispatcherBuilder::new();
    let mut dispatcher = build_dispatcher(dispatcher_builder).build();
    dispatcher.setup(&mut world.res);

    Level {
        config: level.config.clone(),
        world,
        dispatcher,
        resource_manager: level.resource_manager.clone()
    }
}

impl<'a, 'b> Level<'a, 'b> {
    pub fn load<F: FnMut(DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b>>(ctx: &mut Context, config: LevelConfig, resources_manager: Option<ResourceManager>, build_dispatcher: F) -> Self {
        let (mut world, dispatcher) = Self::build_default_world(build_dispatcher);
        let mut resource_manager = resources_manager.unwrap_or_default();
        DeserializeSystem { reader: File::open(&config.world_data_path()).unwrap() }.run_now(&world.res);
        Level { config, world, dispatcher, resource_manager }
    }
    pub fn new<F: FnMut(DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b>, W: FnMut(&mut World) -> ()>(ctx: &mut Context, name: String, build_dispatcher: F, mut populate_world: W) -> Self {
        let (mut world, dispatcher) = Self::build_default_world(build_dispatcher);
        let mut resource_manager = ResourceManager::default();
        populate_world(&mut world);
        let dir = constants::path::GAME_LEVELS_BASE_DIR.join(Path::new(&name));
        let config = LevelConfig {
            name,
            level_dir: dir
        };
        Level { config, world, dispatcher, resource_manager }
    }
    pub fn get_world(&self) -> &World { &self.world }
    pub fn get_world_mut(&mut self) -> &mut World { &mut self.world }
    fn build_default_world<F: FnMut(DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b>>(mut build_dispatcher: F) -> (World, Dispatcher<'a, 'b>) {
        let mut world = crate::eng::ecs::world::create_default_world();
        let dispatcher_builder = DispatcherBuilder::new();
        let mut dispatcher = build_dispatcher(dispatcher_builder).build();
        dispatcher.setup(&mut world.res);
        (world, dispatcher)
    }
    pub fn save(&mut self) {
        if !self.config.level_dir.exists() {
            fs::create_dir(&self.config.level_dir).unwrap();
        }
        SerializeSystem { writer: File::create(&self.config.world_data_path()).unwrap()}.run_now(&self.world.res);
        self.config.save();
    }
    pub fn draw(&mut self, ctx: &mut Context) {

    }
    pub fn update(&mut self, _context: &mut Context, _dt: f32) {
        self.dispatcher.dispatch(&self.world.res);
        self.world.maintain();
    }
}