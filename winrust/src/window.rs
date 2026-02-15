use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

pub struct AppWindow {
    window: Window,
}

impl AppWindow {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .unwrap();

        Self { window }
    }

    pub fn request_redraw(&self) {
        // Draw.
        self.window.request_redraw();
    }
}
