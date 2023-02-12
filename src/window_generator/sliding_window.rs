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

// let x_border = if self.width - self.step_size > 0 {
//    self.width
// } else {
//    0
// };

impl WindowGenerator<DynamicImage> for SlidingWindow {
    fn windows<'a, 'b>(
        &'a self,
        image: &'b DynamicImage,
    ) -> Vec<ImageWindow<SubImage<&'b DynamicImage>>> {
        let mut windows = Vec::new();
        let (width, height) = image.dimensions();

        let last_step_y = last_position(height, self.step_size);

        let last_y = if last_step_y + self.height > height {
            height - self.height
        } else {
            last_step_y + self.height
        };

        let last_step_x = last_position(width, self.step_size);

        let last_x = if last_step_x + self.width > width {
            width - self.width
        } else {
            last_step_x + self.width
        };

        for y in (0..last_y).step_by(self.step_size as usize) {
            for x in (0..last_x).step_by(self.step_size as usize) {
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

fn last_position(length: u32, step_size: u32) -> u32 {
    if length == 0 || step_size == 0 {
        return 0;
    }
    (length - 1) / step_size * step_size
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

    #[test]
    fn test_sliding_window_step_size_7() {
        let image = DynamicImage::new_rgb8(10, 10);
        let window_size = 7;
        let step_size = 7;

        let generator = SlidingWindow {
            width: window_size,
            height: window_size,
            step_size: step_size,
        };
        let windows = generator.windows(&image);
        assert_eq!(windows.len(), 1);
        assert_eq!(windows[0].x, 0);
    }

    #[test]
    fn test_sliding_window_size_greater_than_step_size() {
        let image = DynamicImage::new_rgb8(12, 12);
        let window_size = 5;
        let step_size = 4;

        let generator = SlidingWindow {
            width: window_size,
            height: window_size,
            step_size: step_size,
        };
        let windows = generator.windows(&image);
        assert_eq!(windows.len(), 4);
    }
}
