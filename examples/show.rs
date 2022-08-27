use cxx::let_cxx_string;
use opencv_rs::ffi::imread_cpp;
fn main() {
    let_cxx_string!(s="./");
    imread_cpp(&s, 1);
}
