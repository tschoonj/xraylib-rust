extern crate bindgen;
extern crate cc;
#[cfg(not(feature = "vendored"))]
extern crate pkg_config;
#[cfg(feature = "vendored")]
extern crate xraylib_src;

use std::{env, path::PathBuf};

#[cfg(feature = "vendored")]
fn find_xraylib_vendored() -> (Vec<PathBuf>, Vec<PathBuf>) {
    let artifacts = xraylib_src::Build::new().build();
    println!("cargo:rustc-link-lib=static={}", artifacts.lib());
    (
        vec![artifacts.lib_dir().to_path_buf()],
        vec![artifacts.include_dir().to_path_buf()],
    )
}

#[cfg(not(feature = "vendored"))]
fn find_xraylib_pkg_config() -> (Vec<PathBuf>, Vec<PathBuf>) {
    let library = match pkg_config::Config::new()
        .print_system_libs(false)
        .atleast_version("4.1.2")
        .find("libxrl")
    {
        Ok(lib) => lib,
        Err(e) => {
            panic!("run pkg_config fail: {:?}", e);
        }
    };

    for lib in library.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    (library.link_paths, library.include_paths)
}

fn find_xraylib() -> (Vec<PathBuf>, Vec<PathBuf>) {
    #[cfg(feature = "vendored")]
    {
        find_xraylib_vendored()
    }
    #[cfg(not(feature = "vendored"))]
    {
        find_xraylib_pkg_config()
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let (lib_paths, include_paths) = find_xraylib();

    for lib_path in lib_paths.iter() {
        println!("cargo:rustc-link-search=native={}", lib_path.display());
    }

    for include_path in include_paths.iter() {
        println!("cargo:rerun-if-changed={}", include_path.display());
    }

    let mut builder = bindgen::Builder::default()
        .header_contents("wrapper.h", "#include <xraylib.h>")
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .blocklist_file("stddef.h")
        .blocklist_item("PI")
        .blocklist_item("TWOPI")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    for include_path in include_paths.iter() {
        builder = builder
            .clang_arg("-I")
            .clang_arg(include_path.display().to_string());
    }

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    builder
        .generate()
        .unwrap()
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
