use super::WindowGenerator;
use crate::types::ImageWindow;
use image::{DynamicImage, GenericImageView, SubImage};

/// Struct for generating sliding windows over an image
#[derive(Debug, PartialEq)]
pub struct SlidingWindow {
    /// Width of the window
    pub width: u32,
    /// Height of the window
    pub height: u32,
    /// Step size for moving the window
    pub step_size: u32,
}

impl WindowGenerator<DynamicImage> for SlidingWindow {
    fn windows<'a, 'b>(
        &'a self,
        image: &'b DynamicImage,
    ) -> Vec<ImageWindow<SubImage<&'b DynamicImage>>> {
        let mut windows = Vec::new();
        let (width, height) = image.dimensions();
        let y_border = if self.height - self.step_size > 0 {
            self.height
        } else {
            0
        };
        let x_border = if self.width - self.step_size > 0 {
            self.width
        } else {
            0
        };
        for y in (0..height.saturating_sub(y_border)).step_by(self.step_size as usize) {
            for x in (0..width.saturating_sub(x_border)).step_by(self.step_size as usize) {
                let window = ImageWindow {
                    x,
                    y,
                    view: image.view(x, y, self.width, self.height),
                };
                windows.push(window);
            }
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

        let generator = SlidingWindow {
            width: window_size,
            height: window_size,
            step_size: step_size,
        };

        let windows = generator.windows(&image);

        // Check that the correct number of windows was returned
        assert_eq!(windows.len(), 4);

        // Check that the positions and subimages of the returned windows are correct
        assert_eq!(windows[0].x, 0);
        assert_eq!(windows[0].x, 0);
        assert_eq!(windows[1].x, 5);
        assert_eq!(windows[2].y, 5);
        assert_eq!(windows[3].x, 5);
        assert_eq!(windows[3].y, 5);
    }

    #[test]
    fn test_sliding_window_step_size() {
        let image = DynamicImage::new_rgb8(21, 21);
        let window_size = 10;
        let step_size = 5;

        let generator = SlidingWindow {
            width: window_size,
            height: window_size,
            step_size: step_size,
        };

        let windows = generator.windows(&image);

        // Check that the correct number of windows was returned
        assert_eq!(windows.len(), 9);

        // Check that the positions and subimages of the returned windows are correct
        assert_eq!(windows[0].x, 0);
        assert_eq!(windows[1].x, 5);
        assert_eq!(windows[2].y, 0);
        assert_eq!(windows[3].x, 0);
        assert_eq!(windows[3].y, 5);
    }
}
