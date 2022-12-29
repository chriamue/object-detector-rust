/// Trait for objects that can be trained
pub trait Trainable<'a, X, Y, TX, TY> {
    /// Trains the object using the given input data and labels
    fn fit(&'a mut self, x: &'a X, y: &'a Y) -> Result<(), String>;
}
