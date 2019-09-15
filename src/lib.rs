pub mod context;
pub mod draw_device;
pub mod framebuffer;
pub mod mesh;
pub mod program;
pub mod shader;
pub mod texture;
pub mod window;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
