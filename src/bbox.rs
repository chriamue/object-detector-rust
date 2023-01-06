//! This module contains the `BBox` struct for representing bounding boxes and functions for calculating overlap and union.
//!
//! # Examples
//!
//! ```
//! use object_detector_rust::bbox::BBox;
//!
//! let bbox1 = BBox {
//!     x: 10,
//!     y: 20,
//!     width: 30,
//!     height: 40,
//! };
//!
//! let bbox2 = BBox {
//!     x: 15,
//!     y: 25,
//!     width: 35,
//!     height: 45,
//! };
//!
//! let overlap = bbox1.overlap(&bbox2);
//! let union = bbox1.union_area(&bbox2);
//!
//! assert!(overlap > 0.0 && overlap < 1.0);
//! assert!(union > 1);
//! ```
//!
//! This example shows how to use the `BBox` struct to represent bounding boxes and how to calculate the overlap and union between two bounding boxes.

/// Struct for representing a bounding box
#[derive(Debug, Clone, PartialEq)]
pub struct BBox {
    /// X coordinate of the top-left corner of the bounding box
    pub x: i32,
    /// Y coordinate of the top-left corner of the bounding box
    pub y: i32,
    /// Width of the bounding box
    pub width: u32,
    /// Height of the bounding box
    pub height: u32,
}

