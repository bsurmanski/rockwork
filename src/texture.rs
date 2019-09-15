extern crate gl;
extern crate image;

use gl::types::*;
use std::ffi::*;

#[macro_export]
macro_rules! include_png_texture {
    ($x:literal) => {
        Texture::new_rgba_from_image(
            &mut image::load(
                &mut std::io::Cursor::new(include_bytes!($x).as_ref()),
                image::ImageFormat::PNG,
            )
            .unwrap(),
        )
    };
}

#[macro_export]
macro_rules! include_tga_texture {
    ($x:literal) => {
        Texture::new_rgba_from_image(
            &mut image::load(
                &mut std::io::Cursor::new(include_bytes!($x).as_ref()),
                image::ImageFormat::TGA,
            )
            .unwrap(),
        )
    };
}

pub enum FilteringMode {
    Linear,
    Nearest,
}

impl FilteringMode {
    pub fn gl_enum(&self) -> GLuint {
        match self {
            FilteringMode::Linear => gl::LINEAR,
            FilteringMode::Nearest => gl::NEAREST,
        }
    }
}

pub enum WrapMode {
    Clamp,
    Mirrored,
    Repeat,
}

pub enum TextureFormat {
    Invalid,
    Rgba,
    Depth,
    DepthStencil,
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
    pub width: usize,
    pub height: usize,
    pub filtering_mode: FilteringMode,
    pub wrap_mode: WrapMode,
    pub format: TextureFormat,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

impl Texture {
    fn new(width: usize, height: usize, format: TextureFormat) -> Self {
        unsafe {
            let mut id: GLuint = 0;
            gl::GenTextures(1, &mut id);
            let mut ret = Self {
                id,
                width,
                height,
                format,
                wrap_mode: WrapMode::Clamp,
                filtering_mode: FilteringMode::Nearest,
            };
            ret.set_filtering_mode(FilteringMode::Nearest);
            ret.set_wrap_mode(WrapMode::Repeat);
            return ret;
        }
    }

    pub fn new_rgba_from_image(img: &mut image::DynamicImage) -> Self {
        unsafe {
            // flipping vertical because GL is indexed from the bottom.
            let rgba = image::imageops::flip_vertical(&img.to_rgba());
            let mut texture = Self::new(
                rgba.width() as usize,
                rgba.height() as usize,
                TextureFormat::Rgba,
            );
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                rgba.width() as i32,
                rgba.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                rgba.into_raw().as_ptr() as *const c_void,
            );
            return texture;
        }
    }

    pub fn new_rgba(w: usize, h: usize) -> Self {
        unsafe {
            let mut texture = Self::new(w, h, TextureFormat::Rgba);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                w as i32,
                h as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            return texture;
        }
    }

    pub fn new_depth(w: usize, h: usize) -> Self {
        let texture = Self::new(w, h, TextureFormat::Depth);
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::DEPTH_COMPONENT32 as i32,
                w as i32,
                h as i32,
                0,
                gl::DEPTH_COMPONENT,
                gl::FLOAT,
                std::ptr::null(),
            );
        }
        return texture;
    }

    pub fn new_depth_stencil(w: usize, h: usize) -> Self {
        let texture = Self::new(w, h, TextureFormat::DepthStencil);
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::DEPTH24_STENCIL8 as i32,
                w as i32,
                h as i32,
                0,
                gl::DEPTH_STENCIL,
                gl::FLOAT,
                std::ptr::null(),
            );
        }
        return texture;
    }

    pub fn set_filtering_mode(&mut self, mode: FilteringMode) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                mode.gl_enum() as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                mode.gl_enum() as GLint,
            );
        }
        self.filtering_mode = mode;
    }

    pub fn set_wrap_mode(&mut self, mode: WrapMode) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, mode.gl_enum() as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, mode.gl_enum() as GLint);
        }
        self.wrap_mode = mode;
    }

    pub fn bind(&self, index: usize) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index as GLuint);
            //gl::Enable(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
