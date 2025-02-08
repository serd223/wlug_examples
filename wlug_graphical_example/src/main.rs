use framebrush::{Canvas, BLUE, RED};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use wasmtime::{Caller, Engine};
use wlug::{PlugContext, Plugs};

const DEFAULT_WIDTH: usize = 800;
const DEFAULT_HEIGHT: usize = 600;

type State = Option<Canvas<u32, Vec<u32>>>;

fn draw_rect(mut c: Caller<'_, PlugContext<State>>, x: i32, y: i32, w: i32, h: i32) {
    let PlugContext(_id, state) = c.data_mut();
    if let Some(canvas) = state {
        let w = w as usize;
        let h = h as usize;
        canvas.rect(x, y, w, h, &RED);
    }
}

fn load_plugs(plugs: &mut Plugs<'_, State>, engine: &Engine) -> wasmtime::Result<()> {
    plugs.load("draw.wasm", engine)?;
    plugs.load("draw_two.wasm", engine)?;
    plugs.link()?;
    plugs.init()?;
    Ok(())
}

fn main() -> wasmtime::Result<()> {
    let mut buf = vec![0; DEFAULT_WIDTH * DEFAULT_HEIGHT];
    let engine = Engine::default();

    let mut plugs = Plugs::new(&engine, None);
    plugs.add_host_fn("draw_rect".to_string(), draw_rect);
    load_plugs(&mut plugs, &engine)?;

    let mut window = Window::new(
        "wlug graphical example",
        DEFAULT_WIDTH,
        DEFAULT_HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap();

    window.set_target_fps(144);
    while window.is_open() {
        let (width, height) = window.get_size();
        buf.resize(width * height, 0);

        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            plugs.reset();
            load_plugs(&mut plugs, &engine)?;
        }

        // Begin drawing
        let mut canvas = Canvas::new(buf, (width, height), (DEFAULT_WIDTH, DEFAULT_HEIGHT));
        canvas.fill(0);

        *plugs.state_mut() = Some(canvas); // Transfer ownership of `canvas` to wasm
        plugs.call::<_, ()>("draw_two", "run_both", ())?;

        canvas = plugs.state_mut().take().unwrap(); // Take back the ownership of `canvas`
        canvas.line(10, 10, 50, 50, &BLUE);
        canvas.line(50, 10, 10, 50, &BLUE);

        buf = canvas.finish(); // Take back ownership of `buf`

        // End drawing
        window.update_with_buffer(&buf, width, height).unwrap();
    }

    Ok(())
}
