use image::{GenericImageView, SubImage};

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

/// Returns a vector of image windows for the given image using a sliding window approach.
///
/// The sliding window is a square of size `window_size` pixels that is moved across the image
/// with a step size of `step_size` pixels in both the x and y directions
/// from left to right and top to bottom. At each position, the window is added to the vector
/// of image windows returned by the function.
///
/// # Parameters
///
/// - `image`: The image to generate image windows for.
/// - `window_size`: The size of the sliding window in pixels.
/// - `step_size` - The number of pixels to move the window in both the x and y directions at each iteration
///
/// # Example
///
/// ```
/// use object_detector_rust::utils::sliding_window;
/// use image::DynamicImage;
///
/// let image = DynamicImage::new_rgba8(100, 100);
/// let windows = sliding_window(&image, 50, 25);
/// ```
pub fn sliding_window<I: GenericImageView>(
    image: &I,
    window_size: u32,
    step_size: u32,
) -> Vec<ImageWindow<SubImage<&I>>> {
    let mut windows = Vec::new();
    for y in (0..(image.height() - window_size + 1)).step_by(step_size as usize) {
        for x in (0..(image.width() - window_size + 1)).step_by(step_size as usize) {
            let window = ImageWindow {
                x,
                y,
                view: image.view(x, y, window_size, window_size),
            };
            windows.push(window);
        }
    }
    windows
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::DynamicImage;

    #[test]
    fn test_sliding_window() {
        let image = DynamicImage::new_rgb8(10, 10);
        let window_size = 5;
        let step_size = 5;

        let windows = sliding_window(&image, window_size, step_size);

        // Check that the correct number of windows was returned
        assert_eq!(windows.len(), 4);

        // Check that the positions and subimages of the returned windows are correct
        assert_eq!(windows[0].x, 0);
        assert_eq!(windows[1].x, 5);
        assert_eq!(windows[2].y, 5);
        assert_eq!(windows[3].x, 5);
        assert_eq!(windows[3].y, 5);
    }
}
