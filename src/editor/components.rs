pub use super::gates::components::*;
use crate::components::*;

pub struct PlacingTag;

// I hate rust so much
#[derive(Debug, Clone, Copy)]
pub struct ConnectionPoints<const I: usize, const O: usize> {
    pub inputs: [Coordinate; I],
    pub outputs: [Coordinate; O],
}

impl<const I: usize, const O: usize> Default for ConnectionPoints<I, O> {
    fn default() -> Self {
        Self {
            inputs: [Coordinate::default(); I],
            outputs: [Coordinate::default(); O],
        }
    }
}
