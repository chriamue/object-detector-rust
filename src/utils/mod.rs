//! Utility functions for working with images
//!
use crate::BBox;
use image::{imageops::crop_imm, DynamicImage, GenericImageView, SubImage};

mod sliding_window;
pub use sliding_window::SlidingWindow;

/// Struct representing a window over an image with a position
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageWindow<I> {
    /// X position of the window
    pub x: u32,
    /// Y position of the window
    pub y: u32,
    /// View of the image within the window
    pub view: I,
}

/// Trait for generating windows over an image
pub trait WindowGenerator<I>
where
    I: GenericImageView,
{
    /// Generates a series of windows over the given image
    ///
    /// # Arguments
    ///
    /// * `image` - the image to generate windows for
    ///
    /// # Returns
    ///
    /// A vector of `ImageWindow`s representing the windows over the image
    fn windows<'a, 'b>(&'a self, image: &'b I) -> Vec<ImageWindow<SubImage<&'b I>>>;
}

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
