//! The classifier module provides traits and implementations for object classification algorithms.
//!
//! The Trainable trait allows models to be trained on input data and labels.
//! The Predictable trait allows trained models to make predictions on new input data.
//! The Classifier trait combines both Trainable and Predictable into a single trait for
//!
use crate::prelude::{Predictable, Trainable};

mod bayes_classifier;
pub use bayes_classifier::BayesClassifier;
mod svm_classifier;
pub use svm_classifier::SVMClassifier;
#[cfg(feature = "randomforest")]
mod randomforest_classifier;
#[cfg(feature = "randomforest")]
pub use randomforest_classifier::RandomForestClassifier;

/// The Classifier trait defines the behavior of a machine learning model
/// that is capable of classifying data points into one of several categories.
/// This trait is composed of two other traits: Trainable and Predictable.
pub trait Classifier<X, Y>: Trainable<X, Y> + Predictable<X, Y> {}
