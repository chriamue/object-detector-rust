use image::{DynamicImage, GrayImage};
use imageproc::corners::{corners_fast9, Corner};

use super::Feature;

/// Struct for extracting BRIEF (Binary Robust Independent Elementary Features)
/// from an image.
#[derive(Debug, PartialEq)]
pub struct BriefFeature {
    /// Threshold used for deciding whether a pixel difference is significant
    threshold: u8,
    /// Number of tests to perform for each keypoint
    length: u32,
}

impl Feature for BriefFeature {
    /// Extracts the feature descriptor from the given image
    ///
    /// # Arguments
    ///
    /// * `image` - The image to extract the feature descriptor from.
    ///
    /// # Returns
    ///
    /// A vector of floating point values representing the feature descriptor.
    ///
    /// # Example
    ///
    /// ```
    /// use image::{DynamicImage, GrayImage};
    /// use imageproc::corners::{corners_fast9, Corner};
    /// use object_detector_rust::feature::{Feature, BriefFeature};
    ///
    /// let image = DynamicImage::new_rgb8(100, 100);
    /// let gray_image = image.to_luma8();
    /// let feature = BriefFeature::new();
    /// let descriptor = feature.extract(&image).unwrap();
    /// ```
    fn extract(&self, image: &DynamicImage) -> Result<Vec<f32>, String> {
        let gray_image = image.to_luma8();
        // Detect keypoints using FAST9 corner detector
        let keypoints = corners_fast9(&gray_image, self.threshold);
        let tests = self.generate_tests(&keypoints, &gray_image);
        let descriptor_length = self.length as usize;
        let mut descriptor = Vec::with_capacity(descriptor_length);

        for keypoint in keypoints {
            let x = keypoint.x;
            let y = keypoint.y;
            for test in tests.iter() {
                if descriptor.len() == descriptor_length {
                    continue;
                }
                let (dx, dy) = test;
                match (
                    gray_image.get_pixel_checked(x + dx, y + dy),
                    gray_image.get_pixel_checked(x + dx, y + dy),
                ) {
                    (Some(pixel_a), Some(pixel_b)) => {
                        let pixel_difference: u8 =
                            (pixel_a[0] as i32 - pixel_b[0] as i32).unsigned_abs() as u8;
                        descriptor.push(if pixel_difference > self.threshold {
                            1.0
                        } else {
                            0.0
                        });
                    }
                    _ => (),
                };
            }
        }
        for _ in descriptor.len()..descriptor_length {
            descriptor.push(0.0);
        }

        Ok(descriptor)
    }
}

impl BriefFeature {
    /// Creates a new `BriefFeature` instance with default parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use object_detector_rust::feature::{BriefFeature, Feature};
    ///
    /// let feature = BriefFeature::new();
    /// ```
    pub fn new() -> BriefFeature {
        BriefFeature {
            threshold: 12,
            length: 32,
        }
    }

    /// Creates a new `BriefFeature` with the given threshold and length.
    ///
    /// # Parameters
    ///
    /// * `threshold`: The threshold value used for determining whether a pixel difference is significant.
    ///                The default value is 12.
    /// * `length`: The length of the descriptor, in pixels. The default value is 32.
    pub fn new_with_threshold_and_length(threshold: u8, length: u32) -> BriefFeature {
        BriefFeature { threshold, length }
    }

    /// Generates a list of tests to use for extracting the BRIEF descriptor.
    ///
    /// # Arguments
    ///
    /// * `keypoints` - A list of keypoints to generate tests for.
    /// * `image` - The image the keypoints were detected in.
    ///
    /// # Returns
    ///
    /// A list of tests as pairs of (dx, dy) offsets from the keypoint.
    fn generate_tests(&self, keypoints: &[Corner], image: &GrayImage) -> Vec<(u32, u32)> {
        let mut tests = Vec::new();
        let (width, height) = image.dimensions();
        for corner in keypoints {
            let x = corner.x;
            let y = corner.y;
            for angle in (0..360).step_by(45) {
                let angle = angle as f64;
                let dx = (self.length as f64 * angle.to_radians().cos()) as i32;
                let dy = (self.length as f64 * angle.to_radians().sin()) as i32;
                let x2 = x as i32 + dx;
                let y2 = y as i32 + dy;
                if x2 >= 0 && y2 >= 0 && x2 < width as i32 && y2 < height as i32 {
                    tests.push((x2 as u32, y2 as u32));
                }
            }
        }
        tests
    }
}

impl Default for BriefFeature {
    fn default() -> Self {
        BriefFeature::new_with_threshold_and_length(12, 512)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_threshold_and_length() {
        let brief_feature = BriefFeature::new_with_threshold_and_length(10, 40);
        assert_eq!(brief_feature.threshold, 10);
        assert_eq!(brief_feature.length, 40);
    }

    #[test]
    fn test_generate_tests() {
        let feature = BriefFeature::new();
        let keypoints = [
            Corner {
                x: 10,
                y: 10,
                score: 0.0,
            },
            Corner {
                x: 15,
                y: 20,
                score: 0.0,
            },
            Corner {
                x: 25,
                y: 30,
                score: 0.0,
            },
        ];
        let image = GrayImage::new(50, 50);
        let tests = feature.generate_tests(&keypoints, &image);
        assert!(tests.len() > 0);
    }

    #[test]
    fn test_brief_feature_extract() {
        let image = image::DynamicImage::new_luma8(100, 100);
        let length = 32;
        let feature = BriefFeature::new_with_threshold_and_length(12, length);
        let result = feature.extract(&image);
        assert!(result.is_ok());
        let descriptor = result.unwrap();
        assert_eq!(descriptor.len(), length as usize);
    }
}
