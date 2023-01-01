use std::fs::File;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use object_detector_rust::prelude::*;

fn bench_detect(c: &mut Criterion) {
    // Create a detector and load it with a trained model
    let mut detector = HOGSVMDetector::default();
    let mut file = File::open("tests/hog_svm_detector.bin").unwrap();
    detector.load(&mut file).unwrap();

    // Load an image to detect objects in
    let image = image::open("tests/folder_dataset/data/webcam01.jpg").unwrap();

    // Benchmark the detect function
    c.bench_function("detect", move |b| {
        b.iter(|| detector.detect(black_box(&image)))
    });
}

criterion_group!(benches, bench_detect);
criterion_main!(benches);
