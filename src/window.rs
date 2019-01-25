use crate::context::Context;

use std::ffi::CStr;
use sdl2::video::GLProfile;

pub struct Window {
    gl_context: sdl2::video::GLContext,
    sdl_window: sdl2::video::Window,
}

impl Window {
    pub fn new(ctx: &Context, title: String, w: usize, h: usize) -> Self {
        let gl_attr = ctx.sdl_video.gl_attr();
        #[cfg(not(target_os = "emscripten"))] {
            gl_attr.set_context_profile(GLProfile::Core);
            gl_attr.set_context_version(3, 2);
        }

        #[cfg(target_os = "emscripten")] {
            gl_attr.set_context_profile(GLProfile::GLES);
            gl_attr.set_context_version(3, 0);
        }

        let sdl_window = ctx.sdl_video.window(&title,
                                              w as u32,
                                              h as u32)
            .opengl()
            .build()
            .unwrap();

        let gl_context = sdl_window.gl_create_context().unwrap();

        #[cfg(target_os = "emscripten")]
        let _gl = gl::load_with(|name| emscripten::get_proc_address(name) as *const _);

        #[cfg(not(target_os = "emscripten"))]
        let _gl = gl::load_with(|name| ctx.sdl_video.gl_get_proc_address(name) as *const _);

        dbg!(unsafe { CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8) } );
        dbg!(unsafe { CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8) } );

        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::Disable(gl::BLEND);
            gl::Disable(gl::SCISSOR_TEST);
            gl::FrontFace(gl::CW);
            gl::Viewport(0, 0, w as i32, h as i32);
        }

        Window { 
            gl_context: gl_context,
            sdl_window: sdl_window,
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(0.39, 0.58, 0.92, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn swap_buffers(&self) {
        self.sdl_window.gl_swap_window();
    }
}
