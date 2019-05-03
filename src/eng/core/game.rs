use crate::{
    eng::{
        core::{
           context::Context,
        input_manager::RefInputManager,
            input_manager::InputHandler
        },
        scenes::{
            NextState,
            Scene,
            SceneState,
        },
        gfx::{
            gui::GUI
        },
        utils::{
            constants
        }
    },
    usr::{
        scenes::{
            menu_scene::MenuScene,
            game_scene::GameScene
        }
    }
};
use serde::{
    Deserialize,
    Serialize,
};
use ron;
use std::{
    collections::VecDeque,
    fs::File,
    io::Write,
    string::ToString
};
use glium::Surface;
use glium::glutin;
use crate::eng::core::resource_manager::RefResourceManager;
use core::borrow::BorrowMut;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameCFG {
    pub window_size: (u32, u32),
    pub window_title: String,
    pub window_icon: String,
    pub resizable: bool,
    pub vsync: bool,
    pub fullscreen: bool
}

impl GameCFG {
    fn load() -> Result<Self, ron::de::Error> {
        let config_file = File::open(constants::path::GAME_CONFIG_FILE_LOC.as_path()).map_err(|err| ron::de::Error::from(err))?;
        ron::de::from_reader::<File, Self>(config_file)
    }
    fn save(&self) {
        let mut config_file = File::create(constants::path::GAME_CONFIG_FILE_LOC.as_path()).expect("Could not create configuration files");
        let content = ron::ser::to_string_pretty(&self, Default::default()).expect("Could not searilize configuration");
        config_file.write_all(content.as_bytes()).expect("Could not save configuration file");
    }
}

impl Default for GameCFG {
    fn default() -> Self {
        GameCFG { window_size: (800,600), window_title: "Engine".to_string(), window_icon: "".to_string(),
        resizable: false, vsync: false, fullscreen: false}
    }
}

pub struct Game {
    ctx: Context,
    scenes: VecDeque<Box<dyn Scene>>,
    input_manager: RefInputManager,
    input_handler: InputHandler,
    gui: GUI,
    quit: bool
}

impl Game {
    pub fn new() -> Self {
        let GameConfig  : GameCFG = GameCFG::load().unwrap_or_else(|err| {
            eprintln!("Configuration file does not exist, creating file instead and using defaults. Error: {}", err);
            let config = GameCFG::default();
            config.save();
            config
        });
        let mut context: Context = Context::new(GameConfig);
        let mut gui = GUI::new(&mut context);
        let input_manager = RefInputManager::default();
        let input_handler = InputHandler::new();
        let mut scenes: VecDeque<Box<dyn Scene>> = VecDeque::new();
        let mut menu_scene = Box::new(MenuScene::new(&mut context, input_manager.clone()));
        menu_scene.init_ui(&mut context, &mut gui);
        scenes.push_back(menu_scene);
        Game { ctx: context, input_manager, input_handler, scenes, gui, quit: false }
    }
    fn handle_input(ctx: &mut Context, input_manager: &mut RefInputManager, input_handler: &mut InputHandler, gui: &mut GUI, quit: &mut bool) {
        let hidpi = ctx.display.gl_window().get_hidpi_factor();
        ctx.events_loop.poll_events(|ev| {
            let mut eva = &ev;
            gui.process_event(&eva, hidpi);
            match ev {
                glutin::Event::WindowEvent {event, ..} => match event {
                    e => input_handler.process_input(&input_manager, &e,  quit),
                    _=> (),
                },
                _ => ()
            }
        });
    }
    fn handle_scene_state(result: SceneState, ctx: &mut Context, scenes: &mut VecDeque<Box<dyn Scene>>, gui: &mut GUI, quit: &mut bool) {
        match result {
            NextState::Continue => {

            }
            NextState::Push(mut scene) => {
                scene.init_ui(ctx, gui);
                scenes.push_front(scene);
            }
            NextState::Replace(mut scene) => {
                scene.init_ui(ctx, gui);
                scenes.pop_front();
                scenes.push_front(scene);
            }
            NextState::Pop => {
                scenes.pop_front();
            }
            NextState::QuitGame => {
                *quit = true;
            }
        }
    }
    pub fn run(&mut self) {
        while !self.quit {
            let dt = 1.0;
            let Game { ref mut ctx, ref mut scenes, ref mut input_manager, ref mut input_handler, ref mut gui, ref mut quit, .. } = self;
            Self::handle_input(ctx, input_manager, input_handler, gui, quit);
            //handle update
            Self::handle_scene_state(scenes.front_mut().unwrap().update(ctx, dt), ctx, scenes, gui,quit);
            input_manager.lock().unwrap().update();
            //draw
            let mut target = ctx.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            Self::handle_scene_state(scenes.front_mut().unwrap().draw(ctx, &mut target), ctx, scenes,gui, quit);
            Self::handle_scene_state(gui.render_scene_ui(ctx, &mut target, scenes.front_mut().unwrap()), ctx, scenes, gui, quit);
            target.finish().unwrap();
        }
    }
}