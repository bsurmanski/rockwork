use rockwork::context::Context;
use rockwork::mesh::Mesh;
use rockwork::program::Program;
use rockwork::texture::Texture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::io::Cursor;
use std::io::Error;
use std::time::Duration;

pub struct GameData {
    program: Program,
    quad: Mesh,
    tex: Texture,
}

static mut GAME_DATA: Option<GameData> = None;

fn tick(ctx: &mut Context, dt: Duration) {
    ctx.window().clear();

    let mut gd = unsafe { GAME_DATA.as_mut().unwrap() };
    gd.program.bind_texture(&gd.tex, "tex".to_string());
    gd.program.draw(&gd.quad);

    ctx.swap_buffers();

    for event in ctx.sdl_event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), String> {
    let mut ctx: Context = Context::new();
    ctx.open_window("Draw".to_string(), 320, 240);

    let mut prog = Program::new("Simple".to_string());
    prog.add_vertex_shader(&mut Cursor::new(
        include_bytes!("../assets/deferred.vs").as_ref(),
    ))
    .unwrap();
    prog.add_fragment_shader(&mut Cursor::new(
        include_bytes!("../assets/deferred.fs").as_ref(),
    ))
    .unwrap();
    prog.build().unwrap();

    let quad = Mesh::from_mdl(&mut Cursor::new(
        include_bytes!("../assets/unit_quad.mdl").as_ref(),
    ))
    .unwrap();
    dbg!(unsafe { gl::GetError() });
    dbg!(quad.nelems);

    let mut img = image::load(
        &mut Cursor::new(include_bytes!("../assets/smile.png").as_ref()),
        image::ImageFormat::PNG,
    )
    .unwrap();
    let tex = Texture::new_rgba_from_image(&mut img);

    unsafe {
        GAME_DATA = Some(GameData {
            program: prog,
            quad: quad,
            tex: tex,
        })
    };

    ctx.run(&mut tick);
    Ok(())
}
