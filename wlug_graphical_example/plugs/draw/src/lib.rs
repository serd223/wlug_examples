#[no_mangle]
pub extern "C" fn __name() -> *const u8 {
    b"draw\0".as_ptr()
}

static mut STATE_OFFSET: u32 = 0;

struct State {
    n: i32,
}

#[no_mangle]
pub extern "C" fn __init() {
    let state = Box::new(State { n: 250 });
    let state_ref = Box::leak(state) as *mut State;
    unsafe {
        STATE_OFFSET = state_ref as u32;
    }
}

extern "C" {
    fn draw_rect(x: i32, y: i32, w: i32, h: i32);
}

#[no_mangle]
pub unsafe extern "C" fn draw_run() {
    let state = &mut *(STATE_OFFSET as *mut State);
    draw_rect(60 + ((state.n / 10) % 50), 80, 100, 50);
    state.n += 1;
}
