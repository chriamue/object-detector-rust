//! A trait for object detectors
//!
//! An object detector is a machine learning model that is able to identify objects within an image.
//! This trait defines the common methods that an object detector should have.

use image::DynamicImage;

use crate::prelude::Detection;

mod hog_svm_detector;
pub use hog_svm_detector::HOGSVMDetector;

/// Trait for object detection
pub trait Detector {
    /// Detects objects in an image
    fn detect(&self, image: &DynamicImage) -> Vec<Detection>;
}
