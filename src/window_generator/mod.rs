//! The module for different window generators.
use crate::types::ImageWindow;
use image::{GenericImageView, SubImage};

mod pyramid_window;
mod sliding_window;

pub use pyramid_window::PyramidWindow;
pub use sliding_window::SlidingWindow;

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
