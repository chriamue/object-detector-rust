use crate::prelude::{AnnotatedImage, AnnotatedImageSet, Annotation, BBox};
use image::{DynamicImage, GenericImageView};
use rand::{thread_rng, Rng};

/// Generates random annotations for the provided annotated image set.
///
/// # Arguments
///
/// * `annotated_image_set` - The set of annotated images to add the annotations to.
/// * `count` - The number of annotations to generate for each annotated image.
/// * `max_count` - An optional maximum number of annotations to generate for all annotated images.
/// * `width` - The width of the generated bounding boxes.
/// * `height` - The height of the generated bounding boxes.
pub fn generate_annotations(
    annotated_image_set: &mut dyn AnnotatedImageSet,
    count: usize,
    max_count: Option<usize>,
    width: u32,
    height: u32,
) {
    let max_count = max_count.unwrap_or(std::usize::MAX);
    let mut annotation_count = 0;
    let mut rng = thread_rng();
    let images: Vec<DynamicImage> = annotated_image_set
        .annotated_images()
        .map(|annotated_image| annotated_image.image.clone())
        .collect();
    for image in images {
        let image_size = image.dimensions();
        let mut annotations = Vec::new();
        for _ in 0..count {
            if annotation_count == max_count {
                break;
            }
            let x = rng.gen_range(0..image_size.0 - width);
            let y = rng.gen_range(0..image_size.1 - height);
            let bbox = BBox {
                x: x as i32,
                y: y as i32,
                width,
                height,
            };
            let annotation = Annotation { bbox, class: 0 };
            annotations.push(annotation);
            annotation_count += 1;
        }
        let new_annotated_image = AnnotatedImage { image, annotations };
        annotated_image_set.add_annotated_image(new_annotated_image);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{DataSet, MemoryDataSet};

    use super::*;
    #[test]
    fn test_generate_annotations() {
        let mut dataset = MemoryDataSet::new();

        // Create some annotated images and add them to the dataset
        let annotated_image_1 = AnnotatedImage {
            image: DynamicImage::new_rgba8(100, 100),
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
            image: DynamicImage::new_rgba8(100, 100),
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

        // Generate 10 random annotations for each image with a maximum of 50 total annotations
        generate_annotations(&mut dataset, 10, Some(15), 10, 10);

        // Assert that the annotated image set has the correct number of annotated images
        assert_eq!(dataset.annotated_images_len(), 4);

        assert_eq!(dataset.len(), 1 + 2 + 10 + 5);
    }
}
