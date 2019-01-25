extern crate gl;

use std::io::Read;
use std::io::Error;
use std::ffi::CString;
use gl::types::*;

#[derive(Copy, Clone, Debug)]
pub enum ShaderStage {
    Vertex,
    Geometry,
    Fragment,
}

impl ShaderStage {
    pub fn gl_enum(&self) -> GLuint {
        match self {
            ShaderStage::Vertex => gl::VERTEX_SHADER,
            ShaderStage::Geometry => gl::GEOMETRY_SHADER,
            ShaderStage::Fragment => gl::FRAGMENT_SHADER,
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
            dbg!("drop shader");
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
            dbg!(id);

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
        let mut len: GLint = -1;
        gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
        if len > 0 {
            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
            gl::GetShaderInfoLog(self.id, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
            buffer.set_len(len as usize);
            let err_str = String::from_utf8_unchecked(buffer);
            dbg!(&err_str);
            if err == gl::FALSE as GLint {
                return Err(err_str);
            } else {
                dbg!(err_str);
            }
        }

        return Ok(());
    }

    pub unsafe fn compile(&mut self) -> Result<(), String> {
        if let Some(src) = &self.source {
            let shader_source = CString::new(src.as_bytes()).unwrap();

            #[cfg(target_os = "emscripten")]
            let define = CString::new("#version 300 es\nprecision highp float;\n").unwrap();

            #[cfg(not(target_os = "emscripten"))]
            let define = CString::new("#version 330\n").unwrap();

            let srcs = [define.as_ptr(), shader_source.as_ptr()];
            gl::ShaderSource(self.id, 2, srcs.as_ptr(), std::ptr::null());
            gl::CompileShader(self.id);
            dbg!(self.id);
            if let Err(e) = self.get_compiler_error() {
                panic!(e); //TODO: handle error
            }
            return Ok(());
        }
        Err("Shader must have source to compile".to_string())
    }
}
