use crate::{
    classifier::SVMClassifier,
    detector::{Detection, Detector},
    feature::{Feature, HOGFeature},
    predictable::Predictable,
    trainable::Trainable,
    BBox, Class,
};
use image::DynamicImage;
use ndarray::{Array2, ArrayView1, ArrayView2};

/// Struct representing an object detector using HOG features and an SVM classifier
#[derive(Debug)]
pub struct HOGSVMDetector {
    hog_feature: HOGFeature,
    svm_classifier: SVMClassifier,
}

impl HOGSVMDetector {
    /// Creates a new `HOGSVMDetector` with default parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Trains the HOGSVMDetector on the given training data and labels
    pub fn fit<'a, 'b>(
        &'a mut self,
        x: &'b ArrayView2<f32>,
        y: &'b ArrayView1<bool>,
    ) -> Result<(), String> {
        self.svm_classifier.fit(x, y)
    }
}

impl Default for HOGSVMDetector {
    fn default() -> Self {
        Self {
            hog_feature: HOGFeature::new(),
            svm_classifier: SVMClassifier::new(),
        }
    }
}

impl Detector for HOGSVMDetector {
    fn detect(&self, image: &DynamicImage) -> Vec<Detection> {
        let hog_features = self.hog_feature.extract(image).unwrap();
        let hog_features = Array2::from_shape_vec((1, hog_features.len()), hog_features).unwrap();
        let predictions = self.svm_classifier.predict(&hog_features.view()).unwrap();

        let mut detections = Vec::new();
        for (i, &prediction) in predictions.iter().enumerate() {
            if prediction {
                detections.push(Detection::new(
                    BBox::new(0, 0, image.width(), image.height()),
                    i as Class,
                    1.0,
                ));
            }
        }

        detections
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array1;

    #[test]
    fn test_hog_svm_detector() {
        // Create a simple image with a single black square
        let image = DynamicImage::new_luma8(32, 32);

        // Extract HOG features from the image
        let hog_feature = HOGFeature::new();
        let hog_features = hog_feature.extract(&image).unwrap();

        // Create training data and labels
        let x = Array2::from_shape_vec((1, hog_features.len()), hog_features).unwrap();
        let y = Array1::from(vec![true]);

        // Create a new HOGSVMDetector and train it on the training data
        let mut hog_svm_detector = HOGSVMDetector::new();
        hog_svm_detector.fit(&x.view(), &y.view()).unwrap();

        // Make predictions on the image using the trained detector
        let detections = hog_svm_detector.detect(&image);

        // Check that the predicted detection is correct
        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].class, 0);
        assert_eq!(detections[0].bbox, BBox::new(0, 0, 32, 32));
        assert_eq!(detections[0].confidence, 1.0);
    }
}
