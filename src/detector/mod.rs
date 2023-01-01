//! A trait for object detectors
//!
//! An object detector is a machine learning model that is able to identify objects within an image.
//! This trait defines the common methods that an object detector should have.

use crate::prelude::Detection;
use image::DynamicImage;
use std::error::Error;
use std::io::{Read, Write};

#[cfg(feature = "brief")]
mod brief_svm_detector;
#[cfg(feature = "brief")]
pub use brief_svm_detector::BriefSVMDetector;

#[cfg(feature = "hog")]
mod hog_svm_detector;
#[cfg(feature = "hog")]
pub use hog_svm_detector::HOGSVMDetector;

/// Trait for object detection
pub trait Detector {
    /// Detects objects in an image
    fn detect(&self, image: &DynamicImage) -> Vec<Detection>;
}

/// Trait for objects that can be persisted to and loaded from storage
pub trait PersistentDetector: Detector {
    /// Loads the detector from the provided reader.
    ///
    /// # Arguments
    ///
    /// * `reader` - The reader to load the detector from.
    ///
    /// # Returns
    ///
    /// A `Result` containing an error string if loading fails, or `Ok` if loading succeeds.
    fn load<R: Read>(&mut self, reader: R) -> Result<(), Box<dyn Error>>;

    /// Saves the detector to the provided writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The writer to save the detector to.
    ///
    /// # Returns
    ///
    /// A `Result` containing an error string if saving fails, or `Ok` if saving succeeds.
    fn save<W: Write>(&self, writer: W) -> Result<(), Box<dyn Error>>;
}
