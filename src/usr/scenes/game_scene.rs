use crate::{
    eng::{
        core::{
            context::Context,
            input_manager::RefInputManager,
            resource_manager::RefResourceManager,
            resource_manager::ResourceManager,
            level::Level,
        },
        cfg::{
            level_cfg::LevelConfig
        },
        ecs::{
            systems::{
                input::InputSystem,
                action::ActionSystem
            }
        },
        utils::{
            constants
        },
        scenes::{
            NextState,
            Scene,
            SceneState
        }
    }
};
use glium::{
    Surface,
    glutin,
    glutin::VirtualKeyCode
};
use std::{
    collections::VecDeque
};
use imgui::Ui;
use imgui::*;
use imgui_sys::*;
use imgui::im_str;

pub struct GameScene<'a, 'b> {
    level: Level<'a, 'b>,
    input_manager: RefInputManager
}

impl<'a, 'b> GameScene<'a, 'b> {
    pub fn new(ctx: &mut Context, input_manager: RefInputManager, level_config: LevelConfig) -> Self {
        let level = Level::load(ctx, level_config, None, |builder| {
            builder.with(InputSystem { input_manager: input_manager.clone() }, "input_manager", &[]).
                with(ActionSystem, "action_system", &["input_manager"])
        });
        GameScene {level, input_manager}
    }
}

impl<'a, 'b> Scene for GameScene<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context, _dt: f32) -> SceneState {
        self.level.update(_ctx, _dt);
        NextState::Continue
    }
    fn draw(&mut self, ctx: &mut Context, trgt: &mut glium::Frame) -> SceneState {
        self.level.draw(ctx);
        NextState::Continue
    }
    fn draw_ui(&mut self, ctx: &mut Context, ui: &Ui) -> SceneState {
        let mut result = NextState::Continue;

        ui.with_color_vars(&[(ImGuiCol::WindowBg, (0., 0., 0., 0.))], || {
            ui.window(im_str!("Main Menu")).title_bar(false).size((320.0,240.0), ImGuiCond::Appearing).movable(false).resizable(false).build(|| {
                ui.text(im_str!("Game Main State"));
            });
        });
        result
    }
}