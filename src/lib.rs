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
/// Trait for objects that can be trained
pub mod trainable;
pub mod utils;

pub use bbox::BBox;

/// object class type
pub type Class = u32;

/// annotation is a object bounding box in image and class type
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    /// Bounding box of the object
    pub bbox: BBox,
    /// Class of the object
    pub class: Class,
}

/// Image annotated by list of Annotations
#[derive(Debug, Clone, PartialEq)]
pub struct AnnotatedImage {
    /// Image
    pub image: image::DynamicImage,
    /// Annotations for the image
    pub annotations: Vec<Annotation>,
}

/// the object-detector-rust prelude
pub mod prelude {
    pub use super::bbox::BBox;
    pub use super::classifier::BayesClassifier;
    pub use super::classifier::Classifier;
    #[cfg(feature = "randomforest")]
    pub use super::classifier::RandomForestClassifier;
    pub use super::classifier::SVMClassifier;
    pub use super::detection::Detection;
    #[cfg(feature = "brief")]
    pub use super::detector::BriefSVMDetector;
    pub use super::detector::Detector;
    #[cfg(feature = "hog")]
    pub use super::detector::HOGSVMDetector;
    pub use super::detector::PersistentDetector;
    pub use super::feature::Feature;
    pub use super::predictable::Predictable;
    pub use super::trainable::Trainable;
    pub use super::AnnotatedImage;
    pub use super::Annotation;
    pub use super::Class;
}
