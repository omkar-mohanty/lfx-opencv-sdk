# LFX-Opencv pretest
## Working of the demo
This demo uses the rust bindgen tool to generate bindings for `imread(std::string,int)` function in Opencv4.
```rust
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
```

## Requirements
- Rust
- Opencv4
- Rust Bindgen tool

## Build
1. Install Rust
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Install [Opencv4](https://docs.opencv.org/4.x/d7/d9f/tutorial_linux_install.html)
3. Install rust bindgen tool
```shell
cargo install bindgen
```
## Running tests
```shell
cargo test test_wrapper
```
## Running the demo
The demo reads an image from the crate folder.
```shell
cargo run --example show
```

