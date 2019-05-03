use crate::eng::core::game::GameCFG;
use crate::eng::core::input_manager::RefInputManager;

use glium::glutin;
use glutin::ContextTrait;
use core::borrow::Borrow;

pub struct Context {
    pub events_loop: glutin::EventsLoop,
    pub display: glium::Display
}

impl Context {
    pub fn new(cfg: GameCFG) -> Self {
        let mut el = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new().with_title(cfg.window_title).with_dimensions((cfg.window_size.0, cfg.window_size.1).into());
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &el).unwrap();
        Context { events_loop: el, display }
    }
}
