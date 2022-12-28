#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod bbox;
pub mod feature;

pub use bbox::BBox;

/// the object-detector-rust prelude
pub mod prelude {
    pub use super::bbox::BBox;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
