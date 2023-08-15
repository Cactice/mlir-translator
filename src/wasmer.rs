use anyhow::Result;
use std::{
    io::{self, stdout, Read, Write},
    path::Path,
    sync::Arc,
};
use tempfile::TempDir;
use wasmer::{Cranelift, Function, FunctionEnv, FunctionEnvMut, Imports, Instance, Module, Store};
use wasmer_wasix::{
    virtual_fs::{self, FileSystem, RootFileSystemBuilder},
    Pipe, WasiEnv,
};

fn main() -> Result<()> {
    let simple_frag = std::fs::read_to_string("./simpleFrag.mlir")?;

    // Create a Store.
    // Use LLVM compiler with the default settings
    let compiler = Cranelift::default();
    let mut store = Store::new(compiler);

    println!("Compiling module...");

    // serialize when updating wasm
    // let wasm_bytes = std::fs::read("./mlir-translate.wasm")?;
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

    let _current_dir = std::env::current_dir()?;

    // let fs: Arc<dyn FileSystem + Send + Sync> = Arc::new(virtual_fs::mem_fs::FileSystem::default());
    // let s = RootFileSystemBuilder::default().build();
    // s.mount_directory_entries(Path::new("."), &fs, Path::new("."))?;

    let mut wasi_env = WasiEnv::builder("mlir-translate")
        .stdin(Box::new(stdin_reader))
        .stdout(Box::new(stdout_sender))
        .arg("-serialize-spirv")
        .arg("-no-implicit-module")
        .arg("-mlir-disable-threading")
        .finalize(&mut store)?;

    let import_object = import(&wasi_env, &mut store, &module, env)?;
    let instance = Instance::new(&mut store, &module, &import_object)?;
    wasi_env.initialize(&mut store, instance.clone())?;

    println!("initialized");
    writeln!(stdin_sender, "{}", simple_frag)?;
    drop(stdin_sender);

    println!("written");
    let start = instance.exports.get_function("_start")?;
    let _ = start.call(&mut store, &[]);
    println!("called");
    let mut buf = Vec::new();

    println!("reading...");
    drop(instance);
    drop(wasi_env);
    stdout_reader.read_to_end(&mut buf)?;
    println!("read done!");

    let mut handle = io::stdout().lock();
    handle.write_all(&buf)?;

    Ok(())
}

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

fn import(
    wasi_env: &wasmer_wasix::WasiFunctionEnv,
    store: &mut Store,
    module: &Module,
    env: FunctionEnv<()>,
) -> Result<Imports, anyhow::Error> {
    let mut import_object = wasi_env.import_object(store, module)?;
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
    Ok(import_object)
}