impl BBox {
    /// Creates a new bounding box with the specified coordinates and dimensions
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> BBox {
        BBox {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns the left coordinate of the bounding box
    pub fn left(&self) -> i32 {
        self.x
    }

    /// Returns the right coordinate of the bounding box
    pub fn right(&self) -> i32 {
        self.x + self.width as i32
    }

    /// Returns the top coordinate of the bounding box
    pub fn top(&self) -> i32 {
        self.y
    }

    /// Returns the bottom coordinate of the bounding box
    pub fn bottom(&self) -> i32 {
        self.y + self.height as i32
    }

    /// Calculates the width of the intersection between two bounding boxes
    ///
    /// # Examples
    ///
    /// ```
    /// use object_detector_rust::bbox::BBox;
    ///
    /// let bbox1 = BBox {
    ///     x: 0,
    ///     y: 0,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox2 = BBox {
    ///     x: 50,
    ///     y: 0,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox3 = BBox {
    ///     x: 150,
    ///     y: 0,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox4 = BBox {
    ///     x: -50,
    ///     y: 0,
    ///     width: 100,
    ///     height: 100,
    /// };
    ///
    /// assert_eq!(bbox1.intersect_width(&bbox2), 50);
    /// assert_eq!(bbox1.intersect_width(&bbox3), 0);
    /// assert_eq!(bbox1.intersect_width(&bbox4), 50);
    /// assert_eq!(bbox2.intersect_width(&bbox3), 0);
    /// assert_eq!(bbox2.intersect_width(&bbox4), 0);
    /// assert_eq!(bbox3.intersect_width(&bbox4), 0);
    /// ```
    pub fn intersect_width(&self, other: &BBox) -> u32 {
        (self.right().min(other.right()) - self.left().max(other.left())).max(0) as u32
    }

    /// Calculates the height of the intersection between two bounding boxes
    ///
    /// # Examples
    ///
    /// ```
    /// use object_detector_rust::bbox::BBox;
    ///
    /// let bbox1 = BBox {
    ///     x: 0,
    ///     y: 0,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox2 = BBox {
    ///     x: 0,
    ///     y: 50,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox3 = BBox {
    ///     x: 0,
    ///     y: 150,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox4 = BBox {
    ///     x: 0,
    ///     y: -50,
    ///     width: 100,
    ///     height: 100,
    /// };
    ///
    /// assert_eq!(bbox1.intersect_height(&bbox2), 50);
    /// assert_eq!(bbox1.intersect_height(&bbox3), 0);
    /// assert_eq!(bbox1.intersect_height(&bbox4), 50);
    /// assert_eq!(bbox2.intersect_height(&bbox3), 0);
    /// assert_eq!(bbox2.intersect_height(&bbox4), 0);
    /// assert_eq!(bbox3.intersect_height(&bbox4), 0);
    /// ```
    pub fn intersect_height(&self, other: &BBox) -> u32 {
        (self.bottom().min(other.bottom()) - self.top().max(other.top())).max(0) as u32
    }

    /// Calculates the area of the intersection between two bounding boxes
    ///
    /// # Examples
    ///
    /// ```
    /// use object_detector_rust::bbox::BBox;
    ///
    /// let bbox1 = BBox {
    ///     x: 0,
    ///     y: 0,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox2 = BBox {
    ///     x: 50,
    ///     y: 50,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox3 = BBox {
    ///     x: 150,
    ///     y: 150,
    ///     width: 100,
    ///     height: 100,
    /// };
    ///
    /// assert_eq!(bbox1.intersect_area(&bbox2), 2500);
    /// assert_eq!(bbox1.intersect_area(&bbox3), 0);
    /// assert_eq!(bbox2.intersect_area(&bbox3), 0);
    /// ```
    pub fn intersect_area(&self, other: &BBox) -> u32 {
        self.intersect_width(other) * self.intersect_height(other)
    }

    /// Calculates the area of the bounding box
    ///
    /// # Examples
    ///
    /// ```
    /// use object_detector_rust::bbox::BBox;
    ///
    /// let bbox = BBox {
    ///     x: 0,
    ///     y: 0,
    ///     width: 100,
    ///     height: 100,
    /// };
    ///
    /// assert_eq!(bbox.self_area(), 10000);
    /// ```
    pub fn self_area(&self) -> u32 {
        self.width * self.height
    }

    /// Calculates the area of the union between two bounding boxes
    ///
    /// # Examples
    ///
    /// ```
    /// use object_detector_rust::bbox::BBox;
    ///
    /// let bbox1 = BBox {
    ///     x: 0,
    ///     y: 0,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox2 = BBox {
    ///     x: 50,
    ///     y: 50,
    ///     width: 100,
    ///     height: 100,
    /// };
    ///
    /// assert_eq!(bbox1.union_area(&bbox2), 17500);
    /// ```
    pub fn union_area(&self, other: &BBox) -> u32 {
        self.self_area() + other.self_area() - self.intersect_area(other)
    }

    /// Calculates the overlap between two bounding boxes as the ratio of their intersecting area to their union area
    ///
    /// # Examples
    ///
    /// ```
    /// use object_detector_rust::bbox::BBox;
    ///
    /// let bbox1 = BBox {
    ///     x: 0,
    ///     y: 0,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox2 = BBox {
    ///     x: 50,
    ///     y: 50,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox3 = BBox {
    ///     x: 150,
    ///     y: 150,
    ///     width: 100,
    ///     height: 100,
    /// };
    ///
    /// assert!(bbox1.overlap(&bbox2) > 0.14);
    /// assert_eq!(bbox1.overlap(&bbox3), 0.0);
    /// assert_eq!(bbox2.overlap(&bbox3), 0.0);
    /// ```
    pub fn overlap(&self, other: &BBox) -> f32 {
        self.intersect_area(other) as f32 / self.union_area(other) as f32
    }

    /// Merges two bounding boxes into a single bounding box that encloses both
    ///
    /// # Examples
    ///
    /// ```
    /// use object_detector_rust::bbox::BBox;
    ///
    /// let bbox1 = BBox {
    ///     x: 0,
    ///     y: 0,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox2 = BBox {
    ///     x: 50,
    ///     y: 50,
    ///     width: 100,
    ///     height: 100,
    /// };
    /// let bbox3 = BBox {
    ///     x: -50,
    ///     y: -50,
    ///     width: 100,
    ///     height: 100,
    /// };
    ///
    /// assert_eq!(bbox1.merge(&bbox2), BBox {
    ///     x: 0,
    ///     y: 0,
    ///     width: 150,
    ///     height: 150,
    /// });
    /// assert_eq!(bbox1.merge(&bbox3), BBox {
    ///     x: -50,
    ///     y: -50,
    ///     width: 150,
    ///     height: 150,
    /// });
    /// ```
    pub fn merge(&self, other: &BBox) -> BBox {
        let left = self.left().min(other.left());
        let right = self.right().max(other.right());
        let top = self.top().min(other.top());
        let bottom = self.bottom().max(other.bottom());

        BBox {
            x: left,
            y: top,
            width: (right - left) as u32,
            height: (bottom - top) as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap() {
        let bbox1 = BBox::new(0, 0, 100, 100);
        let bbox2 = BBox {
            x: 50,
            y: 50,
            width: 100,
            height: 100,
        };
        let bbox3 = BBox {
            x: 150,
            y: 150,
            width: 100,
            height: 100,
        };
        let bbox4 = BBox {
            x: -50,
            y: -50,
            width: 100,
            height: 100,
        };

        assert!(bbox1.overlap(&bbox2) > 0.14);
        assert_eq!(bbox1.overlap(&bbox3), 0.0);
        assert!(bbox1.overlap(&bbox4) > 0.14);
        assert_eq!(bbox2.overlap(&bbox3), 0.0);
        assert_eq!(bbox2.overlap(&bbox4), 0.0);
        assert_eq!(bbox3.overlap(&bbox4), 0.0);
    }

    #[test]
    fn test_bbox_merge() {
        let bbox1 = BBox::new(0, 0, 100, 100);
        let bbox2 = BBox::new(50, 50, 100, 100);
        let bbox3 = BBox::new(200, 200, 100, 100);

        let merged_bbox1 = BBox::merge(&bbox1, &bbox2);
        assert_eq!(merged_bbox1.x, 0);
        assert_eq!(merged_bbox1.y, 0);
        assert_eq!(merged_bbox1.width, 150);
        assert_eq!(merged_bbox1.height, 150);

        let merged_bbox2 = BBox::merge(&bbox1, &bbox3);
        assert_eq!(merged_bbox2.x, 0);
        assert_eq!(merged_bbox2.y, 0);
        assert_eq!(merged_bbox2.width, 300);
        assert_eq!(merged_bbox2.height, 300);
    }
}
