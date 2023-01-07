use crate::{bbox::BBox, types::Class};

/// Struct for representing a detected object
#[derive(Debug, Clone, PartialEq)]
pub struct Detection {
    /// Bounding box of the detected object
    pub bbox: BBox,
    /// Class of the detected object
    pub class: Class,
    /// Confidence of the detection
    pub confidence: f32,
}

impl Detection {
    /// Creates a new `Detection` with the given bounding box, class, and confidence.
    ///
    /// # Example
    ///
    /// ```
    /// use object_detector_rust::prelude::{Detection, BBox, Class};
    ///
    /// let bbox = BBox { x: 10, y: 20, width: 30, height: 40 };
    /// let class = 1;
    /// let confidence = 0.9;
    /// let detection = Detection::new(bbox.clone(), class, confidence);
    ///
    /// assert_eq!(detection.bbox, bbox);
    /// assert_eq!(detection.class, class);
    /// assert_eq!(detection.confidence, confidence);
    /// ```
    pub fn new(bbox: BBox, class: Class, confidence: f32) -> Detection {
        Detection {
            bbox,
            class,
            confidence,
        }
    }
}

/// Merges overlapping detections with the same class
pub fn merge_overlapping_detections(detections: &Vec<Detection>) -> Vec<Detection> {
    let mut merged_detections: Vec<Detection> = Vec::new();

    // Iterate over the detections
    for i in 0..detections.len() {
        let mut overlapping_detection_found = false;

        // Check if the current detection overlaps with any of the merged detections
        for merged_detection in &mut merged_detections {
            if detections[i].bbox.overlap(&merged_detection.bbox) > 0.0
                && detections[i].class == merged_detection.class
            {
                // If it does, create a new bounding box that is the union of the two bounding boxes
                let bbox = BBox::merge(&detections[i].bbox, &merged_detection.bbox);
                // Replace the original merged detection with the new one
                *merged_detection =
                    Detection::new(bbox, merged_detection.class, merged_detection.confidence);
                overlapping_detection_found = true;
                break;
            }
        }

        // If no overlapping detection was found, add the current detection to the merged detections
        if !overlapping_detection_found {
            merged_detections.push(detections[i].clone());
        }
    }

    merged_detections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bbox = BBox {
            x: 10,
            y: 20,
            width: 30,
            height: 40,
        };
        let class = 1;
        let confidence = 0.5;
        let detection = Detection::new(bbox.clone(), class, confidence);
        assert_eq!(detection.bbox, bbox);
        assert_eq!(detection.class, class);
        assert_eq!(detection.confidence, confidence);
    }

    #[test]
    fn test_merge_overlapping_detections() {
        // Set up some test detections
        let detections = vec![
            Detection::new(BBox::new(0, 0, 100, 100), 1, 0.9),
            Detection::new(BBox::new(50, 0, 100, 100), 1, 0.8),
            Detection::new(BBox::new(150, 0, 100, 100), 2, 0.7),
            Detection::new(BBox::new(-50, 0, 100, 100), 1, 0.6),
            Detection::new(BBox::new(50, 50, 100, 100), 2, 0.5),
        ];

        // Merge overlapping detections with the same class
        let detections = merge_overlapping_detections(&detections);

        // Check that overlapping detections have been merged correctly
        assert_eq!(detections.len(), 3);
        assert_eq!(
            detections[0],
            Detection::new(BBox::new(-50, 0, 200, 100), 1, 0.9)
        );
        assert_eq!(
            detections[1],
            Detection::new(BBox::new(150, 0, 100, 100), 2, 0.7)
        );
        assert_eq!(
            detections[2],
            Detection::new(BBox::new(50, 50, 100, 100), 2, 0.5)
        );
    }
}
