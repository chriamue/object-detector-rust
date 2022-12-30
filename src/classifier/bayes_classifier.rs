use super::Classifier;
use crate::prelude::Predictable;
use crate::prelude::Trainable;
use linfa::prelude::{Fit, Predict};
use linfa::Dataset;
use linfa_bayes::GaussianNb;
use ndarray::Array1;
use ndarray::ArrayView1;
use ndarray::ArrayView2;

/// The `BayesClassifier` struct provides a Gaussian Naive Bayes classifier.
///
/// This classifier is implemented using the `linfa_bayes` crate
/// and can be trained on a given dataset to classify future data points.
#[derive(Default, Debug)]
pub struct BayesClassifier {
    model: Option<GaussianNb<f32, usize>>,
}

impl BayesClassifier {
    /// Creates a new `BayesClassifier` instance with default parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use object_detector_rust::classifier::BayesClassifier;
    ///
    /// let bayes_classifier = BayesClassifier::new();
    /// ```
    pub fn new() -> Self {
        BayesClassifier { model: None }
    }
}

impl Trainable<f32, usize> for BayesClassifier {
    fn fit(&mut self, x: &ArrayView2<f32>, y: &ArrayView1<usize>) -> Result<(), String> {
        let dataset = Dataset::new(x.to_owned(), y.to_owned());
        let model = GaussianNb::<_, usize>::params().fit(&dataset).unwrap();
        self.model = Some(model);
        Ok(())
    }
}

impl Predictable<f32, usize> for BayesClassifier {
    fn predict(&self, x: &ArrayView2<f32>) -> Result<Array1<usize>, String> {
        Ok(self.model.as_ref().unwrap().predict(x))
    }
}

impl Classifier<f32, usize> for BayesClassifier {}

#[cfg(test)]
mod tests {
    use ndarray::{arr1, arr2};

    use super::*;

    #[test]
    fn test_bayes_classifier() {
        let mut classifier = BayesClassifier::new();

        let x = arr2(&[[1., 1.], [1., 2.], [2., 1.], [2., 2.]]);
        let y = arr1(&[1, 1, 0, 0]);
        assert!(classifier.fit(&x.view(), &y.view()).is_ok());

        let x_pred = arr2(&[[1., 1.], [1., 2.], [2., 1.], [2., 2.]]);
        let y_pred = arr1(&[1, 1, 0, 0]);
        let y_pred_res = classifier.predict(&x_pred.view()).unwrap();
        assert_eq!(y_pred, y_pred_res);
    }
}
