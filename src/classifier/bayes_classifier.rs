use super::Classifier;
use crate::prelude::Predictable;
use crate::prelude::Trainable;
use linfa::prelude::{Fit, Predict};
use linfa::Dataset;
use linfa::Float;
use linfa::Label;
use linfa_bayes::GaussianNb;
use ndarray::Array1;
use ndarray::ArrayView1;
use ndarray::ArrayView2;

/// The `BayesClassifier` struct provides a Gaussian Naive Bayes classifier.
///
/// This classifier is implemented using the `linfa_bayes` crate
/// and can be trained on a given dataset to classify future data points.
#[derive(Default, Debug, PartialEq)]
pub struct BayesClassifier<X, Y>
where
    X: Float,
    Y: Label,
{
    model: Option<GaussianNb<X, Y>>,
}

impl<X, Y> BayesClassifier<X, Y>
where
    X: Float,
    Y: Label,
{
    /// Creates a new `BayesClassifier` instance with default parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use object_detector_rust::classifier::BayesClassifier;
    ///
    /// let bayes_classifier = BayesClassifier::<f32, bool>::new();
    /// ```
    pub fn new() -> Self {
        BayesClassifier { model: None }
    }
}

impl<X, Y> Trainable<X, Y> for BayesClassifier<X, Y>
where
    X: Float,
    Y: Label,
{
    fn fit(&mut self, x: &ArrayView2<X>, y: &ArrayView1<Y>) -> Result<(), String> {
        let dataset = Dataset::new(x.to_owned(), y.to_owned());
        let model = GaussianNb::<X, Y>::params().fit(&dataset).unwrap();
        self.model = Some(model);
        Ok(())
    }
}

impl<X, Y> Predictable<X, Y> for BayesClassifier<X, Y>
where
    X: Float,
    Y: Label,
{
    fn predict(&self, x: &ArrayView2<X>) -> Result<Array1<Y>, String> {
        Ok(self.model.as_ref().unwrap().predict(x))
    }
}

impl<X, Y> Classifier<X, Y> for BayesClassifier<X, Y>
where
    X: Float,
    Y: Label,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use linfa::prelude::ToConfusionMatrix;
    use ndarray::{arr1, arr2};

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
        let predicted_y = classifier.predict(&x.view()).unwrap();
        let cm = predicted_y.confusion_matrix(&y).unwrap();
        println!("{:?}", cm);
        assert!(cm.precision() > 0.8);
        assert!(cm.accuracy() > 0.5);
    }
}
