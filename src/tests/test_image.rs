use image::{DynamicImage, Rgb, RgbImage};
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_filled_ellipse_mut, draw_filled_rect_mut},
    rect::Rect,
};

/// a simple image for testing purposes
pub fn test_image() -> DynamicImage {
    let mut img = RgbImage::new(100, 100);
    draw_filled_circle_mut(&mut img, (75, 25), 25, Rgb([255, 0, 0]));
    draw_filled_rect_mut(&mut img, Rect::at(25, 75).of_size(10, 10), Rgb([0, 0, 255]));
    draw_filled_ellipse_mut(&mut img, (75, 75), 25, 10, Rgb([0, 255, 0]));
    DynamicImage::ImageRgb8(img)
}

#[test]
fn test_image_size() {
    let img = test_image();
    assert_eq!(100, img.width());
    assert_eq!(100, img.height());
}

#[test]
fn test_pixel_color() {
    let img = test_image().into_rgb8();
    assert_eq!(&Rgb([255, 0, 0]), img.get_pixel(75, 25));
    assert_eq!(&Rgb([0, 255, 0]), img.get_pixel(55, 80));
    assert_eq!(&Rgb([0, 0, 255]), img.get_pixel(30, 80));
}

#[ignore = "enable if you want test image as png"]
#[test]
fn save_test_image() {
    let img = test_image();
    img.save("test_image.png").unwrap();
}
