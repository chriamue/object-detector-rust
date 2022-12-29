//! This module contains the `DataSet` trait for representing datasets of images and labels for training.
//!
//! # Examples
//!
//! ```
//! use object_detector_rust::dataset::DataSet;
//! use object_detector_rust::dataset::DummyDataSet;
//!
//! let mut dataset = DummyDataSet::new();
//! match dataset.load() {
//!     Ok(_) => {
//!         let (images, labels) = dataset.get_data();
//!         // Use the images and labels to train a classifier
//!     },
//!     Err(e) => println!("Error loading dataset: {}", e),
//! }
//! ```
//!
//! This example shows how to use the `DataSet` trait to retrieve images and labels for training a classifier.

use image::DynamicImage;
use std::error::Error;

mod folder_dataset;
pub use folder_dataset::FolderDataSet;

mod memory_dataset;
pub use memory_dataset::MemoryDataSet;

use crate::AnnotatedImage;

/// Trait representing a dataset of images and labels for training
pub trait DataSet {
    /// Returns the number of images in the dataset
    fn len(&self) -> usize;

    /// Returns the images and labels for training
    fn get_data(&self) -> (Vec<DynamicImage>, Vec<u32>);

    /// Loads the dataset
    fn load(&mut self) -> Result<(), Box<dyn Error>>;
}

/// trait of a set of annotated images
pub trait AnnotatedImageSet {
    /// adds an annotated image
    fn add_annotated_image(&mut self, annotated_image: AnnotatedImage);
    /// returns length of annotated images
    fn annotated_images_len(&self) -> usize;
    /// returns iterator over annotated images
    fn annotated_images(&self) -> Box<dyn Iterator<Item = &AnnotatedImage> + '_>;
}

/// Dummy dataset that returns a fixed set of images and labels
#[derive(Default)]
pub struct DummyDataSet {
    images: Vec<DynamicImage>,
    labels: Vec<u32>,
}

impl DummyDataSet {
    /// Creates a new `DummyDataSet`
    pub fn new() -> Self {
        Self {
            images: vec![],
            labels: vec![],
        }
    }
}

impl DataSet for DummyDataSet {
    fn len(&self) -> usize {
        self.images.len()
    }

    fn get_data(&self) -> (Vec<DynamicImage>, Vec<u32>) {
        (self.images.clone(), self.labels.clone())
    }

    fn load(&mut self) -> Result<(), Box<dyn Error>> {
        // Load the dummy data into the images and labels vectors
        self.images = vec![DynamicImage::new_rgba8(1, 1), DynamicImage::new_rgba8(1, 1)];
        self.labels = vec![0, 1];
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::DynamicImage;

    #[test]
    fn test_dummy_dataset() {
        let mut dataset = DummyDataSet::new();
        assert_eq!(dataset.len(), 0);
        assert_eq!(dataset.get_data(), (vec![], vec![]));

        dataset.load().unwrap();
        assert_eq!(dataset.len(), 2);
        assert_eq!(
            dataset.get_data(),
            (
                vec![DynamicImage::new_rgba8(1, 1), DynamicImage::new_rgba8(1, 1)],
                vec![0, 1]
            )
        );
    }
}
