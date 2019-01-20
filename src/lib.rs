pub mod context;
pub mod window;
pub mod draw_device;
pub mod program;
pub mod shader;
pub mod texture;
pub mod framebuffer;
pub mod mesh;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
