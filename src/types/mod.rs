//! The types module contains types for representing annotations and annotated images.
use crate::prelude::BBox;
use image::DynamicImage;

/// object class type
pub type Class = u32;

/// annotation is a object bounding box in image and class type
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    /// Bounding box of the object
    pub bbox: BBox,
    /// Class of the object
    pub class: Class,
}

/// Image annotated by list of Annotations
#[derive(Debug, Clone, PartialEq)]
pub struct AnnotatedImage {
    /// Image
    pub image: image::DynamicImage,
    /// Annotations for the image
    pub annotations: Vec<Annotation>,
}

impl Into<Annotation> for (BBox, Class) {
    /// Converts a tuple of bounding box and class into an Annotation.
    fn into(self) -> Annotation {
        let (bbox, class) = self;
        Annotation { bbox, class }
    }
}

impl From<Annotation> for (BBox, Class) {
    /// Converts an Annotation into a tuple of bounding box and class.
    fn from(annotation: Annotation) -> Self {
        let Annotation { bbox, class } = annotation;
        (bbox, class)
    }
}

impl Into<AnnotatedImage> for (DynamicImage, Vec<(BBox, Class)>) {
    /// Converts a tuple of image and a vector of bounding box and class tuples into an `AnnotatedImage`.
    fn into(self) -> AnnotatedImage {
        let (image, annotations) = self;
        AnnotatedImage {
            image,
            annotations: annotations
                .into_iter()
                .map(|(bbox, class)| Annotation { bbox, class })
                .collect(),
        }
    }
}

impl From<AnnotatedImage> for (DynamicImage, Vec<(BBox, Class)>) {
    /// Converts an AnnotatedImage into a tuple of image and a vector of bounding box and class tuples.
    fn from(annotated_image: AnnotatedImage) -> Self {
        let AnnotatedImage { image, annotations } = annotated_image;
        (
            image,
            annotations
                .into_iter()
                .map(|annotation| annotation.into())
                .collect(),
        )
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_image;

    #[test]
    fn test_annotation_conversions() {
        let bbox = BBox {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };
        let class: Class = 1;
        let annotation = Annotation {
            bbox: bbox.clone(),
            class,
        };

        // Test `Into<Annotation>` implementation
        let (bbox, class) = (bbox.clone(), class);
        let converted_annotation: Annotation = (bbox, class).into();
        assert_eq!(annotation, converted_annotation);

        // Test `From<Annotation>` implementation
        let (bbox, class) = annotation.into();
        let expected: (BBox, Class) = (bbox.clone(), class);
        assert_eq!((bbox, class), expected);
    }

    #[test]
    fn test_into_annotated_image() {
        let image = test_image();
        let annotations = vec![
            (
                BBox {
                    x: 0,
                    y: 0,
                    width: 50,
                    height: 50,
                },
                0,
            ),
            (
                BBox {
                    x: 50,
                    y: 0,
                    width: 50,
                    height: 50,
                },
                1,
            ),
            (
                BBox {
                    x: 0,
                    y: 50,
                    width: 50,
                    height: 50,
                },
                2,
            ),
            (
                BBox {
                    x: 50,
                    y: 50,
                    width: 50,
                    height: 50,
                },
                3,
            ),
        ];
        let expected = AnnotatedImage {
            image: image.clone(),
            annotations: annotations
                .clone()
                .into_iter()
                .map(|(bbox, class)| Annotation { bbox, class })
                .collect(),
        };

        let input = (image, annotations);
        let result: AnnotatedImage = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_from_annotated_image() {
        let image = image::DynamicImage::new_rgba8(100, 100);
        let annotations = vec![
            Annotation {
                bbox: BBox {
                    x: 0,
                    y: 0,
                    width: 50,
                    height: 50,
                },
                class: 0,
            },
            Annotation {
                bbox: BBox {
                    x: 50,
                    y: 0,
                    width: 50,
                    height: 50,
                },
                class: 1,
            },
        ];
        // Create an AnnotatedImage with image and a vector of bounding box and class tuples
        let annotated_image = AnnotatedImage {
            image: image.clone(),
            annotations: annotations.clone(),
        };

        // Convert the AnnotatedImage into a tuple of image and a vector of bounding box and class tuples
        let (image_2, annotations_2) = (annotated_image.image, annotated_image.annotations).into();

        // Assert that the image and annotations in the tuple match the original AnnotatedImage
        assert_eq!(image_2, image);
        assert_eq!(
            annotations_2,
            vec![
                Annotation {
                    bbox: BBox {
                        x: 0,
                        y: 0,
                        width: 50,
                        height: 50
                    },
                    class: 0,
                },
                Annotation {
                    bbox: BBox {
                        x: 50,
                        y: 0,
                        width: 50,
                        height: 50
                    },
                    class: 1,
                }
            ]
        );
    }
}
