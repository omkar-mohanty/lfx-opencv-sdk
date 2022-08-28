#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use root::cv::{imread, Mat};
use std::fmt::Display;
use std::path::PathBuf;
use std::error::Error;

#[derive(Debug)]
struct CvError{
    message: String
}

impl CvError {
    pub fn new(message: &str) -> Self {
        CvError {
            message: String::from(message)
        }
    }
}

impl Display for CvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CvError{} 



pub fn imread_wrapper(path: PathBuf) -> Result<Mat,Box<dyn Error>> {
    if !path.exists() {
        return Err(Box::new(CvError::new("Image Does not exist"))) 
    }
    
    let mat: Mat;

    let path = path.to_str().unwrap().as_ptr() as *const [u64; 4usize];

    unsafe {
        mat = imread(path, 1);
    }

    Ok(mat)
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
