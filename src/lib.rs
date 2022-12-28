#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod bbox;
pub mod dataset;
pub mod feature;
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
    pub use super::AnnotatedImage;
    pub use super::Annotation;
    pub use super::Class;
}
