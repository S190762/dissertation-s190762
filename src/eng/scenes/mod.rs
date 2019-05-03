use crate::eng::core::context::Context;
use crate::eng::gfx::gui::GUI;
use imgui::Ui;

pub enum NextState {
    Continue,
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
    Pop,
    QuitGame
}

pub type SceneState = NextState;

pub trait Scene {
    fn update(&mut self, ctx: &mut Context, dt: f32) -> SceneState;
    fn draw(&mut self, ctx: &mut Context, trgt: &mut glium::Frame) -> SceneState;
    fn init_ui(&mut self, ctx: &mut Context, gui: &mut GUI) {}
    fn draw_ui(&mut self, ctx: &mut Context, ui: &Ui) -> SceneState;
}