use linfa::metrics::ToConfusionMatrix;
use linfa::Float;
use linfa::Label;
use ndarray::Array1;
use ndarray::Array2;

use crate::prelude::Predictable;

/// Computes the accuracy of a model's predictions
///
/// # Arguments
///
/// * `model` - A reference to the model being evaluated. It must implement the `Predictable` trait.
/// * `x` - A 2D array containing the input data used to make the predictions
/// * `y` - A 1D array containing the true labels for the input data
///
/// # Returns
///
/// A floating-point value representing the accuracy of the model's predictions.
///
pub fn evaluate_accuracy<X, Y>(model: &dyn Predictable<X, Y>, x: &Array2<X>, y: &Array1<Y>) -> f32
where
    X: Float,
    Y: Label,
{
    let predicted_y = model.predict(&x.view()).unwrap();
    let cm = predicted_y.confusion_matrix(y).unwrap();
    cm.accuracy()
}

/// Computes the precision of a model's predictions
///
/// # Arguments
///
/// * `model` - A reference to the model being evaluated. It must implement the `Predictable` trait.
/// * `x` - A 2D array containing the input data used to make the predictions
/// * `y` - A 1D array containing the true labels for the input data
///
/// # Returns
///
/// A floating-point value representing the precision of the model's predictions.
///
pub fn evaluate_precision<X, Y>(model: &dyn Predictable<X, Y>, x: &Array2<X>, y: &Array1<Y>) -> f32
where
    X: Float,
    Y: Label,
{
    let predicted_y = model.predict(&x.view()).unwrap();
    let cm = predicted_y.confusion_matrix(y).unwrap();
    cm.precision()
}
