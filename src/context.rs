use crate::window::Window;
use sdl2::Sdl;
use sdl2::mixer;
use std::cell::{RefCell, Ref, RefMut};
use std::time;

pub struct Context<T> {
    sdl: Sdl,
    pub sdl_video: sdl2::VideoSubsystem,
    pub sdl_audio: sdl2::AudioSubsystem,
    pub sdl_event_pump: sdl2::EventPump,
    window: Option<Window>,
    running: bool,
    pub game_data: Option<RefCell<T>>,
}

impl<T> Context<T> {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let sdl_video = sdl_context.video().unwrap();
        let sdl_audio = sdl_context.audio().unwrap();
        let sdl_event_pump = sdl_context.event_pump().unwrap();
        mixer::open_audio(22050, mixer::DEFAULT_FORMAT, 2, 2048).unwrap();
        Self {
            sdl: sdl_context,
            sdl_video: sdl_video,
            sdl_audio: sdl_audio,
            sdl_event_pump: sdl_event_pump,
            window: None,
            running: true,
            game_data: None,
        }
    }

    pub fn open_window(&mut self, title: String, w: usize, h: usize) -> &Window {
        self.window = Some(Window::new(&self.sdl_video, title, w, h));
        dbg!("opening window");
        return self.window();
    }

    pub fn sdl_context(&self) -> &Sdl {
        &self.sdl
    }

    pub fn window(&self) -> &Window {
        return &self.window.as_ref().unwrap();
    }

    pub fn swap_buffers(&self) {
        self.window.as_ref().unwrap().swap_buffers();
    }

    pub fn attach_game_data(&mut self, t: T) {
        self.game_data = Some(RefCell::new(t));
    }

    pub fn game_data_mut(&self) -> RefMut<T> {
        self.game_data.as_ref().unwrap().borrow_mut()
    }

    pub fn game_data(&self) -> Ref<T> {
        self.game_data.as_ref().unwrap().borrow()
    }

    pub fn run(&mut self, tick_fn: &mut FnMut(&mut Context<T>, time::Duration)) {
        let mut time = time::Instant::now();
        #[cfg(target_os = "emscripten")]
        {
            let mut s = self;
            emscripten::set_main_loop_callback(
                || {
                    tick_fn(s, time.elapsed());
                    time = time::Instant::now();
                },
                0,
                true,
            );
        }

        #[cfg(not(target_os = "emscripten"))]
        'run: loop {
            tick_fn(self, time::Duration::from_millis(32));

            if !self.running {
                break 'run;
            }

            let start = time;
            let dt = time::Instant::now() - start;
            time = time::Instant::now();
            // should be roughly 30 fps
            //if let Some(sleep_dt) = time::Duration::new(0, 1_000_000_000u32 / 30).checked_sub(dt) {
                std::thread::sleep(time::Duration::from_millis(32));
            //}
        }
    }

    pub fn quit(&mut self) {
        #[cfg(target_os = "emscripten")]
        {
            // TODO: exit emscripten loop
        }
        self.running = false;
    }

    pub fn register_event_callback() {}
}
