use crate::bbox::BBox;
use crate::dataset::DataSet;
use crate::utils::crop_bbox;
use crate::{AnnotatedImage, Annotation, Class};
use image::{open, DynamicImage};
use std::error::Error;
use std::fs::{read_dir, File};
use std::io::{self, BufRead};
use std::path::Path;

/// Struct for representing a dataset loaded from a folder
pub struct FolderDataSet {
    /// Path to the folder containing the dataset
    path: String,
    /// Default width for extracting image patches
    default_width: u32,
    /// Default height for extracting image patches
    default_height: u32,
    /// Vector of class names in the dataset
    names: Vec<String>,
    /// Vector of image patches and labels in the dataset
    data: Vec<(String, DynamicImage)>,
    /// Vector of images with annotations in the dataset
    annotated_images: Vec<AnnotatedImage>,
}

impl FolderDataSet {
    /// Creates a new dataset from the specified folder
    pub fn new(path: &str, default_width: u32, default_height: u32, names: Vec<String>) -> Self {
        Self {
            path: path.to_string(),
            default_width,
            default_height,
            names,
            data: vec![],
            annotated_images: vec![],
        }
    }

    /// Loads a list of label names from the specified file path.
    pub fn load_label_names(path: &str) -> Vec<String> {
        let file = File::open(path).unwrap();
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .collect()
    }

    fn list_pathes(path: &str) -> Vec<(String, String)> {
        let mut file_pathes = Vec::new();
        for entry in read_dir(path).unwrap() {
            let path = entry.unwrap();
            if path.path().to_str().unwrap().ends_with(".jpg") {
                let image_path = path.path();
                let image_path = image_path.as_path().to_str().unwrap();
                let labels_path = image_path.replace("jpg", "txt");
                file_pathes.push((labels_path.to_string(), image_path.to_string()));
            }
        }
        file_pathes
    }

    fn load_annotation(
        image_path: String,
        label: String,
        x: u32,
        y: u32,
        default_width: u32,
        default_height: u32,
    ) -> (String, DynamicImage) {
        let img = open(image_path).unwrap();
        let bbox = BBox {
            x: x as i32,
            y: y as i32,
            width: default_width,
            height: default_height,
        };
        let window = crop_bbox(&img, &bbox);
        (label, window)
    }

    fn load_annotated_images(
        pathes: &Vec<(String, String)>,
        default_width: u32,
        default_height: u32,
        class_names: &Vec<String>,
    ) -> Vec<AnnotatedImage> {
        let mut annotated_images: Vec<AnnotatedImage> = Vec::new();
        for path in pathes {
            let file = File::open(&path.0).unwrap();
            let img = open(&path.1).unwrap();
            let mut annotations = Vec::new();
            for line in io::BufReader::new(file).lines() {
                match line {
                    Ok(line) => {
                        let mut l = line.split(' ');
                        let label = l.next().unwrap();
                        let x: u32 = l.next().unwrap().parse().unwrap();
                        let y: u32 = l.next().unwrap().parse().unwrap();
                        let bbox = BBox {
                            x: x as i32,
                            y: y as i32,
                            width: default_width,
                            height: default_height,
                        };
                        let class = Self::label_id(label, &class_names);

                        annotations.push(Annotation { bbox, class });
                    }
                    _ => (),
                }
            }
            annotated_images.push(AnnotatedImage {
                image: img,
                annotations,
            })
        }
        annotated_images
    }

    fn load_annotations(
        pathes: &Vec<(String, String)>,
        default_width: u32,
        default_height: u32,
    ) -> Vec<(String, DynamicImage)> {
        let mut annotations = Vec::new();
        for path in pathes {
            let file = File::open(&path.0).unwrap();
            for line in io::BufReader::new(file).lines() {
                match line {
                    Ok(line) => {
                        let mut l = line.split(' ');
                        let label = l.next().unwrap();
                        let x: u32 = l.next().unwrap().parse().unwrap();
                        let y: u32 = l.next().unwrap().parse().unwrap();
                        annotations.push(Self::load_annotation(
                            path.1.clone(),
                            label.to_string(),
                            x,
                            y,
                            default_width,
                            default_height,
                        ));
                    }
                    _ => (),
                }
            }
        }
        annotations
    }

    fn label_id(label: &str, labels: &[String]) -> Class {
        labels.iter().position(|x| x == label).unwrap() as Class
    }
}

impl DataSet for FolderDataSet {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn get_data(&self) -> (Vec<DynamicImage>, Vec<u32>) {
        let x = self.data.iter().map(|(_, img)| img.clone()).collect();
        let y = self
            .data
            .iter()
            .map(|(label, _)| Self::label_id(label, &self.names))
            .collect();
        (x, y)
    }

    fn load(&mut self) -> Result<(), Box<dyn Error>> {
        let path = Path::new(&self.path);
        if !path.is_dir() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid folder path",
            )));
        }
        let pathes = Self::list_pathes(&self.path);
        self.data = Self::load_annotations(&pathes, self.default_width, self.default_height);
        self.annotated_images = Self::load_annotated_images(
            &pathes,
            self.default_width,
            self.default_height,
            &self.names,
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folder_dataset() {
        // Set the path to the test data folder
        let data_path = std::fs::canonicalize("tests/folder_dataset/data").unwrap();
        let labels_path = std::fs::canonicalize("tests/folder_dataset/data/labels.txt").unwrap();
        let label_names = FolderDataSet::load_label_names(labels_path.to_str().unwrap());
        // Create a new instance of the FolderDataSet struct
        let mut dataset = FolderDataSet::new(data_path.to_str().unwrap(), 64, 64, label_names);

        // Load the dataset
        dataset.load().unwrap();

        // Test the length of the dataset
        assert_eq!(dataset.len(), 42);
    }
}
