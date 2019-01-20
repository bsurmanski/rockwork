use rockwork::context::Context;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn tick(ctx: &mut Context) {
    ctx.window().clear();
    ctx.swap_buffers();

    for event in ctx.sdl_event_pump.poll_iter() {
        match event {
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0);
            },
            _ => {}
        }
    }
}

fn main() {
    let mut ctx = Context::new();
    ctx.open_window("Hello".to_string(), 320, 240);
    
    ctx.run(&mut tick);
    //emscripten::set_main_loop_callback(tick, 60, true);
}
