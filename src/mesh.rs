use gl::types::*;
use std::ffi::c_void;
use std::io::Read;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Vertex {
    position: [f32; 3],
    normal: [i16; 3],
    uv: [u16; 2],
    color: [u8; 3],
    material: u8,
    boneid: [u8; 2],
    boneweight: [u8; 2],
    incident_edge_id: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Face {
    vertex_ids: [u16; 3],
    incident_edge_id: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Edge {
    vertex_ids: [u16; 2],
    face_ids: [u16; 2],
    first_edges: [u16; 2],
    second_edges: [u16; 2],
}

#[derive(Serialize, Deserialize)]
pub struct MdlHeader {
    magic: [u8; 3],
    version: u8,
    nverts: u32,
    nfaces: u32,
    nedges: u32,
    nbones: u8,
    name: [u8; 15],
}

pub struct Mesh {
    ibo: GLuint,
    vbo: GLuint,
    nelems: usize,
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ibo);
        }
    }
}

impl Mesh {
    fn new() -> Self {
        let mut vbo: GLuint = 0;
        let mut ibo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ibo);
        }

        Mesh {
            vbo: vbo,
            ibo: ibo,
            nelems: 0,
        }
    }

    fn from_mdl(f: &mut Read) -> Result<Self, String> {
        let header: MdlHeader = bincode::deserialize_from(
            f.take(std::mem::size_of::<MdlHeader>() as u64)).unwrap();

        if String::from_utf8_lossy(&header.magic[0..2]) != "MDL" {
            return Err("Bad header in .mdl file".to_string());
        }

        let verts: Vec<Vertex> = bincode::deserialize_from(
            f.take(std::mem::size_of::<Vertex> as u64 * header.nverts as u64)).unwrap(); 
        let faces: Vec<Face> = bincode::deserialize_from(
            f.take(std::mem::size_of::<Face> as u64 * header.nfaces as u64)).unwrap(); 

        let mut mesh = Self::new();
        mesh.upload_vertex_data(verts);
        mesh.upload_face_data(faces);
        Ok(mesh)
    }

    fn upload_vertex_data(&mut self, verts: Vec<Vertex>) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 
                           (std::mem::size_of::<Vertex>() * verts.len()) as isize,
                           verts.as_ptr() as *const c_void,
                           gl::STATIC_DRAW);
        }
    }

    fn upload_face_data(&mut self, faces: Vec<Face>) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 
                           (std::mem::size_of::<Face>() * faces.len()) as isize,
                           faces.as_ptr() as *const c_void,
                           gl::STATIC_DRAW);
            self.nelems = faces.len() * 3;
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
        }
    }

    fn draw(&self) {
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.nelems as i32, 
                             gl::UNSIGNED_SHORT, std::ptr::null());
        }
    }
}
