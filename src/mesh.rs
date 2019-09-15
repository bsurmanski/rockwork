use gl::types::*;
use std::ffi::c_void;
use std::io::Read;
use std::io::Cursor;

#[macro_export]
macro_rules! include_mdl {
    ($x:literal) => {
        Mesh::from_mdl(&mut std::io::Cursor::new(include_bytes!($x).as_ref())).unwrap()
    };
}

#[derive(Default, Debug)]
#[repr(C)]
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

#[derive(Default, Debug)]
#[repr(C)]
pub struct Face {
    vertex_ids: [u16; 3],
}

#[repr(C)]
pub struct Edge {
    vertex_ids: [u16; 2],
    face_ids: [u16; 2],
    first_edges: [u16; 2],
    second_edges: [u16; 2],
}

#[derive(Default, Debug)]
#[repr(C)]
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
    vao: GLuint,
    pub nelems: usize,
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            dbg!("drop mesh");
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ibo);
        }
    }
}

fn read_into<T>(f: &mut Read, s: &mut T) -> std::io::Result<()> {
    unsafe {
        let slice = std::slice::from_raw_parts_mut(
            (s as *mut T) as *mut u8,
            std::mem::size_of::<T>());
        f.read_exact(slice)?;
        Ok(())
    }
}

fn read_into_slice<T>(f: &mut Read, s: &mut [T]) -> std::io::Result<()> where T: std::fmt::Debug{
    unsafe {
        let slice = std::slice::from_raw_parts_mut(
            (s.as_ptr() as *mut T) as *mut u8,
            std::mem::size_of::<T>() * s.len());
        f.read_exact(slice)?;
        Ok(())
    }
}

impl Mesh {
    pub fn new() -> Self {
        let mut vbo: GLuint = 0;
        let mut ibo: GLuint = 0;
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ibo);
            gl::GenVertexArrays(1, &mut vao);
        }

        Mesh {
            vbo: vbo,
            ibo: ibo,
            vao: vao,
            nelems: 0,
        }
    }

    pub fn from_mdl(f: &mut Read) -> Result<Self, String> {
        let mut header: MdlHeader = Default::default();
        read_into(f, &mut header).unwrap();

        if String::from_utf8_lossy(&header.magic[0..3]) != "MDL" {
            return Err(format!("Bad header in .mdl file: {:?}", String::from_utf8_lossy(&header.magic[0..2])));
        }

        let mut verts: Vec<Vertex> = Vec::with_capacity(header.nverts as usize);
        let mut faces: Vec<Face> = Vec::with_capacity(header.nfaces as usize);
        unsafe {
            verts.set_len(header.nverts as usize);
            faces.set_len(header.nfaces as usize);
            read_into_slice(f, verts.as_mut_slice()).unwrap();
            read_into_slice(f, faces.as_mut_slice()).unwrap();
        }

        let mut mesh = Self::new();
        mesh.upload_vertex_data(verts);
        mesh.upload_face_data(faces);
        Ok(mesh)
    }

    pub fn upload_vertex_data(&mut self, verts: Vec<Vertex>) {
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 
                           (std::mem::size_of::<Vertex>() * verts.len()) as GLsizeiptr,
                           verts.as_ptr() as *const GLvoid,
                           gl::STATIC_DRAW);

            // pos
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 32, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            // norm
            gl::VertexAttribPointer(1, 3, gl::SHORT, gl::TRUE, 32, 12 as *const _);
            gl::EnableVertexAttribArray(1);
            // uv
            gl::VertexAttribPointer(2, 2, gl::UNSIGNED_SHORT, gl::TRUE, 32, 18 as *const _);
            gl::EnableVertexAttribArray(2);
            // color
            gl::VertexAttribPointer(3, 3, gl::UNSIGNED_BYTE, gl::TRUE, 32, 22 as *const _);
            gl::EnableVertexAttribArray(3);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        }
    }

    pub fn upload_face_data(&mut self, faces: Vec<Face>) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 
                           (std::mem::size_of::<Face>() * faces.len()) as GLsizeiptr,
                           faces.as_ptr() as *const GLvoid,
                           gl::STATIC_DRAW);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            self.nelems = faces.len() * 3;
        }
    }

    pub fn bind(&self) {
        unsafe {
            //gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.nelems as i32, 
                             gl::UNSIGNED_SHORT, std::ptr::null());
        }
    }
}
