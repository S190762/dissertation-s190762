use crate::{
    eng::{
        core::{
            context::Context,
            input_manager::RefInputManager,
            resource_manager::RefResourceManager
        },
        cfg::{
            level_cfg::LevelConfig
        },
        utils::{
            constants
        },
        scenes::{
            NextState,
            Scene,
            SceneState
        }
    },
    usr::{
        scenes::{
            game_scene::GameScene
        }
    }
};

use glium::Surface;
use imgui_sys::*;
use imgui::*;
use walkdir::WalkDir;
use imgui::Ui;
use imgui::im_str;
use std::ffi::OsStr;
use imgui::ImGuiCol;
use glium::glutin;

pub struct MenuScene {
    resource_manager: RefResourceManager,
    input_manager: RefInputManager,
    levels: Vec<LevelConfig>
}

impl MenuScene {
    pub fn new(ctx: &mut Context, input_manager: RefInputManager) -> Self {
        let resource_manager = RefResourceManager::default();
        let levels: Vec<LevelConfig> = WalkDir::new(constants::path::GAME_LEVELS_BASE_DIR.as_path()).into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| { e.file_name() == OsStr::new(constants::path::GAME_LEVELS_CFG_NAME.as_path())})
            .filter_map(|config_file| LevelConfig::load(config_file.path().parent().unwrap().to_owned()).ok())
            .collect();
        MenuScene {resource_manager, input_manager, levels}
    }
}

impl Scene for MenuScene {
    fn update(&mut self, _ctx: &mut Context, _dt: f32) -> SceneState {
        let mut next_state = NextState::Continue;
        next_state
    }
    fn draw(&mut self, ctx: &mut Context, trgt: &mut glium::Frame) -> SceneState {
        NextState::Continue
    }
    fn draw_ui(&mut self, ctx: &mut Context, ui: &Ui) -> SceneState {
        let mut result = NextState::Continue;

        let MenuScene { ref levels, ref resource_manager, ref input_manager, .. } = self;

        ui.with_color_vars(&[(ImGuiCol::WindowBg, (0., 0., 0., 0.))], || {
            ui.window(im_str!("Main Menu")).title_bar(false).size((320.0,240.0), ImGuiCond::Appearing).movable(false).resizable(false).build(|| {
                if ui.button(im_str!("Load Game"), (-1., 0.)) {
                    if let Some(config) = levels.iter().nth(0) {
                        result = NextState::Replace(Box::new(GameScene::new(ctx, self.input_manager.clone(), config.clone())));
                    }
                }
                ui.button(im_str!("Exit Game"), (-1., 0.));
            });
        });
        result
    }
}