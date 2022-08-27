
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("opencv_rs/cv.hpp");

        type Mat;

        fn imread(path: &CxxString,flags: i32) -> UniquePtr<Mat>;
    }
}
