use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
    },
};
use crate::eng::core::context::Context;
use glium::glutin;
use crate::eng::gfx::gui::GUI;
use imgui::ImGui;

pub type JustPressed = bool;
pub type RefInputManager = Arc<Mutex<InputManager>>;

unsafe impl Send for InputManager {}
unsafe impl Sync for InputManager {}

pub struct InputManager {
    pressed_keys: HashMap<glutin::VirtualKeyCode, JustPressed>
}

impl Default for InputManager {
    fn default() -> Self {
        InputManager::new()
    }
}

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {}
    }
    pub fn process_ui(&mut self, event: &glutin::Event) {

    }
    pub fn process_input(&mut self, input: &RefInputManager,  event: &glutin::WindowEvent, quit: &mut bool) {
        let mut input_manager = input.lock().unwrap();
        match *event {
            glutin::WindowEvent::CloseRequested => *quit = true,
            _=> ()
        };
        let input = match *event {
            glutin::WindowEvent::KeyboardInput{input, ..} => input,
            _=> return
        };
        let pressed = input.state == glutin::ElementState::Pressed;
        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return
        };
        match key {
            glutin::VirtualKeyCode::Q => *quit = true,
            _ => ()
        };
        input_manager.update_key(key, pressed);
    }
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            pressed_keys: HashMap::new()
        }
    }
    pub fn is_key_pressed(&self, key: glutin::VirtualKeyCode) -> Option<JustPressed> {
        self.pressed_keys.get(&key).cloned()
    }
    pub fn update(&mut self) {
        for (_, just_pressed) in self.pressed_keys.iter_mut() {
            *just_pressed = false;
        }
    }
    pub fn update_key(&mut self, key: glutin::VirtualKeyCode, pressed: bool) {
        if pressed {
            let jp = *self.pressed_keys.get(&key).unwrap_or(&true);
            self.pressed_keys.insert(key, jp);
        } else {
            self.pressed_keys.remove(&key);
        }
    }

}