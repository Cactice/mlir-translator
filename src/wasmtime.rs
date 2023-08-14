use std::{
    fs,
    io::{self, Write},
};

use anyhow::{Ok, Result};
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmer_wasi::WasiStateBuilder;
use wasmtime::*;
use wasmtime_wasi::{file::File, sync::WasiCtxBuilder, Dir, WasiFile};

fn main() -> Result<()> {
    let simple_frag = std::fs::read_to_string("./simpleFrag.mlir")?;
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let file = fs::File::open(".")?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let mut wasi = WasiCtxBuilder::new()
        // .preopened_dir(Dir::from_std_file(file), "/")?
        .arg("-serialize-spirv")?
        .arg("-no-implicit-module")?
        .inherit_stdout()
        .build();
    let stdin = ReadPipe::from(simple_frag);
    let stdout = Box::new(WritePipe::new_in_memory());

    wasi.push_arg("-serialize-spirv")?;
    wasi.push_arg("-no-implicit-module")?;
    wasi.push_arg("-mlir-disable-threading")?;
    wasi.set_stdin(Box::new(stdin));
    wasi.set_stdout(stdout.clone());

    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, "./mlir-translate.wasm")?;
    imports(&mut linker, &mut store, module)?;

    let _ = linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ());
    drop(store);
    let contents: Vec<u8> = stdout
        .try_into_inner()
        .expect("sole remaining reference to WritePipe")
        .into_inner();

    // println!("contents of stdout: {:?}", unsafe {
    //     String::from_utf8_lossy(&contents)
    // });

    Ok(())
}

fn imports(
    linker: &mut Linker<wasi_common::WasiCtx>,
    store: &mut Store<wasi_common::WasiCtx>,
    module: Module,
) -> Result<(), Error> {
    let ty1 = FuncType::new([ValType::I32], [ValType::I32]);
    let ty2 = FuncType::new([ValType::I32, ValType::I32], [ValType::I32]);
    let ty3 = FuncType::new([ValType::I32, ValType::I32, ValType::I32], [ValType::I32]);
    let ty4 = FuncType::new(
        [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
        [ValType::I32],
    );
    linker
        .func_new(
            "env",
            "__syscall_faccessat",
            ty4.clone(),
            |_store, _input, output| {
                output[0] = Val::I32(0);
                Ok(())
            },
        )?
        .func_new(
            "env",
            "__syscall_chdir",
            ty1.clone(),
            |_store, _input, output| {
                output[0] = Val::I32(0);
                Ok(())
            },
        )?
        .func_new("env", "__syscall_getcwd", ty2, |_store, _input, output| {
            output[0] = Val::I32(0);
            Ok(())
        })?
        .func_new(
            "env",
            "__syscall_getdents64",
            ty3.clone(),
            |_store, _input, output| {
                output[0] = Val::I32(0);
                Ok(())
            },
        )?
        .func_new(
            "env",
            "__syscall_readlinkat",
            ty4,
            |_store, _input, output| {
                output[0] = Val::I32(0);
                Ok(())
            },
        )?
        .func_new(
            "env",
            "__syscall_unlinkat",
            ty3.clone(),
            |_store, _input, output| {
                output[0] = Val::I32(0);
                Ok(())
            },
        )?
        .func_new("env", "__syscall_rmdir", ty1, |_store, _input, output| {
            output[0] = Val::I32(0);
            Ok(())
        })?
        .func_new(
            "env",
            "__syscall_statfs64",
            ty3.clone(),
            |_store, _input, output| {
                output[0] = Val::I32(0);
                Ok(())
            },
        )?
        .module(store, "", &module)?;
    Ok(())
}
