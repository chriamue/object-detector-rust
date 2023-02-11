use super::{SlidingWindow, WindowGenerator};
use crate::types::ImageWindow;
use image::{DynamicImage, SubImage};

/// Struct for generating sliding windows over an image
#[derive(Debug, PartialEq)]
pub struct PyramidWindow {
    /// Width of the layer 1 window
    pub width: u32,
    /// Height of the layer 1 window
    pub height: u32,
    /// Step size for moving the window
    pub step_size: u32,
    /// the pyramid layers
    pub layers: usize,
}

impl WindowGenerator<DynamicImage> for PyramidWindow {
    fn windows<'a, 'b>(
        &'a self,
        image: &'b DynamicImage,
    ) -> Vec<ImageWindow<SubImage<&'b DynamicImage>>> {
        let mut windows = Vec::new();
        for scale in 1..self.layers as u32 + 1 {
            let sliding_window = SlidingWindow {
                width: self.width * scale,
                height: self.height * scale,
                step_size: self.step_size * scale,
            };
            windows.append(&mut sliding_window.windows(image));
        }
        windows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_window() {
        let image = DynamicImage::new_rgb8(10, 10);
        let window_size = 5;
        let step_size = 5;

        let generator = PyramidWindow {
            width: window_size,
            height: window_size,
            step_size: step_size,
            layers: 2,
        };

        let windows = generator.windows(&image);

        // Check that the correct number of windows was returned
        assert_eq!(windows.len(), 5);

        // Check that the positions and subimages of the returned windows are correct
        assert_eq!(windows[0].x, 0);
        assert_eq!(windows[0].x, 0);
        assert_eq!(windows[1].x, 5);
        assert_eq!(windows[2].y, 5);
        assert_eq!(windows[3].x, 5);
        assert_eq!(windows[3].y, 5);
    }
}
