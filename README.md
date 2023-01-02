# Object Detector Rust 🔍

[![Github Repo](https://img.shields.io/badge/github-repo-green)](https://github.com/chriamue/object-detector-rust/)
[![codecov](https://codecov.io/gh/chriamue/object-detector-rust/branch/main/graph/badge.svg?token=RJ6T5D9DZT)](https://codecov.io/gh/chriamue/object-detector-rust)
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

## Training 🏋️‍♀️

To train a model for object detection, you will need a dataset of labeled images that includes examples of the objects you want to detect. You can use tools such as [Image Label Tool](https://chriamue.github.io/image-label-tool/) to label your images.

Once you have collected and labeled your dataset, you can use image processing and machine learning libraries to extract features from the images and train a model using these features.

To train a model, you will need to:

1. Load and preprocess the images in your dataset. You can use image processing libraries to load the images from files and apply any desired transformations, such as resizing or converting to grayscale.

2. Extract features from the images. You can use image processing libraries to extract features such as HOG, or ORB from the images.

3. Create a model using a machine learning library. You can choose a model that is suitable for object detection, such as an SVM classifier.

4. Train the model on the features using the `fit` method provided by the model.

![Training](https://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.github.com/chriamue/object-detector-rust/main/docs/training.puml)

## Usage 📖

To use the Object Detector Rust library, you will need to add it to your project as a dependency. You can do this by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
object-detector-rust = "0.1"
```

Then, use the crate in your Rust code:

```rust
use object_detector_rust::prelude::*;

fn main() {
}
```

You can then use the library to train and use a classifier or detector on your annotated images.

To train a classifier, you will need to create a `DataSet` object and populate it with annotated images. You can then create a `Classifier` object, such as a `BayesClassifier` or `SVMClassifier`, and pass it the `DataSet` object to train it.

```rust .ignore}
use image::DynamicImage;
use object_detector_rust::prelude::*;
use object_detector_rust::utils::extract_data;
use object_detector_rust::feature::HOGFeature;
// Create a memory-based DataSet
let mut dataset = MemoryDataSet::new();

// Add some annotated images to the DataSet
dataset.add_annotated_image(AnnotatedImage {
    image: DynamicImage::new_rgba8(128, 128),
    annotations: vec![Annotation {
        bbox: BBox { x: 0, y: 0, width: 32, height: 32 },
        class: 0,
    }],
});
dataset.add_annotated_image(AnnotatedImage {
    image: DynamicImage::new_rgba8(128, 128),
    annotations: vec![Annotation {
        bbox: BBox { x: 50, y: 50, width: 32, height: 32 },
        class: 1,
    }],
});
let class = 1;
let feature = HOGFeature::default();
// Create a BayesClassifier and train it on the DataSet
let mut classifier = BayesClassifier::new();
let (x, y) = dataset.get_data();
let x: Vec<Vec<f32>> = x
        .iter()
        .map(|image| feature.extract(image).unwrap())
        .collect();
    let y = y
        .iter()
        .map(|y| if *y == class { true } else { false })
        .collect();
let (x, y) = extract_data(x, y);
classifier.fit(&x.view(), &y.view());
```

To use a classifier to predict the class of an image, you can call the `predict` method on the classifier and pass it the image.

```rust .ignore
let prediction = classifier.predict(&image);
```

To use a detector in Object Detector Rust, you will need to do the following:

1. Create a `Detector` object. This can be done using one of the provided implementations, such as `HOGSVMDetector` or `BriefSVMDetector`, or by creating a custom implementation of the `Detector` trait.
2. Train the `Detector` object on a `DataSet` object. The `DataSet` trait includes methods for adding annotated images and iterating over them.
3. Use the Detector object's detect method to detect objects in images and return their bounding boxes and class predictions.

For example:

```rust .ignore
use object_detector_rust::prelude::{DataSet, Detector, HOGSVMDetector};

// Create a HOGSVMDetector object
let mut detector = HOGSVMDetector::new();

// Create a DataSet object and add some annotated images to it
let mut dataset = MemoryDataSet::new();
dataset.add_annotated_image(annotated_image_1);
dataset.add_annotated_image(annotated_image_2);

// Train the detector on the dataset
detector.train(&dataset);

// Use the detector to detect objects in an image
let detections = detector.detect(&image);
```

## Building and Testing 🛠️

To build the Object Detector Rust crate, run `cargo build` from the command line. To run the unit tests, use `cargo test`.

## Running Benchmarks 🏋️‍♀️

To run the benchmarks for the Object Detector Rust crate, use the cargo bench command. This will run all the benchmarks defined in the benches directory.

You can also run a specific benchmark by specifying its name, like this:

```sh
cargo bench
```

If you want to see the output of the benchmarks as they are running, you can use the --verbose flag:

```sh
cargo bench --verbose
```

## ORB Support 🚧

ORB support is currently planned, but not yet implemented. Stay tuned for updates!

## Contributing 🙏

We welcome contributions to Object Detector Rust! If you have a bug fix or feature you would like to add, please open a pull request.

## License 📜

Object Detector Rust is licensed under the [MIT License](https://github.com/chriamue/object-detector-rust/blob/main/LICENSE).

## Note 📝

This project is a remake of [Hog Detector](https://github.com/chriamue/hog-detector).
It was made with assistance of chatgpt.
Find the dialog [here](https://github.com/chriamue/object-detector-rust/blob/main/transcript.md).
