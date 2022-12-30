use ndarray::{ArrayView1, ArrayView2};

/// Trait for objects that can be trained
pub trait Trainable<X, Y> {
    /// Trains the `SVMClassifier` on the provided training data.
    ///
    /// # Arguments
    ///
    /// * `x` - A 2D array of training data features.
    /// * `y` - A 1D array of training data labels.
    ///
    /// # Returns
    ///
    /// A `Result` containing an error string if training fails, or `Ok` if training succeeds.
    fn fit(&mut self, x: &ArrayView2<X>, y: &ArrayView1<Y>) -> Result<(), String>;
}
