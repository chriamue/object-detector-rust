//! This module contains traits and structs for feature descriptors that can be extracted from images.
//!
//! # Examples
//!
//! ```
//! use object_detector_rust::feature::{DummyFeature, Feature};
//! use image::{DynamicImage, RgbImage};
//!
//! let image = DynamicImage::ImageRgb8(RgbImage::new(16, 16));
//! let feature = DummyFeature::new(123.0);
//! let descriptor = feature.extract(&image).unwrap();
//!
//! assert_eq!(descriptor.len(), 1);
//! assert_eq!(descriptor[0], 123.0);
//! ```
//!
//! This example shows how to use the `DummyFeature` struct, which is a simple implementation of the `Feature` trait, to extract a feature descriptor from an image.

use image::DynamicImage;
use std::result::Result;

mod hog_feature;
pub use hog_feature::HOGFeature;

mod brief_feature;
pub use brief_feature::BriefFeature;

/// Trait for feature descriptors that can be extracted from images
pub trait Feature {
    /// Extracts the feature descriptor from the given image
    fn extract(&self, image: &DynamicImage) -> Result<Vec<f32>, String>;
}

/// Dummy feature descriptor that always returns the same value
pub struct DummyFeature {
    value: f32,
}

impl DummyFeature {
    /// Creates a new dummy feature descriptor with the given value
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl Feature for DummyFeature {
    fn extract(&self, _image: &DynamicImage) -> Result<Vec<f32>, String> {
        Ok(vec![self.value])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;

    #[test]
    fn test_dummy_feature() {
        let image = DynamicImage::ImageRgb8(RgbImage::new(16, 16));
        let feature = DummyFeature::new(123.0);
        let descriptor = feature.extract(&image).unwrap();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0], 123.0);
    }
}
