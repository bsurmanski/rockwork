extern crate gl;

use gl::types::*;
use nalgebra::{Matrix2, Vector2, Vector4};
use std::ffi::CString;
use std::{error, fmt, io};

use crate::framebuffer::*;
use crate::mesh::*;
use crate::shader::*;
use crate::texture::*;

#[macro_export]
macro_rules! include_simple_program {
    ($name:expr, $vs:literal, $fs:literal) => {
        Program::new_simple_program(
            $name,
            &mut std::io::Cursor::new(include_bytes!($vs).as_ref()),
            &mut std::io::Cursor::new(include_bytes!($fs).as_ref()),
        )
        .unwrap()
    };
}

#[derive(Debug)]
pub enum ProgramError {
    ReadError(io::Error),
    BuildError(String),
    LinkError(String),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program Error")
    }
}

impl error::Error for ProgramError {}

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

    pub fn new_simple_program(
        name: String,
        vs: &mut io::Read,
        fs: &mut io::Read,
    ) -> Result<Program, ProgramError> {
        let mut p = Program::new(name);
        p.add_vertex_shader(vs)?;
        p.add_fragment_shader(fs)?;
        p.build()?;
        return Ok(p);
    }

    pub fn add_vertex_shader(&mut self, src: &mut io::Read) -> Result<(), ProgramError> {
        let shader = Shader::new_with_source(ShaderStage::Vertex, src)
            .map_err(|e| ProgramError::ReadError(e))?;
        self.vertex_shader = Some(shader);
        Ok(())
    }

    pub fn add_geometry_shader(&mut self, src: &mut io::Read) -> Result<(), ProgramError> {
        let shader = Shader::new_with_source(ShaderStage::Geometry, src)
            .map_err(|e| ProgramError::ReadError(e))?;
        self.geometry_shader = Some(shader);
        Ok(())
    }

    pub fn add_fragment_shader(&mut self, src: &mut io::Read) -> Result<(), ProgramError> {
        let shader = Shader::new_with_source(ShaderStage::Fragment, src)
            .map_err(|e| ProgramError::ReadError(e))?;
        self.fragment_shader = Some(shader);
        Ok(())
    }

    unsafe fn get_link_error(&self) -> Result<(), ProgramError> {
        let mut err: GLint = -1;
        let mut len = 0;
        gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut err);
        gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
        gl::GetProgramInfoLog(
            self.id,
            len,
            std::ptr::null_mut(),
            buffer.as_mut_ptr() as *mut i8,
        );
        buffer.set_len(len as usize);
        let err_str = String::from_utf8_unchecked(buffer);
        dbg!(&err_str);
        if err == gl::FALSE as GLint {
            return Err(ProgramError::BuildError(err_str.to_string()));
        } else {
            dbg!(err_str);
        }
        return Ok(());
    }

    pub fn build(&mut self) -> Result<(), ProgramError> {
        unsafe {
            let mut shaders = [
                &mut self.vertex_shader,
                &mut self.geometry_shader,
                &mut self.fragment_shader,
            ];
            for s in shaders.iter_mut() {
                if let Some(shader) = s {
                    shader.compile().map_err(|e| ProgramError::BuildError(e));
                    gl::AttachShader(self.id, shader.id);
                }
            }

            gl::BindAttribLocation(self.id, 0, CString::new("position").unwrap().as_ptr());
            gl::BindAttribLocation(self.id, 1, CString::new("normal").unwrap().as_ptr());
            gl::BindAttribLocation(self.id, 2, CString::new("uv").unwrap().as_ptr());
            gl::BindAttribLocation(self.id, 3, CString::new("color").unwrap().as_ptr());

            gl::LinkProgram(self.id);
            self.get_link_error()?;

            let mut shaders = [
                &mut self.vertex_shader,
                &mut self.geometry_shader,
                &mut self.fragment_shader,
            ];
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

    pub fn bind_texture(&mut self, name: &str, tex: &Texture, unit: i32) {
        unsafe {
            self.bind();
            let loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            if loc >= 0 {
                gl::Uniform1i(loc, unit);
                tex.bind(unit as usize);
            }
        }
    }

    pub fn set_uniform_mat2(&self, name: &str, u: &Matrix2<f32>) {
        unsafe {
            self.bind();
            let loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::UniformMatrix2fv(loc, 1, gl::TRUE, u.as_slice().as_ptr());
        }
    }

    pub fn set_uniform_vec2(&self, name: &str, u: &Vector2<f32>) {
        unsafe {
            self.bind();
            let loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::Uniform2fv(loc, 1, u.as_slice().as_ptr());
        }
    }

    pub fn set_uniform_vec4(&self, name: &str, u: &Vector4<f32>) {
        unsafe {
            self.bind();
            let loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::Uniform4fv(loc, 1, u.as_slice().as_ptr());
        }
    }

    pub fn set_uniform_float(&self, name: &str, u: f32) {
        unsafe {
            self.bind();
            let loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::Uniform1f(loc, u);
        }
    }

    pub fn draw(&mut self, mesh: &Mesh) {
        self.bind();
        mesh.bind();
        mesh.draw();
    }
}
