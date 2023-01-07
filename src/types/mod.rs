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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_image;

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
}
