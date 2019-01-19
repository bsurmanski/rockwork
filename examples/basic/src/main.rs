use rockwork::context::Context;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let mut ctx = Context::new();
    ctx.open_window("Hello".to_string(), 320, 240);

    let mut event_pump = ctx.sdl_context().event_pump().unwrap();
    'running: loop {
        ctx.window().clear();
        ctx.swap_buffers();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
