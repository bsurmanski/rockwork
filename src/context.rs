use sdl2::Sdl;
use crate::window::Window;

pub struct Context {
    sdl: Sdl,
    window: Option<Window>,
}

impl Context {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let sdl_video = sdl_context.video().unwrap();


        Self { sdl: sdl_context, window: None }
    }

    pub fn open_window(&mut self, title: String, w: usize, h: usize) -> &Window {
        self.window = Some(Window::new(self, title, w, h));
        return self.window();
    }

    pub fn sdl_context(&self) -> &Sdl {
        &self.sdl
    }

    pub fn window(&self) -> &Window {
        return &self.window.as_ref().unwrap();
    }

    pub fn swap_buffers(&mut self) {
        self.window.as_ref().unwrap().swap_buffers();
    }

    pub fn register_event_callback() {
    }
}
