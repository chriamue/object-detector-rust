use crate::{BBox, Class};

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
