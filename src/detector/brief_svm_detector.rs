use std::{
    error::Error,
    io::{Read, Write},
};

use crate::{
    bbox::BBox,
    classifier::SVMClassifier,
    detector::{Detection, Detector},
    feature::{BriefFeature, Feature},
    predictable::Predictable,
    trainable::Trainable,
    types::Class,
    window_generator::{SlidingWindow, WindowGenerator},
};
use image::{DynamicImage, GenericImageView};
use ndarray::{Array2, ArrayView1, ArrayView2};

use super::PersistentDetector;

/// Struct representing an object detector using HOG features and an SVM classifier
#[derive(Debug, PartialEq)]
pub struct BriefSVMDetector<W>
where
    W: WindowGenerator<DynamicImage>,
{
    brief_feature: BriefFeature,
    svm_classifier: SVMClassifier<f32, bool>,
    window_generator: W,
}

impl<W> BriefSVMDetector<W>
where
    W: WindowGenerator<DynamicImage>,
{
    /// Creates a new `HOGSVMDetector` with default parameters
    pub fn new(window_generator: W) -> Self {
        Self {
            brief_feature: BriefFeature::default(),
            svm_classifier: SVMClassifier::new(),
            window_generator,
        }
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

impl Default for BriefSVMDetector<SlidingWindow> {
    fn default() -> Self {
        Self {
            brief_feature: BriefFeature::default(),
            svm_classifier: SVMClassifier::new(),
            window_generator: SlidingWindow {
                width: 32,
                height: 32,
                step_size: 32,
            },
        }
    }
}

impl<W> Detector for BriefSVMDetector<W>
where
    W: WindowGenerator<DynamicImage>,
{
    fn detect(&self, image: &DynamicImage) -> Vec<Detection> {
        let windows = self.window_generator.windows(image);
        let windows_len = windows.len();
        let brief_features: Vec<Vec<f32>> = windows
            .iter()
            .flat_map(|window| {
                self.brief_feature
                    .extract(&DynamicImage::ImageRgba8(window.view.to_image()))
            })
            .collect();

        let features_len = match brief_features.first() {
            Some(features) => features.len(),
            None => 0,
        };
        let brief_features: Vec<f32> = brief_features.into_iter().flatten().collect();
        let brief_features =
            Array2::from_shape_vec((windows_len, features_len), brief_features).unwrap();
        let predictions = self.svm_classifier.predict(&brief_features.view()).unwrap();
        assert_eq!(windows_len, predictions.len());
        let mut detections = Vec::new();
        for (i, &prediction) in predictions.iter().enumerate() {
            if prediction {
                let window = windows[i];
                detections.push(Detection::new(
                    BBox::new(
                        window.x as i32,
                        window.y as i32,
                        window.view.width(),
                        window.view.height(),
                    ),
                    1 as Class,
                    1.0,
                ));
            }
        }
        detections
    }
}

impl<WG> PersistentDetector for BriefSVMDetector<WG>
where
    WG: WindowGenerator<DynamicImage>,
{
    fn save<W: Write>(&self, mut writer: W) -> Result<(), Box<dyn Error>> {
        // Serialize the SVMClassifier using the `bincode` crate
        let svm_classifier_bytes = bincode::serialize(&self.svm_classifier)?;

        // Write the serialized bytes to the writer
        writer.write_all(&svm_classifier_bytes)?;

        Ok(())
    }

    fn load<R: Read>(&mut self, mut reader: R) -> Result<(), Box<dyn Error>> {
        // Read the serialized bytes from the reader
        let mut svm_classifier_bytes = Vec::new();
        reader.read_to_end(&mut svm_classifier_bytes)?;

        // Deserialize the bytes using the `bincode` crate
        self.svm_classifier = bincode::deserialize(&svm_classifier_bytes)?;

        Ok(())
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

        // Extract Brief features from the image
        let hog_feature = BriefFeature::new();
        let hog_features = hog_feature.extract(&image).unwrap();

        // Create training data and labels
        let x = Array2::from_shape_vec((1, hog_features.len()), hog_features).unwrap();
        let y = Array1::from(vec![true]);

        let window_generator = SlidingWindow {
            width: 32,
            height: 32,
            step_size: 32,
        };
        // Create a new BriefSVMDetector and train it on the training data
        let mut hog_svm_detector = BriefSVMDetector::new(window_generator);
        hog_svm_detector.fit(&x.view(), &y.view()).unwrap();

        // Make predictions on the image using the trained detector
        let detections = hog_svm_detector.detect(&image);

        // Check that the predicted detection is correct
        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].class, 1);
        assert_eq!(detections[0].bbox, BBox::new(0, 0, 32, 32));
        assert_eq!(detections[0].confidence, 1.0);
    }

    #[test]
    fn test_default_hog_svm_detector() {
        // Create a simple image with a single black square
        let image = DynamicImage::new_luma8(32, 32);

        // Extract Brief features from the image
        let hog_feature = BriefFeature::new();
        let hog_features = hog_feature.extract(&image).unwrap();

        // Create training data and labels
        let x = Array2::from_shape_vec((1, hog_features.len()), hog_features).unwrap();
        let y = Array1::from(vec![true]);

        // Create a new BriefSVMDetector and train it on the training data
        let mut hog_svm_detector = BriefSVMDetector::default();
        hog_svm_detector.fit(&x.view(), &y.view()).unwrap();

        // Make predictions on the image using the trained detector
        let detections = hog_svm_detector.detect(&image);

        // Check that the predicted detection is correct
        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].class, 1);
        assert_eq!(detections[0].bbox, BBox::new(0, 0, 32, 32));
        assert_eq!(detections[0].confidence, 1.0);
    }

    #[test]
    fn test_persistent_detector() {
        // Create a simple image with a single black square
        let image = DynamicImage::new_luma8(32, 32);

        // Extract Brief features from the image
        let brief_feature = BriefFeature::new();
        let brief_features = brief_feature.extract(&image).unwrap();

        // Create training data and labels
        let x = Array2::from_shape_vec((1, brief_features.len()), brief_features).unwrap();
        let y = Array1::from(vec![true]);

        // Create a new BriefSVMDetector and train it on the training data
        let mut brief_svm_detector = BriefSVMDetector::default();
        brief_svm_detector.fit(&x.view(), &y.view()).unwrap();

        // Save the detector to a Vec<u8>
        let mut data = Vec::new();
        brief_svm_detector.save(&mut data).unwrap();

        // Load the detector from the Vec<u8>
        let mut loaded_detector = BriefSVMDetector::default();
        loaded_detector.load(&mut &data[..]).unwrap();

        // Ensure that the original and loaded detectors are equal
        assert_eq!(brief_svm_detector, loaded_detector);
    }
}
