use std::fs::File;

use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use image::{DynamicImage, RgbImage};
use object_detector_rust::prelude::*;
use object_detector_rust::utils::crop_bbox;

fn bench_hog_svm_detector(c: &mut Criterion) {
    // Create a detector and load it with a trained model
    let mut detector = HOGSVMDetector::default();
    let mut file = File::open("tests/hog_svm_detector.bin").unwrap();
    detector.load(&mut file).unwrap();

    // Load an image to detect objects in
    let image = image::open("tests/folder_dataset/data/webcam01.jpg").unwrap();

    // Benchmark the detect function
    c.bench_function("hog-svm-detect", move |b| {
        b.iter(|| detector.detect(black_box(&image)))
    });
}

fn bench_brief_svm_detector(c: &mut Criterion) {
    // Create a detector and load it with a trained model
    let mut detector = BriefSVMDetector::default();
    let mut file = File::open("tests/brief_svm_detector.bin").unwrap();
    detector.load(&mut file).unwrap();

    // Load an image to detect objects in
    let image = image::open("tests/folder_dataset/data/webcam01.jpg").unwrap();

    // Benchmark the detect function
    c.bench_function("brief-svm-detect", move |b| {
        b.iter(|| detector.detect(black_box(&image)))
    });
}

fn bench_crop_bbox(b: &mut Bencher) {
    let image = DynamicImage::ImageRgb8(RgbImage::new(100, 100));
    let bbox: BBox = BBox::new(20, 20, 32, 32);
    b.iter(|| {
        let _cropped = crop_bbox(&image, &bbox);
    })
}

fn bench_utils(c: &mut Criterion) {
    c.bench_function("crop_bbox", bench_crop_bbox);
}

criterion_group!(
    benches,
    bench_hog_svm_detector,
    bench_brief_svm_detector,
    bench_utils
);
criterion_main!(benches);
