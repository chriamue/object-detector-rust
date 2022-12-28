# Object Detector Rust 🔍

Object Detector Rust is a framework for detecting objects in images using feature-based algorithms. It includes implementations of the HOG and ORB algorithms, as well as functions for feature matching and object localization. The framework is written in Rust, a statically-typed, memory-safe language that can be compiled to WebAssembly for use in web browsers or other environments.

## Features 💪

- Object detection using HOG and ORB feature-based algorithms
- Feature matching and object localization functions
- Written in Rust, compiled to WebAssembly

## Usage 📖

To use Object Detector Rust, add it as a dependency to your project's `Cargo.toml` file:

```toml
[dependencies]
object-detector-rust = "0.1"
```

Then, use the crate in your Rust code:

```rust
extern crate object_detector_rust;

use object_detector_rust::{HOGDetector, ORBDetector};

fn main() {
    let hog_detector = HOGDetector::new();
    let orb_detector = ORBDetector::new();
}
```

## Building and Testing 🛠️

To build the Object Detector Rust crate, run `cargo build` from the command line. To run the unit tests, use `cargo test`.

## ORB Support 🚧

ORB support is currently planned, but not yet implemented. Stay tuned for updates!

## Contributing 🙏

We welcome contributions to Object Detector Rust! If you have a bug fix or feature you would like to add, please open a pull request.

## License 📜

Object Detector Rust is licensed under the [MIT License](LICENSE).
