extern crate gl;
extern crate image;

use gl::types::*;
use std::ffi::*;

pub enum FilteringMode {
    Linear,
    Nearest,
}

impl FilteringMode {
    pub fn gl_enum(&self) -> GLuint {
        match self {
            FilteringMode::Linear => gl::LINEAR,
            FilteringMode::Nearest => gl::NEAREST
        }
    }
}

pub enum WrapMode {
    Clamp,
    Mirrored,
    Repeat,
}

impl WrapMode {
    pub fn gl_enum(&self) -> GLuint {
        match self {
            WrapMode::Clamp => gl::CLAMP_TO_EDGE,
            WrapMode::Mirrored => gl::MIRRORED_REPEAT,
            WrapMode::Repeat => gl::REPEAT,
        }
    }
}

pub struct Texture {
    pub id: GLuint,
    width: usize,
    height: usize,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

impl Texture {
    fn new(w: usize, h: usize) -> Self {
        unsafe {
            let mut id: GLuint = 0;
            gl::GenTextures(1, &mut id);
            Self { id: id, width: w, height: h }
        }
    }

    pub fn new_rgba_from_image(img: &mut image::DynamicImage) -> Self {
        unsafe {
            let rgba = img.to_rgba();
            let mut texture = Self::new(rgba.width() as usize, rgba.height() as usize);
            let buf = img.to_rgba().into_raw();
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGBA as i32,
                           rgba.width() as i32,
                           rgba.height() as i32,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           buf.as_ptr() as *const c_void);
            texture.set_filtering_mode(FilteringMode::Nearest);
            texture.set_wrap_mode(WrapMode::Repeat);
            return texture;
        }
    }

    pub fn new_rgba(w: usize, h: usize) -> Self {
        unsafe {
            let mut texture = Self::new(w, h);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGBA as i32,
                           w as i32,
                           h as i32,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           std::ptr::null());
            texture.set_filtering_mode(FilteringMode::Nearest);
            texture.set_wrap_mode(WrapMode::Repeat);
            return texture;
        }
    }

    pub fn new_depth(w: usize, h: usize) -> Self {
        let texture = Self::new(w, h);
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::DEPTH_COMPONENT32 as i32, 
                           w as i32, 
                           h as i32, 
                           0,
                           gl::DEPTH_COMPONENT,
                           gl::FLOAT,
                           std::ptr::null());
        }
        return texture;
    }

    pub fn new_depth_stencil(w: usize, h: usize) -> Self {
        let texture = Self::new(w, h);
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::DEPTH24_STENCIL8 as i32, 
                           w as i32, 
                           h as i32, 
                           0,
                           gl::DEPTH_STENCIL,
                           gl::FLOAT,
                           std::ptr::null());
        }
        return texture;
    }

    pub fn set_filtering_mode(&mut self, mode: FilteringMode) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, mode.gl_enum() as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mode.gl_enum() as GLint);
        }
    }

    pub fn set_wrap_mode(&mut self, mode: WrapMode) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, mode.gl_enum() as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, mode.gl_enum() as GLint);
        }
    }
}
