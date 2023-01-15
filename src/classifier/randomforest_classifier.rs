use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use serde::{Deserialize, Serialize};
use smartcore::{
    ensemble::random_forest_classifier::RandomForestClassifier as RFC,
    numbers::{basenum::Number, floatnum::FloatNumber},
};

use crate::prelude::{Predictable, Trainable};

use super::Classifier;

/// A struct for training and making predictions with a random forest classifier.
#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct RandomForestClassifier<X, Y>
where
    X: Number + FloatNumber + PartialOrd,
    Y: Number + Ord,
{
    model: Option<RFC<X, Y, Array2<X>, Array1<Y>>>,
}

impl<X, Y> RandomForestClassifier<X, Y>
where
    X: Number + FloatNumber + PartialOrd,
    Y: Number + Ord,
{
    /// Creates a new instance of the `RandomForestClassifier` struct.
    ///
    /// # Returns
    ///
    /// A new instance of the `RandomForestClassifier` struct with the `model` field set to `None`.
    pub fn new() -> Self {
        Self { model: None }
    }
}

impl<X, Y> Trainable<X, Y> for RandomForestClassifier<X, Y>
where
    X: Number + FloatNumber + PartialOrd,
    Y: Number + Ord,
{
    fn fit(&mut self, x: &ArrayView2<X>, y: &ArrayView1<Y>) -> Result<(), String> {
        let x = x.to_owned();
        let y = y.to_owned();
        let model = RFC::fit(&x, &y, Default::default()).unwrap();
        self.model = Some(model);
        Ok(())
    }
}

impl<X, Y> Predictable<X, Y> for RandomForestClassifier<X, Y>
where
    X: Number + FloatNumber + PartialOrd,
    Y: Number + Ord,
{
    fn predict(&self, x: &ArrayView2<X>) -> Result<Array1<Y>, String> {
        let x = x.to_owned();
        let model = self.model.as_ref().unwrap();
        let y = model.predict(&x).unwrap();
        Ok(y)
    }
}

impl<X, Y> Classifier<X, Y> for RandomForestClassifier<X, Y>
where
    X: Number + FloatNumber + PartialOrd,
    Y: Number + Ord,
{
}

#[cfg(test)]
mod tests {
    use crate::utils::{evaluate_accuracy, evaluate_precision};

    use super::*;
    use linfa::prelude::ToConfusionMatrix;
    use ndarray::Array;

    #[test]
    fn test_svm_classifier() {
        let mut classifier = RandomForestClassifier::new();
        let x: Vec<f32> = (0..30).map(|x| x as f32).collect();
        let x = Array::from_shape_vec((10, 3), x).unwrap();
        let y = Array::from_vec(vec![1, 0, 1, 1, 0, 1, 0, 1, 1, 0]);
        classifier.fit(&x.view(), &y.view()).unwrap();
        let test_x: Vec<f32> = (0..6).map(|x| x as f32).collect();
        let test_x = Array::from_shape_vec((2, 3), test_x).unwrap();
        let expected_y = Array::from_vec(vec![1, 0]);
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
        let classifier = RandomForestClassifier::<f32, usize>::new();
        let classifier2 = RandomForestClassifier::<f32, usize>::new();
        assert!(classifier.eq(&classifier2));
    }
}
