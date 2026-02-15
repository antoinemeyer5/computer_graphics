use std::collections::HashSet;

use winit::{
    event::{ElementState, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default)]
pub struct InputState {
    keys_down: HashSet<KeyCode>,
}

impl InputState {
    pub fn handle(&mut self, event: &WindowEvent) {
        if let WindowEvent::KeyboardInput { event, .. } = event {
            if let PhysicalKey::Code(code) = event.physical_key {
                match event.state {
                    ElementState::Pressed => {
                        self.keys_down.insert(code);
                        println!("Key down: {:?}", code);
                    }
                    ElementState::Released => {
                        self.keys_down.remove(&code);
                    }
                }
            }
        }
    }
}
