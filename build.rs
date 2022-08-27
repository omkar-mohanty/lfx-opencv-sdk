extern crate bindgen;
extern crate pkg_config;
extern crate cc;
use std::{env, path::PathBuf};

fn main() {
    cc::Build::new()
        .cpp(true)
        .include("/usr/include/opencv4")
        .file("./cv.hpp")
        .compile("cv.a");
        

    let bindings = bindgen::Builder::default()
        .clang_arg("-I/usr/include/opencv4")
        .clang_arg("-x").clang_arg("c++")
        .clang_arg("-std=c++14")
        .clang_arg("-I./")
        .opaque_type("std::.*")
        .enable_cxx_namespaces()
        .whitelist_type("cv::Mat")
        .whitelist_function("cv::imread")
        .header("cv.hpp")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldnt write bindings"); 
}
