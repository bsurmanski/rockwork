use crate::context::Context;

use sdl2::video::GLProfile;


pub struct Window {
    sdl_window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
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
            sdl_window: sdl_window,
            gl_context: gl_context
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
