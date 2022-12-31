use super::Classifier;
use crate::prelude::{Predictable, Trainable};
use linfa::prelude::Predict;
use linfa::prelude::*;
use linfa_svm::Svm;
use ndarray::Array1;
use ndarray::ArrayView1;
use ndarray::ArrayView2;
use serde::{Deserialize, Serialize};

/// The SVMClassifier struct represents an SVM classifier with a trained model.
/// The model is an option type, which means it is initially empty.
/// The new function creates a new SVMClassifier instance with an empty model.
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct SVMClassifier {
    /// The trained SVM model
    model: Option<Svm<f32, bool>>,
}

impl SVMClassifier {
    /// Creates a new `SVMClassifier`.
    pub fn new() -> Self {
        SVMClassifier { model: None }
    }
}

impl Trainable<f32, bool> for SVMClassifier {
    fn fit(&mut self, x: &ArrayView2<f32>, y: &ArrayView1<bool>) -> Result<(), String> {
        let dataset = Dataset::new(x.to_owned(), y.to_owned());

        let model = Svm::<_, bool>::params()
            .gaussian_kernel(80.0)
            .eps(0.1)
            .pos_neg_weights(5.0, 50.0)
            .fit(&dataset)
            .unwrap();

        self.model = Some(model);
        Ok(())
    }
}

impl Predictable<f32, bool> for SVMClassifier {
    fn predict(&self, x: &ArrayView2<f32>) -> Result<Array1<bool>, String> {
        Ok(self.model.as_ref().unwrap().predict(x))
    }
}

impl Classifier<f32, bool> for SVMClassifier {}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array;

    #[test]
    fn test_svm_classifier() {
        let mut classifier = SVMClassifier::new();
        let x: Vec<f32> = (0..30).map(|x| x as f32).collect();
        let x = Array::from_shape_vec((10, 3), x).unwrap();
        let y = Array::from_vec(vec![
            true, false, true, true, false, true, false, true, true, false,
        ]);
        classifier.fit(&x.view(), &y.view()).unwrap();
        let test_x: Vec<f32> = (0..6).map(|x| x as f32).collect();
        let test_x = Array::from_shape_vec((2, 3), test_x).unwrap();
        let expected_y = Array::from_vec(vec![true, false]);
        let predicted_y = classifier.predict(&test_x.view()).unwrap();
        assert_eq!(2, predicted_y.len());

        assert_eq!(expected_y, predicted_y);
    }
}
