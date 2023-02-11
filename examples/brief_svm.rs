#[cfg(not(feature = "brief"))]
fn main() {
    eprintln!("example needs brief feature: cargo run --features brief --example brief_svm");
}

#[cfg(feature = "brief")]
fn main() -> Result<(), String> {
    use std::fs::File;

    use object_detector_rust::{
        bbox::BBox,
        dataset::{DataSet, FolderDataSet},
        detector::{BriefSVMDetector, Detector, PersistentDetector},
        feature::{BriefFeature, Feature},
        types::Class,
        utils::{draw_bboxes, extract_data},
        window_generator::SlidingWindow,
    };

    let class: Class = 5;
    // Set the path to the test data folder
    let data_path = std::fs::canonicalize("tests/folder_dataset/data").unwrap();
    let labels_path = std::fs::canonicalize("tests/folder_dataset/data/labels.txt").unwrap();
    let label_names = FolderDataSet::load_label_names(labels_path.to_str().unwrap());
    // Create a new instance of the FolderDataSet struct
    let mut dataset = FolderDataSet::new(data_path.to_str().unwrap(), 32, 32, label_names);
    dataset.load().unwrap();
    let (x, y) = dataset.get_data();
    let feature = BriefFeature::default();
    let x: Vec<Vec<f32>> = x
        .iter()
        .map(|image| feature.extract(image).unwrap())
        .collect();
    let y = y
        .iter()
        .map(|y| if *y == class { true } else { false })
        .collect();
    let (x, y) = extract_data(x, y);

    let window_generator = SlidingWindow {
        width: 32,
        height: 32,
        step_size: 32,
    };
    // Create a HOGSVMDetector and train it on the training data
    let mut detector = BriefSVMDetector::new(window_generator);
    detector.fit(&x.view(), &y.view())?;

    let mut file = File::create("tests/brief_svm_detector.bin").unwrap();
    detector.save(&mut file).unwrap();

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
