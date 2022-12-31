use object_detector_rust::{
    dataset::{DataSet, FolderDataSet},
    detector::{Detector, HOGSVMDetector},
    feature::{Feature, HOGFeature},
    utils::{draw_bboxes, extract_data},
    BBox, Class,
};

fn main() -> Result<(), String> {
    let class: Class = 1;
    // Set the path to the test data folder
    let data_path = std::fs::canonicalize("tests/folder_dataset/data").unwrap();
    let labels_path = std::fs::canonicalize("tests/folder_dataset/data/labels.txt").unwrap();
    let label_names = FolderDataSet::load_label_names(labels_path.to_str().unwrap());
    // Create a new instance of the FolderDataSet struct
    let dataset = FolderDataSet::new(data_path.to_str().unwrap(), 32, 32, label_names);

    let (x, y) = dataset.get_data();
    let feature = HOGFeature::default();
    let x: Vec<f32> = x
        .iter()
        .flat_map(|image| feature.extract(image).unwrap())
        .collect();
    let y = y
        .iter()
        .map(|y| if *y == class { true } else { false })
        .collect();
    let (x, y) = extract_data(x, y);

    // Create a HOGSVMDetector and train it on the training data
    let mut detector = HOGSVMDetector::new();
    detector.fit(&x.view(), &y.view())?;

    // Load an image to run detection on
    let mut image = image::open("tests/folder_dataset/data/webcam01.jpg").unwrap();

    // Run detection on the image
    let detections = detector.detect(&image);
    let bboxes: Vec<BBox> = detections.iter().map(|d| d.bbox.clone()).collect();
    // Draw the bounding boxes on the image
    draw_bboxes(&mut image, &bboxes);

    // Save the image with detections to a file
    image.save("detected_image.jpg").unwrap();

    Ok(())
}
