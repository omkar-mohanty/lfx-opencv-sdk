#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use root::cv::{imread, Mat};
use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Debug)]
struct CvError {
    message: String,
}

impl CvError {
    pub fn new(message: &str) -> Self {
        CvError {
            message: String::from(message),
        }
    }
}

impl Display for CvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CvError {}

/// Wrapper function for imread(std::string, int) to make it safe to use in rust memory model
pub fn imread_wrapper(path: PathBuf, flag: i32) -> Result<Mat, Box<dyn Error>> {
    if !path.exists() {
        return Err(Box::new(CvError::new("Image Does not exist")));
    }

    let mat: Mat;

    let path = path.to_str().unwrap().as_ptr() as *const [u64; 4usize];

    unsafe {
        mat = imread(path, flag);
    }

    Ok(mat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapper_empty() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::new();

        if let Err(_) = imread_wrapper(path, 1) {
            return Ok(());
        }

        panic!("Empty path must be handled")
    }

    #[test]
    fn test_wrapper() -> Result<(), Box<dyn Error>> {
        let path = std::env::current_dir().unwrap();
        println!("{}", path.to_str().unwrap());
        let path = PathBuf::from("./catto.jpg");

        if let Err(msg) = imread_wrapper(path, 1) {
            return Err(msg)
        }

        Ok(())
    }
}
