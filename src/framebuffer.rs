extern crate gl;

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

pub struct Framebuffer {
    id: GLuint
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
            Framebuffer { id: id}
        }
    }
}
