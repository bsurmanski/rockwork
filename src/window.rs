use crate::context::Context;

use sdl2::video::GLProfile;


pub struct Window {
    sdl_window: sdl2::video::Window
}

impl Window {
    pub fn new(ctx: &Context, title: String, w: usize, h: usize) -> Self {
        let sdl_video = ctx.sdl_context().video().unwrap();
        let gl_attr = sdl_video.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let sdl_window = sdl_video.window(&title,
                                          w as u32,
                                          h as u32).opengl().build().unwrap();

        let ctx = sdl_window.gl_create_context().unwrap();
        gl::load_with(|name| sdl_video.gl_get_proc_address(name) as *const _);
        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (3, 3));

        Window { sdl_window: sdl_window }
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
