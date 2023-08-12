use std::io::Read;

use anyhow::Result;
use wasmer::{Extern, Function, FunctionEnv, FunctionEnvMut, Imports, Instance, Module, Store};
use wasmer_wasix::{Pipe, WasiEnv};

fn ret_0(_env: FunctionEnvMut<()>, _a: i32, _b: i32, _c: i32, _d: i32) -> i32 {
    0
}

fn main() -> Result<()> {
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = std::fs::read("./mlir-translate.wasm")?;

    // Create a Store.
    let mut store = Store::default();

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;
    println!("Done!");

    let (stdout_tx, mut stdout_rx) = Pipe::channel();
    let env = FunctionEnv::new(&mut store, ());
    let env = FunctionEnv::new(&mut store, ());

    let mut import_object = Imports::new();
    import_object.define(
        "env",
        "__syscall_faccessat",
        Function::new_typed_with_env(&mut store, &env, ret_0),
    );
    let mut wasi_env = WasiEnv::builder("hello")
        .stdout(Box::new(stdout_tx))
        .finalize(&mut store)?;
    let instance = Instance::new(&mut store, &module, &import_object)?;
    wasi_env.initialize(&mut store, instance.clone())?;

    eprintln!("Run complete - reading output");

    let start = instance.exports.get_function("_start")?;
    start.call(&mut store, &[])?;

    // eprintln!("Output: {buf}");

    Ok(())
}
