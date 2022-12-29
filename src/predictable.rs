/// Trait for objects that can make predictions
pub trait Predictable<'a, X, TX> {
    /// Makes predictions using the trained object on the given input data
    fn predict(&'a self, x: &'a X) -> Result<Vec<TX>, String>;
}
