extern crate gl;

use std::io::Read;
use std::io::Error;
use std::ffi::CString;
use gl::types::*;

use crate::shader::*;
use crate::framebuffer::*;
use crate::mesh::*;
use crate::texture::*;

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
            dbg!("drop program");
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
        let mut err: GLint = -1;
        let mut len = 0;
        gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut err);
        gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
        gl::GetProgramInfoLog(self.id, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
        buffer.set_len(len as usize);
        let err_str = String::from_utf8_unchecked(buffer);
        dbg!(&err_str);
        if err == gl::FALSE as GLint {
            return Err(err_str);
        } else {
            dbg!(err_str);
        }
        return Ok(());
    }

    pub fn build(&mut self) -> Result<(), String> {
        unsafe {
            let mut shaders = [&mut self.vertex_shader, &mut self.geometry_shader, &mut self.fragment_shader];
            for s in shaders.iter_mut() {
                if let Some(shader) = s {
                    shader.compile()?;
                    gl::AttachShader(self.id, shader.id);
                }
            }

            gl::BindAttribLocation(self.id, 0, CString::new("position").unwrap().as_ptr());
            gl::BindAttribLocation(self.id, 1, CString::new("normal").unwrap().as_ptr());
            gl::BindAttribLocation(self.id, 2, CString::new("uv").unwrap().as_ptr());
            gl::BindAttribLocation(self.id, 3, CString::new("color").unwrap().as_ptr());

            gl::LinkProgram(self.id);
            self.get_link_error()?;

            let mut shaders = [&mut self.vertex_shader, &mut self.geometry_shader, &mut self.fragment_shader];
            for s in shaders.iter_mut() {
                if let Some(shader) = s {
                    gl::DetachShader(self.id, shader.id);
                }
            }

            Ok(())
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn bind_texture(&mut self, tex: &Texture, unit: i32, name: String) {
        unsafe {
            let loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::Uniform1i(loc, unit);
            tex.bind(unit as usize);
        }
    }

    pub fn draw(&mut self, mesh: &Mesh) {
        self.bind();
        mesh.bind();
        mesh.draw();
    }
}
