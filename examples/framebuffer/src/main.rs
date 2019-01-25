use rockwork::context::Context;
use rockwork::mesh::Mesh;
use rockwork::program::Program;
use rockwork::texture::Texture;
use rockwork::framebuffer::Framebuffer;
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
    fb: Framebuffer,
    color_tex: Texture,
    light_tex: Texture,
}

static mut GAME_DATA: Option<GameData> = None;

fn tick(ctx: &mut Context, _dt: Duration) {
    ctx.window().clear();

    let gd = unsafe { GAME_DATA.as_mut().unwrap() };

    gd.program.bind_texture(&gd.color_tex, 0, "tex".to_string());
    gd.program.draw(&gd.quad);

    ctx.swap_buffers();

    dbg!(unsafe { gl::GetError() });

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
    let w = 320;
    let h = 240;
    ctx.open_window("Draw".to_string(), w, h);

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

    let mut img = image::load(
        &mut Cursor::new(include_bytes!("../assets/smile.png").as_ref()),
        image::ImageFormat::PNG,
    )
    .unwrap();
    dbg!(unsafe { gl::GetError() });
    let tex = Texture::new_rgba_from_image(&mut img);
    dbg!(unsafe { gl::GetError() });

    let mut fb = Framebuffer::new();
    dbg!(unsafe { gl::GetError() });
    let color_tex = Texture::new_rgba(w, h);
    dbg!(unsafe { gl::GetError() });
    let light_tex = Texture::new_rgba(w, h);
    fb.add_target(&color_tex);
    fb.add_target(&light_tex);

    dbg!(unsafe { gl::GetError() });
    prog.bind_texture(&tex, 0, "tex".to_string());
    fb.bind();
    prog.draw(&quad);
    Framebuffer::unbind();

    dbg!(unsafe { gl::GetError() });
    unsafe {
        GAME_DATA = Some(GameData {
            program: prog,
            quad: quad,
            tex: tex,
            fb: fb,
            color_tex: color_tex,
            light_tex: light_tex,
        })
    };

    ctx.run(&mut tick);
    Ok(())
}
