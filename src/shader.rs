extern crate gl;

use std::fmt;
use std::io::Read;
use std::io::Error;
use std::error;
use std::ffi::CString;
use gl::types::*;

pub enum ShaderStage {
    Vertex,
    Geometry,
    Fragment,
}

impl ShaderStage {
    pub fn gl_enum(&self) -> GLuint {
        match self {
            Vertex => gl::VERTEX_SHADER,
            Geometry => gl::GEOMETRY_SHADER,
            Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

pub struct Shader {
    pub id: GLuint,
    stage: ShaderStage,
    source: Option<String>,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Shader {
    pub fn new_with_source(stage: ShaderStage, src: &mut Read) -> Result<Shader, Error> {
        unsafe {
            let mut src_str = String::new();
            src.read_to_string(&mut src_str)?;
            let id = gl::CreateShader(stage.gl_enum());

            Ok(Shader {
                id: id,
                source: Some(src_str),
                stage: stage,
            })
        }
    }

    unsafe fn get_compiler_error(&self) -> Result<(), String> {
        let mut err = 0;
        gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut err);
        if err != 0 {
            let mut len = 0;
            gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
            gl::GetShaderInfoLog(self.id, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
            buffer.set_len(len as usize);
            return Err(String::from_utf8_unchecked(buffer));
        }
        return Ok(());
    }

    pub unsafe fn compile(&mut self) -> Result<(), Error> {
        if let Some(src) = &self.source {
            let shader_source = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(self.id, 1, &shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(self.id);
            if let Err(e) = self.get_compiler_error() {
                panic!(e); //TODO: handle error
            }
        }
        Ok(())
    }
}
