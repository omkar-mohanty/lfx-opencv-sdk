extern crate bindgen;
extern crate pkg_config;

use std::{env, path::PathBuf};

fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("./cv.hpp")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-I/usr/include/opencv4")
        .compile("opencv_rs");
//    println!("cargo:rustc-link-search=/usr/include/opencv4");

    println!("cargo:rerun-if-changed=cv.hpp");

/*    let bindings = bindgen::Builder::default()
        .clang_arg("-I/usr/include/opencv4")
        .clang_arg("-x").clang_arg("c++")
        .clang_arg("-std=c++11")
        .opaque_type("std::string")
        .whitelist_function("imread")
        .header("cv.hpp")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldnt write bindings"); */
}
