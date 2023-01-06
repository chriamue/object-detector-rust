use crate::{
    prelude::{AnnotatedImage, AnnotatedImageSet, Annotation, BBox, Detector},
    Class,
};
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

/// generates hard negative samples, see: [Hard Negative Mining](https://openaccess.thecvf.com/content_ECCV_2018/papers/SouYoung_Jin_Unsupervised_Hard-Negative_Mining_ECCV_2018_paper.pdf)
///
/// # Arguments
///
/// * `annotated_image_set` - The set of annotated images to add the annotations to.
/// * `detector` - The hard negative mining detector.
/// * `class`- The class to assign to the hard negative samples.
/// * `max_count` - An optional maximum number of annotations to generate for all annotated images.
/// * `width` - The width of the generated bounding boxes.
/// * `height` - The height of the generated bounding boxes.
pub fn add_hard_negative_samples(
    annotated_image_set: &mut dyn AnnotatedImageSet,
    detector: &dyn Detector,
    class: Class,
    max_count: Option<usize>,
    width: u32,
    height: u32,
) {
    let annotated_images = generate_negative_samples(
        annotated_image_set,
        detector,
        class,
        max_count,
        width,
        height,
    );
    annotated_images
        .into_iter()
        .for_each(|annotated_image| annotated_image_set.add_annotated_image(annotated_image));
}

/// generates negative samples
/// ///
/// # Arguments
///
/// * `annotated_image_set` - The set of annotated images to add the annotations to.
/// * `detector` - The hard negative mining detector.
/// * `class`- The class to assign to the hard negative samples.
/// * `max_count` - An optional maximum number of annotations to generate for all annotated images.
/// * `width` - The width of the generated bounding boxes.
/// * `height` - The height of the generated bounding boxes.
///
/// # Returns
///
/// Vec<AnnotatedImage> - The generated negative samples.
///
pub fn generate_negative_samples(
    annotated_image_set: &dyn AnnotatedImageSet,
    detector: &dyn Detector,
    class: Class,
    max_count: Option<usize>,
    width: u32,
    height: u32,
) -> Vec<AnnotatedImage> {
    let mut annotations_counter = 0;
    let mut generated_annotated_images: Vec<AnnotatedImage> = Vec::new();
    for annotated_image in annotated_image_set.annotated_images() {
        let detections = detector.detect(&annotated_image.image);
        let mut false_pos_annotations = Vec::new();
        detections.iter().for_each(|detection| {
            if max_count.is_some() && max_count.unwrap() >= annotations_counter {
                return;
            }
            if detection.class as u32 == class {
                let mut false_pos = true;
                annotated_image.annotations.iter().for_each(|annotation| {
                    if class == annotation.class {
                        if annotation.bbox.overlap(&detection.bbox) > 0.1 {
                            false_pos = false;
                        }
                    }
                });
                if false_pos {
                    let false_pos_bbox = detection.bbox.clone();
                    let false_pos_annotation: Annotation = Annotation {
                        bbox: BBox {
                            x: false_pos_bbox.x,
                            y: false_pos_bbox.y,
                            width,
                            height,
                        },
                        class: 0,
                    };
                    false_pos_annotations.push(false_pos_annotation);
                    annotations_counter += 1;
                }
            }
        });
        generated_annotated_images.push(AnnotatedImage {
            image: annotated_image.image.clone(),
            annotations: false_pos_annotations,
        });
    }
    generated_annotated_images
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bbox::BBox;
    use crate::prelude::{DataSet, Detection, MemoryDataSet};
    use image::DynamicImage;
    use mockall::predicate::*;
    use mockall::*;

    // Define the mock Detector trait
    mock! {
        DummyDetector {}
        impl Detector for DummyDetector {
            fn detect(&self, image: &DynamicImage) -> Vec<Detection>;
        }
    }

    // Set up the mock Detector with default return values
    fn mock_detector() -> MockDummyDetector {
        let mut mock = MockDummyDetector::new();
        mock.expect_detect().returning(|_| {
            vec![Detection {
                bbox: BBox::new(10, 10, 5, 5),
                class: 1,
                confidence: 1.0,
            }]
        });
        mock
    }

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

    #[test]
    fn test_generate_negative_samples() {
        // Set up the mock Detector
        let mock_detector = mock_detector();

        // Set up the MemoryDataSet and add some test annotated images
        let mut dataset = MemoryDataSet::new();
        let annotated_image_1 = AnnotatedImage {
            image: DynamicImage::new_rgba8(20, 20),
            annotations: vec![
                Annotation {
                    bbox: BBox {
                        x: 0,
                        y: 0,
                        width: 5,
                        height: 5,
                    },
                    class: 0,
                },
                Annotation {
                    bbox: BBox {
                        x: 5,
                        y: 5,
                        width: 5,
                        height: 5,
                    },
                    class: 1,
                },
            ],
        };
        let annotated_image_2 = AnnotatedImage {
            image: DynamicImage::new_rgba8(20, 20),
            annotations: vec![
                Annotation {
                    bbox: BBox {
                        x: 0,
                        y: 0,
                        width: 5,
                        height: 5,
                    },
                    class: 1,
                },
                Annotation {
                    bbox: BBox {
                        x: 10,
                        y: 10,
                        width: 10,
                        height: 10,
                    },
                    class: 1,
                },
            ],
        };
        dataset.add_annotated_image(annotated_image_1);
        dataset.add_annotated_image(annotated_image_2);

        // Call generate_negative_samples with the mock Detector and test dataset
        let negative_samples = generate_negative_samples(&dataset, &mock_detector, 1, None, 5, 5);

        // Verify that the correct number of negative samples were generated
        assert_eq!(negative_samples.len(), 2);
        assert_eq!(negative_samples[0].annotations.len(), 1);
        assert_eq!(negative_samples[1].annotations.len(), 0);
    }

    #[test]
    fn test_add_hard_negative_samples() {
        // Set up the mock Detector
        let mock_detector = mock_detector();

        // Set up the MemoryDataSet and add some test annotated images
        let mut dataset = MemoryDataSet::new();
        let annotated_image_1 = AnnotatedImage {
            image: DynamicImage::new_rgba8(20, 20),
            annotations: vec![
                Annotation {
                    bbox: BBox {
                        x: 0,
                        y: 0,
                        width: 5,
                        height: 5,
                    },
                    class: 0,
                },
                Annotation {
                    bbox: BBox {
                        x: 5,
                        y: 5,
                        width: 5,
                        height: 5,
                    },
                    class: 1,
                },
            ],
        };
        let annotated_image_2 = AnnotatedImage {
            image: DynamicImage::new_rgba8(20, 20),
            annotations: vec![
                Annotation {
                    bbox: BBox {
                        x: 0,
                        y: 0,
                        width: 5,
                        height: 5,
                    },
                    class: 1,
                },
                Annotation {
                    bbox: BBox {
                        x: 10,
                        y: 10,
                        width: 10,
                        height: 10,
                    },
                    class: 1,
                },
            ],
        };
        dataset.add_annotated_image(annotated_image_1);
        dataset.add_annotated_image(annotated_image_2);

        // Call generate_negative_samples with the mock Detector and test dataset
        add_hard_negative_samples(&mut dataset, &mock_detector, 1, None, 5, 5);
        assert_eq!(dataset.annotated_images_len(), 4);
        assert_eq!(
            dataset
                .annotated_images()
                .skip(2)
                .next()
                .unwrap()
                .annotations
                .len(),
            1
        );
        assert_eq!(
            dataset.annotated_images().last().unwrap().annotations.len(),
            0
        );
    }
}
