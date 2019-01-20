use sdl2::Sdl;
use crate::window::Window;
use std::time;

pub struct Context {
    sdl: Sdl,
    pub sdl_video: sdl2::VideoSubsystem,
    pub sdl_event_pump: sdl2::EventPump,
    window: Option<Window>,
    running: bool,
}

impl Context {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let sdl_video = sdl_context.video().unwrap();
        let sdl_event_pump = sdl_context.event_pump().unwrap();
        Self {
            sdl: sdl_context, 
            sdl_video: sdl_video,
            sdl_event_pump: sdl_event_pump,
            window: None,
            running: true 
        }
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

    pub fn run(&mut self, tick_fn: &mut FnMut(&mut Context)) {
        #[cfg(target_os = "emscripten")] {
            emscripten::set_main_loop_callback(|| { tick_fn(self); }, 60, true);
        }

        #[cfg(not(target_os = "emscripten"))]
        'run: loop {
            let _start = time::Instant::now();
            tick_fn(self);

            if !self.running {
                break 'run;
            }

            // should be roughly 60 fps
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60)
                               );//- start.elapsed());
        }
    }

    pub fn quit(&mut self) {
        #[cfg(target_os = "emscripten")] {
            // TODO: exit emscripten loop
        }
        self.running = false;
    }

    pub fn register_event_callback() {
    }
}
