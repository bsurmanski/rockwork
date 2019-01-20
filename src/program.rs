extern crate gl;

use std::io::Read;
use std::io::Error;
use gl::types::*;

use crate::shader::*;
use crate::framebuffer::*;

pub struct Program {
    pub id: GLuint,
    name: String,
    vertex_shader: Option<Shader>,
    geometry_shader: Option<Shader>,
    fragment_shader: Option<Shader>,
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Program {
    pub fn new(name: String) -> Self {
        unsafe {
            Program {
                id: gl::CreateProgram(),
                name: name,
                vertex_shader: None,
                geometry_shader: None,
                fragment_shader: None,
            }
        }
    }

    pub fn add_vertex_shader(&mut self, src: &mut Read) -> Result<(), Error> {
        let shader = Shader::new_with_source(ShaderStage::Vertex, src)?;
        self.vertex_shader = Some(shader);
        Ok(())
    }

    pub fn add_geometry_shader(&mut self, src: &mut Read) -> Result<(), Error> {
        let shader = Shader::new_with_source(ShaderStage::Geometry, src)?;
        self.geometry_shader = Some(shader);
        Ok(())
    }

    pub fn add_fragment_shader(&mut self, src: &mut Read) -> Result<(), Error> {
        let shader = Shader::new_with_source(ShaderStage::Fragment, src)?;
        self.fragment_shader = Some(shader);
        Ok(())
    }

    unsafe fn get_link_error(&self) -> Result<(), String> {
        let mut err = 0;
        gl::GetShaderiv(self.id, gl::LINK_STATUS, &mut err);
        if err != 0 {
            let mut len = 0;
            gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
            gl::GetProgramInfoLog(self.id, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
            buffer.set_len(len as usize);
            return Err(String::from_utf8_unchecked(buffer));
        }
        return Ok(());
    }

    pub fn build(&mut self) -> Result<(), Error> {
        let mut shaders = [&mut self.vertex_shader, &mut self.geometry_shader, &mut self.fragment_shader];
        unsafe {
            for s in shaders.iter_mut() {
                if let Some(shader) = s {
                    shader.compile();
                    gl::AttachShader(self.id, shader.id);
                }
            }
            gl::LinkProgram(self.id);
            if let Err(e) = self.get_link_error() {
                panic!(e); //TODO: handle error
            }
            Ok(())
        }
    }

    pub fn draw(&mut self, _fb: Framebuffer) {
    }
}
