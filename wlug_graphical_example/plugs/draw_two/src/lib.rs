#[no_mangle]
pub extern "C" fn __name() -> *const u8 {
    b"draw_two\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn __deps() -> *const u8 {
    b"draw\0".as_ptr()
}

extern "C" {
    fn draw_rect(x: i32, y: i32, w: i32, h: i32);
    fn draw_run();
}

#[no_mangle]
pub unsafe extern "C" fn run_both() {
    draw_run();
    let x = 120;
    let y = 80;
    let w = 50;
    let h = 30;
    let pad = 10;
    for i in 1..10 {
        draw_rect(x + pad + w * i, y + pad + h * i, w, h);
    }
}
