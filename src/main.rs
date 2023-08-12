use anyhow::Result;
use std::io::{Read, Write};
use wasmer::{
    Extern, Function, FunctionEnv, FunctionEnvMut, Imports, Instance, Module, Store, Value,
};
use wasmer_wasix::{Pipe, WasiEnv};

fn zero_return4(_env: FunctionEnvMut<()>, _a: i32, _b: i32, _c: i32, _d: i32) -> i32 {
    0
}
fn zero_return3(_env: FunctionEnvMut<()>, _a: i32, _b: i32, _c: i32) -> i32 {
    0
}
fn zero_return2(_env: FunctionEnvMut<()>, _a: i32, _b: i32) -> i32 {
    0
}
fn zero_return1(_env: FunctionEnvMut<()>, _a: i32) -> i32 {
    0
}

fn main() -> Result<()> {
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = std::fs::read("./mlir-translate.wasm")?;
    let simple_frag = std::fs::read_to_string("./simpleFrag.mlir")?;

    // Create a Store.
    let mut store = Store::default();

    println!("Compiling module...");
    // Let's compile the Wasm module.
    // let module = Module::new(&store, wasm_bytes)?;
    // module.serialize_to_file("./mlir-translate.wasm-module")?;
    let module = unsafe { Module::deserialize_from_file(&store, "./mlir-translate.wasm-module") }?;

    println!("Done compile");
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = runtime.enter();
    println!("Tokio ready");
    let (mut stdin_sender, stdin_reader) = Pipe::channel();
    let (stdout_sender, mut stdout_reader) = Pipe::channel();
    let env = FunctionEnv::new(&mut store, ());

    writeln!(stdin_sender, "{}", simple_frag)?;
    let mut wasi_env = WasiEnv::builder("hello")
        .stdin(Box::new(stdin_reader))
        .stdout(Box::new(stdout_sender))
        .arg("-serialize-spirv")
        .arg("-no-implicit-module")
        .finalize(&mut store)?;

    let mut import_object = wasi_env.import_object(&mut store, &module)?;
    import_various(&mut import_object, &mut store, env);

    let instance = Instance::new(&mut store, &module, &import_object)?;
    wasi_env.initialize(&mut store, instance.clone())?;

    println!("Run complete - reading output");

    stdin_sender.write(simple_frag.as_bytes())?;
    println!("written, calling");
    let start = instance.exports.get_function("_start")?;
    start.call(&mut store, &[])?;
    println!("called");
    let mut buf = String::new();
    stdout_reader.read_to_string(&mut buf)?;
    println!("Read \"{}\" from the WASI stdout!", buf.trim());

    // eprintln!("Output: {buf}");

    Ok(())
}

fn import_various(import_object: &mut Imports, store: &mut Store, env: FunctionEnv<()>) {
    import_object.define(
        "env",
        "__syscall_faccessat",
        Function::new_typed_with_env(store, &env, zero_return4),
    );
    import_object.define(
        "env",
        "__syscall_chdir",
        Function::new_typed_with_env(store, &env, zero_return1),
    );
    import_object.define(
        "env",
        "__syscall_getcwd",
        Function::new_typed_with_env(store, &env, zero_return2),
    );
    import_object.define(
        "env",
        "__syscall_getdents64",
        Function::new_typed_with_env(store, &env, zero_return3),
    );
    import_object.define(
        "env",
        "__syscall_readlinkat",
        Function::new_typed_with_env(store, &env, zero_return4),
    );
    import_object.define(
        "env",
        "__syscall_unlinkat",
        Function::new_typed_with_env(store, &env, zero_return3),
    );
    import_object.define(
        "env",
        "__syscall_rmdir",
        Function::new_typed_with_env(store, &env, zero_return1),
    );
    import_object.define(
        "env",
        "__syscall_statfs64",
        Function::new_typed_with_env(store, &env, zero_return3),
    );
}
