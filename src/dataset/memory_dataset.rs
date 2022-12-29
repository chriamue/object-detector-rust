use super::{AnnotatedImageSet, DataSet};
use crate::{utils::crop_bbox, AnnotatedImage};
use image::DynamicImage;
use std::error::Error;

/// Struct for representing a dataset in memory
///
/// # Examples
///
/// ```
/// use image::DynamicImage;
/// use object_detector_rust::{AnnotatedImage, dataset::{AnnotatedImageSet, MemoryDataSet, DataSet}};
///
/// let mut dataset = MemoryDataSet::new();
/// dataset.add_annotated_image(AnnotatedImage {
///     image: DynamicImage::new_rgba8(100, 100),
///     annotations: Vec::new(),
/// });
///
/// assert_eq!(dataset.annotated_images_len(), 1);
/// assert_eq!(dataset.len(), 0);
/// ```
#[derive(Default)]
pub struct MemoryDataSet {
    /// Vector of images with annotations in the dataset
    annotated_images: Vec<AnnotatedImage>,
}

impl MemoryDataSet {
    /// Creates a new, empty dataset
    pub fn new() -> Self {
        Self {
            annotated_images: vec![],
        }
    }
}

impl AnnotatedImageSet for MemoryDataSet {
    fn annotated_images_len(&self) -> usize {
        self.annotated_images.len()
    }

    /// Adds an annotated image to the dataset
    fn add_annotated_image(&mut self, annotated_image: AnnotatedImage) {
        self.annotated_images.push(annotated_image);
    }

    /// returns iterator over images
    fn annotated_images(&self) -> Box<dyn Iterator<Item = &AnnotatedImage> + '_> {
        Box::new(self.annotated_images.iter())
    }
}

impl DataSet for MemoryDataSet {
    fn len(&self) -> usize {
        self.annotated_images
            .iter()
            .map(|annotated_image| annotated_image.annotations.len())
            .sum()
    }

    fn get_data(&self) -> (Vec<DynamicImage>, Vec<u32>) {
        let mut images = Vec::new();
        let mut labels = Vec::new();

        for annotated_image in self.annotated_images.iter() {
            for annotation in annotated_image.annotations.iter() {
                images.push(crop_bbox(&annotated_image.image, &annotation.bbox));
                labels.push(annotation.class);
            }
        }

        (images, labels)
    }

    fn load(&mut self) -> Result<(), Box<dyn Error>> {
        // Since the annotated images are already provided, there is no need to implement a load method
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Annotation, BBox};

    #[test]
    fn test_memory_dataset_add_annotated_image() {
        let mut dataset = MemoryDataSet::new();
        let image = DynamicImage::new_rgba8(10, 10);
        let annotation1 = Annotation {
            bbox: BBox {
                x: 0,
                y: 0,
                width: 5,
                height: 5,
            },
            class: 0,
        };
        let annotation2 = Annotation {
            bbox: BBox {
                x: 5,
                y: 5,
                width: 5,
                height: 5,
            },
            class: 1,
        };
        let annotated_image = AnnotatedImage {
            image,
            annotations: vec![annotation1, annotation2],
        };

        dataset.add_annotated_image(annotated_image);

        assert_eq!(dataset.annotated_images_len(), 1);
        assert_eq!(dataset.len(), 2);
    }

    #[test]
    fn test_memory_dataset_get_data() {
        let mut dataset = MemoryDataSet::new();
        let image = DynamicImage::new_rgba8(10, 10);
        let annotation1 = Annotation {
            bbox: BBox {
                x: 0,
                y: 0,
                width: 5,
                height: 5,
            },
            class: 0,
        };
        let annotation2 = Annotation {
            bbox: BBox {
                x: 5,
                y: 5,
                width: 5,
                height: 5,
            },
            class: 1,
        };
        let annotated_image = AnnotatedImage {
            image,
            annotations: vec![annotation1, annotation2],
        };

        dataset.add_annotated_image(annotated_image);

        let (images, labels) = dataset.get_data();
        assert_eq!(images.len(), 2);
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0], 0);
        assert_eq!(labels[1], 1);
    }

    #[test]
    fn test_memory_dataset_load() {
        let mut dataset = MemoryDataSet::new();

        let result = dataset.load();
        assert!(result.is_ok());
    }
    #[test]
    fn test_annotated_images() {
        let mut dataset = MemoryDataSet::new();

        // Create some annotated images and add them to the dataset
        let annotated_image_1 = AnnotatedImage {
            image: DynamicImage::new_rgba8(1, 1),
            annotations: vec![Annotation {
                bbox: BBox {
                    x: 0,
                    y: 0,
                    width: 1,
                    height: 1,
                },
                class: 0,
            }],
        };
        let annotated_image_2 = AnnotatedImage {
            image: DynamicImage::new_rgba8(2, 2),
            annotations: vec![
                Annotation {
                    bbox: BBox {
                        x: 0,
                        y: 0,
                        width: 1,
                        height: 1,
                    },
                    class: 1,
                },
                Annotation {
                    bbox: BBox {
                        x: 1,
                        y: 1,
                        width: 1,
                        height: 1,
                    },
                    class: 2,
                },
            ],
        };
        dataset.add_annotated_image(annotated_image_1.clone());
        dataset.add_annotated_image(annotated_image_2.clone());

        // Get the annotated images from the dataset and check that they are correct
        let annotated_images: Vec<&AnnotatedImage> = dataset.annotated_images().collect();
        assert_eq!(
            annotated_images,
            vec![&annotated_image_1, &annotated_image_2]
        );
    }
}
