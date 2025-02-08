#[no_mangle]
pub extern "C" fn __name() -> *const u8 {
    b"test_plug\0".as_ptr()
}

extern "C" {
    fn alloc_host(name_ptr: u32, source_ptr: u32, source_size: u32);
    /// Will return -1 if name doesn't exist in data
    fn get_host(name_ptr: u32, dest_ptr: u32) -> i32;

}

static mut STATE_OFFSET: u32 = 0;

struct State {
    n: i32,
    // Something like the line below wouldn't work because alloc_host will only store the contents of State and the pointer to the other data will be invalidated upon a reset
    // some_ref: &Data
    // You would need to store the other data seperately and then update the reference in this state to point to the new valid reference to your data during initialization
}

const STATE_NAME: *const u8 = b"state\0".as_ptr();

// safe wrapper around extern print
fn print(s: impl AsRef<str>) {
    extern "C" {
        fn print(str_ptr: u32);
    }
    let mut buf = String::new();
    buf.push_str(s.as_ref());
    buf.push('\n');
    buf.push('\0');
    unsafe {
        print(buf.as_ptr() as u32);
    }
}

#[no_mangle]
pub extern "C" fn __init() {
    print("[test_plug.init] Initializing...");
    let data = Box::new([0u8; size_of::<State>()]);
    let data_ptr = Box::leak(data) as *mut u8;
    let name = STATE_NAME as u32;
    if unsafe { get_host(name, data_ptr as u32) } < 0 {
        let data = unsafe { (data_ptr as *mut State).as_mut().unwrap() };
        data.n = 0;
    }
    unsafe {
        STATE_OFFSET = data_ptr as u32;
    }
    print("[test_plug.init] Initialization complete!");
}

#[no_mangle]
pub extern "C" fn run() {
    let state = unsafe { &mut *(STATE_OFFSET as *mut State) };
    print(format!("[test_plug.run] state.n = {}", state.n));
    print("[test_plug.run] Mutating state...");
    state.n += 10;
    print(format!(
        "[test_plug.run] New value of state.n = {}",
        state.n
    ));
}

#[no_mangle]
pub extern "C" fn __reset() {
    print("[test_plug.reset] Storing `state`");
    unsafe {
        alloc_host(STATE_NAME as u32, STATE_OFFSET, size_of::<State>() as u32);
    }
    print("[test_plug.reset] Storage complete");
}
