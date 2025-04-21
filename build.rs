use std::{env, path::PathBuf};

fn main() {
    // CMake is responsible for building the static library, but since
    // Cargo is now responsible for linking, we must tell cargo all libs
    // that CMake is using to build the lib
    let dst = cmake::Config::new("add-cpp")
        .build();

    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=static=add_static");

    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=stdc++");
    } else {
        // just ignore other archs for now, since this is just a dummy test
        panic!("Building on other targets not supported");
    }

    let bindings = bindgen::Builder::default()
        .header("add-cpp/src/add.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Couldn't generate bindings");

    let out_path = PathBuf::from("src/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings");
}