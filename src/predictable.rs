use ndarray::{Array1, ArrayView2};

/// Trait for objects that can make predictions
pub trait Predictable<X, Y> {
    /// Makes predictions on the provided data using the trained `SVMClassifier`.
    ///
    /// # Arguments
    ///
    /// * `x` - A 2D array of data to make predictions on.
    ///
    /// # Returns
    ///
    /// A `Result` containing an error string if prediction fails, or a 1D array of predictions if
    /// prediction succeeds.
    fn predict(&self, x: &ArrayView2<X>) -> Result<Array1<Y>, String>;
}
