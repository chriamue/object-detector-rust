#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod bbox;
pub mod classifier;
pub mod dataset;
/// Struct for representing a detected object
pub mod detection;
pub mod detector;
pub mod feature;
/// Trait for objects that can make predictions
pub mod predictable;
pub mod tests;
/// Trait for objects that can be trained
pub mod trainable;
pub mod types;
pub mod utils;

/// the object-detector-rust prelude
pub mod prelude {
    pub use super::bbox::BBox;
    pub use super::classifier::BayesClassifier;
    pub use super::classifier::Classifier;
    #[cfg(feature = "randomforest")]
    pub use super::classifier::RandomForestClassifier;
    pub use super::classifier::SVMClassifier;
    pub use super::dataset::AnnotatedImageSet;
    pub use super::dataset::DataSet;
    pub use super::dataset::MemoryDataSet;
    pub use super::detection::Detection;
    #[cfg(feature = "brief")]
    pub use super::detector::BriefSVMDetector;
    pub use super::detector::Detector;
    #[cfg(feature = "hog")]
    pub use super::detector::HOGSVMDetector;
    pub use super::detector::PersistentDetector;
    pub use super::feature::BriefFeature;
    pub use super::feature::Feature;
    pub use super::feature::HOGFeature;
    pub use super::predictable::Predictable;
    pub use super::trainable::Trainable;
    pub use super::types::AnnotatedImage;
    pub use super::types::Annotation;
    pub use super::types::Class;
}
