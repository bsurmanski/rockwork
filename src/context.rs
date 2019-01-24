use sdl2::Sdl;
use crate::window::Window;
use std::time;

pub struct Context<T> {
    sdl: Sdl,
    pub sdl_video: sdl2::VideoSubsystem,
    pub sdl_event_pump: sdl2::EventPump,
    window: Option<Window>,
    running: bool,
    game_data: Option<T>,
}

impl<T> Context<T> {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let sdl_video = sdl_context.video().unwrap();
        let sdl_event_pump = sdl_context.event_pump().unwrap();
        Self {
            sdl: sdl_context, 
            sdl_video: sdl_video,
            sdl_event_pump: sdl_event_pump,
            window: None,
            running: true,
            game_data: None,
        }
    }

    pub fn set_game_data(&mut self, data: T) {
        self.game_data = Some(data);
    }

    pub fn game_data(&self) -> &T {
        self.game_data.as_ref().unwrap()
    }

    pub fn game_data_mut(&mut self) -> &mut T {
        self.game_data.as_mut().unwrap()
    }

    pub fn open_window(&mut self, title: String, w: usize, h: usize) -> &Window {
        self.window = Some(Window::new(self, title, w, h));
        dbg!("opening window");
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

    pub fn run(&mut self, tick_fn: &mut FnMut(&mut Context<T>, time::Duration)) {
        let mut time = time::Instant::now();
        #[cfg(target_os = "emscripten")] {
            emscripten::set_main_loop_callback(|| { 
                tick_fn(self, time.elapsed()); 
                time = time::Instant::now();
            }, 0, true);
        }

        #[cfg(not(target_os = "emscripten"))]
        'run: loop {
            tick_fn(self, time.elapsed());

            if !self.running {
                break 'run;
            }

            time = time::Instant::now();
            // should be roughly 60 fps
            std::thread::sleep(time::Duration::new(0, 1_000_000_000u32 / 60)
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
