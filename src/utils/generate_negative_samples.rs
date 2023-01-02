use crate::{
    prelude::{AnnotatedImageSet, Detector},
    AnnotatedImage, Annotation,
};

/// trait for generating data
/// generates hard negative samples, see: [Hard Negative Mining](https://openaccess.thecvf.com/content_ECCV_2018/papers/SouYoung_Jin_Unsupervised_Hard-Negative_Mining_ECCV_2018_paper.pdf)
pub fn add_hard_negative_samples(
    dataset: &mut dyn AnnotatedImageSet,
    detector: &dyn Detector,
    class: u32,
    max_annotations: Option<usize>,
) {
    let annotated_images = generate_negative_samples(dataset, detector, class, max_annotations);
    annotated_images
        .into_iter()
        .for_each(|annotated_image| dataset.add_annotated_image(annotated_image));
}

/// generates negative samples
pub fn generate_negative_samples(
    dataset: &dyn AnnotatedImageSet,
    detector: &dyn Detector,
    class: u32,
    max_annotations: Option<usize>,
) -> Vec<AnnotatedImage> {
    let mut annotations_counter = 0;
    let mut generated_annotated_images: Vec<AnnotatedImage> = Vec::new();
    for annotated_image in dataset.annotated_images() {
        let detections = detector.detect(&annotated_image.image);
        let mut false_pos_annotations = Vec::new();
        detections.iter().for_each(|detection| {
            if max_annotations.is_some() && max_annotations.unwrap() >= annotations_counter {
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
                        bbox: false_pos_bbox,
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
    use crate::prelude::{Detection, MemoryDataSet};
    use crate::BBox;

    use super::*;
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
        let negative_samples = generate_negative_samples(&dataset, &mock_detector, 1, None);

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
        add_hard_negative_samples(&mut dataset, &mock_detector, 1, None);
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
