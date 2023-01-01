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
            if max_annotations.is_some() && max_annotations.unwrap() <= annotations_counter {
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
