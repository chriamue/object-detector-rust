//! Utility functions for working with images
//!
use crate::BBox;
use image::{imageops::crop_imm, DynamicImage, GenericImageView, Rgba, SubImage};
use imageproc::{drawing::draw_hollow_rect_mut, rect::Rect};

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

/// Draws bounding boxes on an image.
///
/// # Example
///
/// ```
/// use object_detector_rust::{BBox, utils::draw_bboxes};
/// use image::DynamicImage;
///
/// let mut image = DynamicImage::new_rgb8(100, 100);
/// let bboxes = vec![BBox::new(10, 10, 20, 20)];
/// draw_bboxes(&mut image, &bboxes);
/// ```
///
/// This example will draw a red bounding box with top-left corner at (10, 10) and bottom-right corner at (30, 30) on a 100x100 image.
pub fn draw_bboxes(image: &mut DynamicImage, bboxes: &[BBox]) {
    let (width, height) = image.dimensions();
    let color = Rgba([255, 0, 0, 255]);
    for bbox in bboxes {
        let (x1, y1, x2, y2) = (
            bbox.x.max(0) as i32,
            bbox.y.max(0) as i32,
            (bbox.x + bbox.width as i32).min(width as i32 - 1) as i32,
            (bbox.y + bbox.height as i32).min(height as i32 - 1) as i32,
        );
        draw_hollow_rect_mut(
            image,
            Rect::at(x1, y1).of_size((x2 - x1) as u32, (y2 - y1) as u32),
            color,
        );
    }
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

    #[test]
    fn test_draw_bboxes() {
        // Create an empty image
        let mut image = DynamicImage::new_rgb8(100, 100);

        // Draw two bounding boxes on the image
        let bboxes = vec![BBox::new(10, 10, 10, 10), BBox::new(30, 30, 40, 40)];
        draw_bboxes(&mut image, &bboxes);

        // Check that the pixels at the expected locations have the expected colors
        assert_eq!(image.get_pixel(9, 9), image::Rgba([0, 0, 0, 255]));
        assert_eq!(image.get_pixel(10, 10), image::Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(29, 29), image::Rgba([0, 0, 0, 255]));
        assert_eq!(image.get_pixel(30, 30), image::Rgba([255, 0, 0, 255]));
    }
}
