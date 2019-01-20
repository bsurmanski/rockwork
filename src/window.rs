use crate::context::Context;

use sdl2::video::GLProfile;


pub struct Window {
    gl_context: sdl2::video::GLContext,
    sdl_window: sdl2::video::Window,
    sdl_video: sdl2::VideoSubsystem,
    sdl_event_pump: sdl2::EventPump,
}

impl Window {
    pub fn new(ctx: &Context, title: String, w: usize, h: usize) -> Self {
        let sdl_video = ctx.sdl_context().video().unwrap();

        let sdl_window = sdl_video.window(&title,
                                          w as u32,
                                          h as u32)
            .opengl()
            .build()
            .unwrap();

        let gl_context = sdl_window.gl_create_context().unwrap();

        #[cfg(target_os = "emscripten")]
        gl::load_with(|name| emscripten::get_proc_address(name) as *const _);

        #[cfg(not(target_os = "emscripten"))]
        gl::load_with(|name| sdl_video.gl_get_proc_address(name) as *const _);

        Window { 
            gl_context: gl_context,
            sdl_window: sdl_window,
            sdl_video: sdl_video,
            sdl_event_pump: ctx.sdl_context().event_pump().unwrap(),
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn swap_buffers(&self) {
        self.sdl_window.gl_swap_window();
    }
}
