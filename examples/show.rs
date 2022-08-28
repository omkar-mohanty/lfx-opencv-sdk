use std::path::PathBuf;

use opencv_rs::imread_wrapper;
fn main() {
    let path = PathBuf::from("./catto.jpg");
    imread_wrapper(path, 1).unwrap();
}
