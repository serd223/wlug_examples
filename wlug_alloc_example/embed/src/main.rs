use std::{collections::HashMap, io::Write};

use wlug::{PlugContext, Plugs, wasmtime::{self, Caller, Engine}};

struct State {
    data: HashMap<String, Vec<u8>>,
    plug_names: Vec<String>,
}

// This example defines simple host functions to allocate arbitrary persistent memory from inside of plugins
// These functions are `alloc_host` and `get_host`. There is also a `print` function for debugging within the plugins
// The implementations of these functions can be found at the end of this file
fn main() -> wasmtime::Result<()> {
    let engine = Engine::default();
    let mut plugs = Plugs::new(
        &engine,
        State {
            data: HashMap::new(),
            plug_names: Vec::new(),
        },
    );

    plugs.add_host_fn("alloc_host".to_string(), alloc_host);
    plugs.add_host_fn("get_host".to_string(), get_host);
    plugs.add_host_fn("print".to_string(), print);

    println!("[main] First run:\n");
    load_run_plug(&mut plugs, &engine)?;

    println!("\n[main] Resetting plugin:\n");
    println!("---------------");
    plugs.reset()?;
    plugs.state_mut().plug_names.clear();
    println!("---------------");

    println!("\n[main] Second run after reload:\n");
    load_run_plug(&mut plugs, &engine)?;

    plugs.reset()?; // Reset to update state.data
    println!("---------------");
    plugs.state_mut().plug_names.clear();
    println!(
        "\n[main] Contents of `state.data`:\n    {:?}\n",
        plugs.state().data
    );

    Ok(())
}

// Convenience function for easily handling reloading
fn load_run_plug(plugs: &mut Plugs<State>, engine: &Engine) -> wasmtime::Result<()> {
    let plug_name = "test_plug";
    let id = plugs.load("test_plug.wasm", engine)?;
    plugs
        .state_mut()
        .plug_names
        .insert(id, plug_name.to_string());

    let plug_name = "test_plug_c";
    let id = plugs.load("test_plug_c.wasm", engine)?;
    plugs
        .state_mut()
        .plug_names
        .insert(id, plug_name.to_string());

    plugs.link()?;
    println!("[main.load_run_plug] Plugs::init start");
    println!("---------------");
    plugs.init()?;
    println!("---------------");
    println!("[main.load_run_plug] Plugs::init end");
    println!("---------------");
    plugs.call::<_, ()>("test_plug", "run", ())?;
    println!("---------------");
    plugs.call::<_, ()>("test_plug_c", "run", ())?;
    println!("---------------");

    Ok(())
}

fn alloc_host(
    mut c: Caller<'_, PlugContext<State>>,
    name_ptr: u32,
    source_ptr: u32,
    source_size: u32,
) {
    let memory_export = c.get_export("memory").unwrap().into_memory().unwrap();

    let mut data = Vec::new();
    let mut name = {
        let PlugContext(id, state) = c.data();
        state.plug_names[*id].clone()
    };
    let memory = memory_export.data(&mut c);
    push_wasm_str_into(&mut name, memory, name_ptr);

    for i in 0..source_size {
        data.push(memory[(source_ptr + i) as usize]);
    }

    let PlugContext(_, state) = c.data_mut();
    state.data.insert(name, data);
}

/// Will return -1 if name doesn't exist in data
fn get_host(mut c: Caller<'_, PlugContext<State>>, name_ptr: u32, dest_ptr: u32) -> i32 {
    let memory_export = c.get_export("memory").unwrap().into_memory().unwrap();
    let mut name = {
        let PlugContext(id, state) = c.data();
        if let Some(name) = state.plug_names.get(*id) {
            name.clone()
        } else {
            return -1;
        }
    };
    let memory = memory_export.data(&mut c);
    push_wasm_str_into(&mut name, memory, name_ptr);

    let PlugContext(_, state) = c.data_mut();
    if let Some(data) = state.data.get(&name) {
        let data = data.clone();
        let memory = memory_export.data_mut(&mut c);
        for i in 0..data.len() {
            memory[dest_ptr as usize + i] = data[i];
        }
        1
    } else {
        -1
    }
}

fn print(mut c: Caller<'_, PlugContext<State>>, str_ptr: u32) {
    let memory = c.get_export("memory").unwrap().into_memory().unwrap();
    let memory = memory.data(&mut c);

    let buf = read_wasm_str(memory, str_ptr);
    print!("{buf}");
    std::io::stdout().flush().unwrap();
}

fn push_wasm_str_into(buf: &mut String, memory: &[u8], offset: u32) {
    let mut offset = offset as usize;
    let mut c = memory[offset] as char;
    while c != '\0' {
        buf.push(c);
        offset += 1;
        c = memory[offset] as char;
    }
}

fn read_wasm_str(memory: &[u8], offset: u32) -> String {
    let mut buf = String::new();
    push_wasm_str_into(&mut buf, memory, offset);
    buf
}
