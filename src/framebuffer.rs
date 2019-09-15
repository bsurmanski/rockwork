extern crate gl;

use gl::types::*;
use crate::texture::*;

static TARGETS: [GLuint; 10] = [
    gl::COLOR_ATTACHMENT0,
    gl::COLOR_ATTACHMENT1,
    gl::COLOR_ATTACHMENT2,
    gl::COLOR_ATTACHMENT3,
    gl::COLOR_ATTACHMENT4,
    gl::COLOR_ATTACHMENT5,
    gl::COLOR_ATTACHMENT6,
    gl::COLOR_ATTACHMENT7,
    gl::COLOR_ATTACHMENT8,
    gl::COLOR_ATTACHMENT9,
];

struct Binding {
    target: GLuint,
}

pub struct Framebuffer {
    id: GLuint,
    bindings: Vec<Binding>,
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}

impl Framebuffer {
    pub fn new() -> Self {
        unsafe {
            let mut id: GLuint = 0;
            gl::GenFramebuffers(1, &mut id);
            Framebuffer { id: id, bindings: Vec::new() }
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            gl::DrawBuffers(self.bindings.len() as GLint, TARGETS.as_ptr());
        }
    }

    pub fn add_target(&mut self, texture: &Texture) {
        // TODO: support depth / stencil
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            match texture.format {
                TextureFormat::Rgba => {
                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    gl::COLOR_ATTACHMENT0 + self.bindings.len() as GLuint,
                    gl::TEXTURE_2D,
                    texture.id, 0);
                }

                TextureFormat::Depth => {
                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    gl::DEPTH_ATTACHMENT,
                    gl::TEXTURE_2D,
                    texture.id, 0);
                }
                _ => { panic!("unhandled texture format added to framebuffer"); }
            }
        }
        self.bindings.push(Binding { target: texture.id });
    }
}
