use super::Classifier;
use crate::prelude::{Predictable, Trainable};
use linfa::prelude::Predict;
use linfa::prelude::*;
use linfa::Label;
use linfa_svm::Svm;
use ndarray::Array1;
use ndarray::ArrayView1;
use ndarray::ArrayView2;
use serde::{Deserialize, Serialize};

/// The SVMClassifier struct represents an SVM classifier with a trained model.
/// The model is an option type, which means it is initially empty.
/// The new function creates a new SVMClassifier instance with an empty model.
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct SVMClassifier<X, Y>
where
    X: Float,
    Y: Label,
{
    /// The trained SVM model
    model: Option<Svm<X, Y>>,
}

impl<X, Y> SVMClassifier<X, Y>
where
    X: Float,
    Y: Label,
{
    /// Creates a new `SVMClassifier`.
    pub fn new() -> Self {
        SVMClassifier { model: None }
    }
}

impl Trainable<f32, bool> for SVMClassifier<f32, bool> {
    fn fit(&mut self, x: &ArrayView2<f32>, y: &ArrayView1<bool>) -> Result<(), String> {
        let dataset = Dataset::new(x.to_owned(), y.to_owned());

        let model = Svm::<_, bool>::params()
            .gaussian_kernel(80.0)
            .eps(0.1)
            .pos_neg_weights(50.0, 50.0)
            .fit(&dataset)
            .unwrap();

        self.model = Some(model);
        Ok(())
    }
}

impl Predictable<f32, bool> for SVMClassifier<f32, bool> {
    fn predict(&self, x: &ArrayView2<f32>) -> Result<Array1<bool>, String> {
        Ok(self.model.as_ref().unwrap().predict(x))
    }
}

impl Classifier<f32, bool> for SVMClassifier<f32, bool> {}

#[cfg(test)]
mod tests {
    use crate::utils::{evaluate_accuracy, evaluate_precision};

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
        assert!(evaluate_accuracy(&classifier, &test_x, &expected_y) > 0.5);
        assert!(evaluate_precision(&classifier, &test_x, &expected_y) > 0.8);
        let predicted_y = classifier.predict(&x.view()).unwrap();
        let cm = predicted_y.confusion_matrix(&y).unwrap();
        println!("{:?}", cm);
        assert!(cm.precision() > 0.8);
        assert!(cm.accuracy() > 0.5);
    }

    #[test]
    fn test_partial_eq() {
        let classifier = SVMClassifier::<f32, usize>::new();
        let classifier2 = classifier.clone();
        assert!(classifier.eq(&classifier2));
    }
}
