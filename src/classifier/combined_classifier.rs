use std::marker::PhantomData;

use linfa::{Float, Label};
use ndarray::{Array1, ArrayView1, ArrayView2};
use serde::{ser::SerializeStruct, ser::Serializer, Deserialize, Deserializer, Serialize};

use crate::prelude::{Predictable, Trainable};

use super::Classifier;

/// A classifier that combines the predictions of two other classifiers.
///
/// # Type Parameters
///
/// * X - The type of the features in the training and prediction data.
/// * Y - The type of the labels in the training and prediction data.
/// * C1 - The type of the first classifier.
/// * C2 - The type of the second classifier.
#[derive(Clone, Debug, PartialEq)]
pub struct CombinedClassifier<X, Y, C1, C2>
where
    X: Float,
    Y: Label,
    C1: Classifier<X, Y>,
    C2: Classifier<X, Y>,
{
    x: PhantomData<*const X>,
    y: PhantomData<*const Y>,
    classifier1: C1,
    classifier2: C2,
}

impl<X, Y, C1, C2> CombinedClassifier<X, Y, C1, C2>
where
    X: Float,
    Y: Label,
    C1: Classifier<X, Y>,
    C2: Classifier<X, Y>,
{
    /// Creates a new CombinedClassifier
    ///
    /// # Arguments
    ///
    /// * classifier1 - The first classifier to use in the combination.
    /// * classifier2 - The second classifier to use in the combination.
    ///
    /// # Returns
    ///
    /// A new CombinedClassifier object.
    pub fn new(classifier1: C1, classifier2: C2) -> Self {
        Self {
            classifier1,
            classifier2,
            x: PhantomData,
            y: PhantomData,
        }
    }
}

impl<X, Y, C1, C2> Default for CombinedClassifier<X, Y, C1, C2>
where
    X: Float,
    Y: Label,
    C1: Classifier<X, Y> + Default,
    C2: Classifier<X, Y> + Default,
{
    fn default() -> Self {
        Self::new(C1::default(), C2::default())
    }
}

impl<X, Y, C1, C2> Trainable<X, Y> for CombinedClassifier<X, Y, C1, C2>
where
    X: Float,
    Y: Label,
    C1: Classifier<X, Y>,
    C2: Classifier<X, Y>,
{
    fn fit(&mut self, x: &ArrayView2<X>, y: &ArrayView1<Y>) -> Result<(), String> {
        // Train the first classifier
        self.classifier1.fit(x, y)?;

        // Train the second classifier
        self.classifier2.fit(x, y)?;

        Ok(())
    }
}

impl<X, Y, C1, C2> Predictable<X, Y> for CombinedClassifier<X, Y, C1, C2>
where
    X: Float,
    Y: Label + Copy,
    C1: Classifier<X, Y>,
    C2: Classifier<X, Y>,
{
    fn predict(&self, x: &ArrayView2<X>) -> Result<Array1<Y>, String> {
        // Make predictions using the first classifier
        let predictions1 = self.classifier1.predict(x)?;

        // Make predictions using the second classifier
        let predictions2 = self.classifier2.predict(x)?;

        // Combine the predictions by taking the intersection
        let combined_predictions = predictions1
            .iter()
            .zip(predictions2.iter())
            .map(|(p1, p2)| if p1 == p2 { *p1 } else { Y::default() })
            .collect();

        Ok(combined_predictions)
    }
}

impl<X, Y, C1, C2> Classifier<X, Y> for CombinedClassifier<X, Y, C1, C2>
where
    X: Float,
    Y: Label + Copy,
    C1: Classifier<X, Y>,
    C2: Classifier<X, Y>,
{
}

impl<X, Y, C1, C2> Serialize for CombinedClassifier<X, Y, C1, C2>
where
    X: Float,
    Y: Label,
    C1: Classifier<X, Y> + Serialize,
    C2: Classifier<X, Y> + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CombinedClassifier", 2)?;
        state.serialize_field("classifier1", &self.classifier1)?;
        state.serialize_field("classifier2", &self.classifier2)?;
        state.end()
    }
}

impl<'de, X, Y, C1, C2> Deserialize<'de> for CombinedClassifier<X, Y, C1, C2>
where
    X: Float,
    Y: Label,
    C1: Classifier<X, Y> + Deserialize<'de>,
    C2: Classifier<X, Y> + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CombinedClassifierData<C1, C2> {
            classifier1: C1,
            classifier2: C2,
        }

        let combined_classifier_data = CombinedClassifierData::deserialize(deserializer)?;
        Ok(CombinedClassifier {
            x: PhantomData,
            y: PhantomData,
            classifier1: combined_classifier_data.classifier1,
            classifier2: combined_classifier_data.classifier2,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{BayesClassifier, RandomForestClassifier, SVMClassifier};
    use ndarray::array;

    #[test]
    fn test_combined_classifier() {
        // Create an instance of CombinedClassifier using two classifier implementations
        let mut combined_classifier: CombinedClassifier<
            f32,
            bool,
            SVMClassifier<_, _>,
            BayesClassifier<_, _>,
        > = CombinedClassifier::default();

        // Generate training data and labels
        let x_train = array![[1., 1.], [0., 2.], [1., 3.], [1., 4.], [0., 5.], [1., 6.]];
        let y_train = array![true, false, true, true, false, true];

        // Fit the CombinedClassifier using the training data and labels
        combined_classifier
            .fit(&x_train.view(), &y_train.view())
            .unwrap();

        // Generate test data
        let x_test = array![[1., 1.], [0., 2.]];

        // Make predictions using the CombinedClassifier
        let y_pred = combined_classifier.predict(&x_test.view()).unwrap();

        // Assert that the predictions are correct
        assert_eq!(y_pred, array![true, false]);
    }

    #[test]
    fn test_combined_classifier_serialization() {
        // Create an instance of CombinedClassifier using two classifier implementations
        let combined_classifier: CombinedClassifier<
            f32,
            usize,
            RandomForestClassifier<_, _>,
            RandomForestClassifier<_, _>,
        > = CombinedClassifier::default();

        let serialized = serde_json::to_string(&combined_classifier).unwrap();

        let deserialized: CombinedClassifier<
            f32,
            usize,
            RandomForestClassifier<f32, usize>,
            RandomForestClassifier<f32, usize>,
        > = serde_json::from_str(&serialized).unwrap();

        assert_eq!(combined_classifier, deserialized);
    }
}
