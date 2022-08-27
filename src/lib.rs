#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::path::PathBuf;
use root::cv::{imread, Mat};
pub fn imread_wrapper(path :PathBuf) -> Mat {
    let mat: Mat;

    let path = path.to_str().unwrap().as_ptr() as *const [u64;4usize];
    unsafe {
        mat = imread(path, 1);
    }

    mat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapper() {
        let path = PathBuf::new();
        imread_wrapper(path);
    }
}
