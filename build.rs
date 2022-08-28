extern crate bindgen;
extern crate pkg_config;
use std::{env, path::PathBuf};

fn main() {
    let lib = pkg_config::probe_library("opencv4").unwrap();

    lib.ld_args
        .iter()
        .for_each(|args| {
            args.iter()
                .for_each(|link| println!("cargo-rustc-link-lib={}",link))
        });

    let bindings = bindgen::Builder::default()
        .clang_args(["-I/usr/include/opencv4", "-x", "c++", "-std=c++14"])
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
