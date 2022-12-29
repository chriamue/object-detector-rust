//! Struct for extracting HOG features from images.
//!
//! # Examples
//!
//! ```
//! use image::DynamicImage;
//! use object_detector_rust::feature::{Feature, HOGFeature};
//!
//! let image = DynamicImage::new_luma8(128, 128);
//! let hog_feature = HOGFeature::default();
//! let features = hog_feature.extract(&image).unwrap();
//! ```

use image::DynamicImage;
use imageproc::hog::{hog, HogOptions};

use super::Feature;

/// Struct for extracting HOG features from images.
#[derive(Debug)]
pub struct HOGFeature {
    options: HogOptions,
}

impl HOGFeature {
    /// Creates a new HOG feature extractor
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `HogFeature` with the given `HogOptions`.
    ///
    /// # Example
    ///
    /// ```
    /// use imageproc::hog::HogOptions;
    /// use object_detector_rust::feature::{Feature, HOGFeature};
    ///
    /// let hog_options = HogOptions {
    ///     orientations: 9,
    ///     signed: true,
    ///     cell_side: 8,
    ///     block_side: 2,
    ///     block_stride: 1,
    /// };
    /// let hog_feature = HOGFeature::new_with_options(hog_options);
    /// ```
    ///
    /// In this example, a `HogFeature` is created with 9 gradient orientation bins,
    /// treating gradients in opposite directions as equal,
    /// and using 8x8 pixel cells and 2x2 cell blocks with a stride of 1 cell.
    ///
    pub fn new_with_options(options: HogOptions) -> Self {
        Self { options }
    }

    /// Creates a new HOG feature extractor with default options
    pub fn default() -> Self {
        let default_options = HogOptions {
            orientations: 9,
            signed: false,
            cell_side: 8,
            block_side: 2,
            block_stride: 1,
        };
        Self {
            options: default_options,
        }
    }
}

impl Feature for HOGFeature {
    fn extract(&self, image: &DynamicImage) -> Result<Vec<f32>, String> {
        let gray_image = image
            .as_luma8()
            .ok_or("Error converting image to grayscale")?;
        Ok(hog(&gray_image, self.options).map_err(|e| e.to_string())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hog_feature() {
        // Define the input image for the test
        let image = image::DynamicImage::new_luma8(64, 64);

        // Create an instance of the HOG feature with default options
        let hog_feature = HOGFeature::new();

        // Call the extract method on the HOG feature
        let hog_descriptor = hog_feature.extract(&image).unwrap();

        // Assert that the output of the extract method is correct
        assert_eq!(hog_descriptor.iter().sum::<f32>(), 0.0);
    }

    #[test]
    fn test_hog_feature_with_options() {
        // Define the input image for the test
        let image = image::DynamicImage::new_luma8(64, 64);

        let hog_options = HogOptions {
            orientations: 9,
            signed: true,
            cell_side: 8,
            block_side: 2,
            block_stride: 1,
        };
        let hog_feature = HOGFeature::new_with_options(hog_options);

        // Call the extract method on the HOG feature
        let hog_descriptor = hog_feature.extract(&image).unwrap();

        // Assert that the output of the extract method is correct
        assert_eq!(hog_descriptor.iter().sum::<f32>(), 0.0);
    }
}
