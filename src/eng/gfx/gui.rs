use crate::eng::scenes::{
    Scene,
    SceneState
};
use crate::eng::core::context::Context as MyContext;
use imgui::{
    FontGlyphRange,
    FrameSize,
    ImFontConfig,
    ImGui,
    ImGuiCond,
    ImTexture,
    ImVec4,
    Ui,
    Window
};
use glium::{
    backend::{Context, Facade},
    Texture2d,
    glutin,
    Display,
    glutin::Event,
    Surface
};
use imgui_winit_support;
use std::rc::Rc;
use std::time::Instant;
use imgui_glium_renderer::Renderer;

use std::{
    fs::File,
    io::Read,
};


pub struct GUI {
    pub imgui: ImGui,
    pub renderer: Renderer,
    last_frame: Instant,
    pub hidpi_factor: f64
}

impl GUI {
    pub fn new(ctx: &mut MyContext) -> Self {
        let mut imgui = ImGui::init();
        imgui.set_ini_filename(None);
        let hidpi_factor = ctx.display.gl_window().get_hidpi_factor().round();

        let font_size = (13.0 * hidpi_factor) as f32;

        imgui.fonts().add_default_font_with_config(
            ImFontConfig::new()
                .oversample_h(1)
                .pixel_snap_h(true)
                .size_pixels(font_size),
        );

        imgui.set_font_global_scale((1.0 / hidpi_factor) as f32);

        let mut renderer = Renderer::init(&mut imgui, &ctx.display).expect("Failed to initialize renderer");

        imgui_winit_support::configure_keys(&mut imgui);

        GUI { imgui, renderer, last_frame: Instant::now(), hidpi_factor }
    }
    pub fn process_event(&mut self, event: &Event, hidpi: f64) {
        imgui_winit_support::handle_event(
            &mut self.imgui,
            &event,
            hidpi,
            self.hidpi_factor,
        );
    }
    pub fn render_scene_ui(&mut self, ctx: &mut MyContext, target: &mut glium::Frame, scene: &mut Box<dyn Scene>) -> SceneState {
        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        imgui_winit_support::update_mouse_cursor(&self.imgui, &ctx.display.gl_window());

        let frame_size = imgui_winit_support::get_frame_size(&ctx.display.gl_window(), self.hidpi_factor).unwrap();

        let ui = self.imgui.frame(frame_size, delta_s);

        let next_scene_state = scene.draw_ui(ctx, &ui);

        self.renderer.render(target, ui).expect("Rendering failed");

        next_scene_state
    }

}