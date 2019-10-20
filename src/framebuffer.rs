extern crate gl;

use crate::texture::*;
use gl::types::*;

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

enum DepthStencil {
    Depth(Texture),
    Stencil(Texture),
    DepthStencil(Texture)
}

pub struct Framebuffer {
    id: GLuint,
    color: Vec<Texture>,
    depth_stencil: Option<DepthStencil>,
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}

impl Framebuffer {
    pub fn new_empty() -> Self {
        unsafe {
            let mut id: GLuint = 0;
            gl::GenFramebuffers(1, &mut id);
            Framebuffer {
                id: id,
                color: Vec::new(),
                depth_stencil: None,
            }
        }
    }

    pub fn new(w: usize, h: usize, bindings: &[TextureFormat]) -> Self {
        assert!(bindings.iter().filter(
                |i| match i { TextureFormat::Depth | TextureFormat::DepthStencil => true, _ => false }).count() <= 1,
                "A Framebuffer should have no more than one depth/depthstencil buffer.");

        let mut fb = Self::new_empty();

        for b in bindings {
            let t = match b {
                TextureFormat::Rgba => Texture::new_rgba(w, h),
                TextureFormat::Depth => Texture::new_depth(w, h),
                TextureFormat::DepthStencil => Texture::new_depth_stencil(w, h),
                _ => panic!("unsupported texture format bound to framebuffer")
            };
            fb.add_target(t); 
        }

        fb
    }

    pub fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            gl::DrawBuffers(self.color.len() as GLint, TARGETS.as_ptr());
        }
    }

    pub fn color_target(&self, i: usize) -> &Texture {
        &self.color[i]
    }

    pub fn add_target(&mut self, texture: Texture) {
        // TODO: support depth / stencil
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            match texture.format() {
                TextureFormat::Rgba => {
                    gl::FramebufferTexture2D(
                        gl::FRAMEBUFFER,
                        gl::COLOR_ATTACHMENT0 + self.color.len() as GLuint,
                        gl::TEXTURE_2D,
                        texture.id(),
                        0,
                    );
                    self.color.push(texture);
                }

                TextureFormat::Depth => {
                    gl::FramebufferTexture2D(
                        gl::FRAMEBUFFER,
                        gl::DEPTH_ATTACHMENT,
                        gl::TEXTURE_2D,
                        texture.id(),
                        0,
                    );
                    if self.depth_stencil.is_some() {
                        panic!("attempt to attach more than one depth/depth_stencil to a framebuffer.");
                    }
                    self.depth_stencil = Some(DepthStencil::Depth(texture));
                }
                _ => {
                    panic!("unhandled texture format added to framebuffer");
                }
            }
        }
    }

    pub fn clear(&self) {
        self.bind();

    }
}
