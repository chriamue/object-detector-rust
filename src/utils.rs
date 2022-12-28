//! Utility functions for working with images
//!
use crate::BBox;
use image::{imageops::crop_imm, DynamicImage};

/// Crops an image to the dimensions specified in the bounding box
///
/// # Arguments
///
/// * `image` - The image to crop
/// * `bbox` - The bounding box defining the region to crop
///
/// # Returns
///
/// The cropped image
pub fn crop_bbox(image: &DynamicImage, bbox: &BBox) -> DynamicImage {
    let cropped = crop_imm(image, bbox.x as u32, bbox.y as u32, bbox.width, bbox.height).to_image();
    DynamicImage::ImageRgba8(cropped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crop_bbox() {
        let image = DynamicImage::new_rgba8(100, 100);
        let bbox = BBox {
            x: 25,
            y: 25,
            width: 50,
            height: 50,
        };
        let cropped_image = crop_bbox(&image, &bbox);

        // Check that the dimensions of the cropped image are correct
        assert_eq!(cropped_image.width(), 50);
        assert_eq!(cropped_image.height(), 50);
    }
}
