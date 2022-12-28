# Object Detector Rust 🔍

[![Github Repo](https://img.shields.io/badge/github-repo-green)](https://github.com/chriamue/object-detector-rust/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Docs](https://img.shields.io/badge/Docs-online-green.svg)](https://chriamue.github.io/object-detector-rust/object_detector_rust/)

Object Detector Rust is a framework for detecting objects in images using feature-based algorithms. It includes implementations of the HOG and ORB algorithms, as well as functions for feature matching and object localization. The framework is written in Rust, a statically-typed, memory-safe language that can be compiled to WebAssembly for use in web browsers or other environments.

## Features 💪

- Object detection using HOG and ORB feature-based algorithms
- Feature matching and object localization functions
- Written in Rust, compiled to WebAssembly

## Architecture 🧱

The overall architecture of the Object Detector Rust system is shown in the following diagram:

![Object Detection Diagram](https://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.github.com/chriamue/object-detector-rust/main/docs/object-detection-diagram.puml)

In this system, the input image is passed to the feature extractor, which extracts features from the image. The extracted features are then matched against features in a template image using a matcher. The matcher passes the locations of the matched features to a localizer, which determines the location of the object in the input image and annotates the image with a bounding box around the object.

## Usage 📖

To use Object Detector Rust, add it as a dependency to your project's `Cargo.toml` file:

```toml
[dependencies]
object-detector-rust = "0.1"
```

Then, use the crate in your Rust code:

```rust
use object_detector_rust::*;

fn main() {
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
