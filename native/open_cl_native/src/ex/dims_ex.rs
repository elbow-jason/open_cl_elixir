use opencl_core::Dims;
use rustler::{NifTuple, NifUntaggedEnum};
// use crate::traits::NativeWrapper;

pub trait Dimension {
    fn product(&self) -> usize;

    fn matches_length(&self, len: usize) -> bool {
        self.product() == len
    }
}

#[derive(NifTuple, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ThreeDims(usize, usize, usize);

impl Dimension for ThreeDims {
    fn product(&self) -> usize {
        self.0 * self.1 * self.2
    }
}

#[derive(NifTuple, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TwoDims(usize, usize);

impl Dimension for TwoDims {
    fn product(&self) -> usize {
        self.0 * self.1
    }
}

#[derive(NifTuple, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct OneDim(usize);

impl Dimension for OneDim {
    fn product(&self) -> usize {
        self.0
    }
}

#[derive(NifUntaggedEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DimsEx {
    LoneNum(usize),
    One(OneDim),
    Two(TwoDims),
    Three(ThreeDims),
}

impl Dimension for DimsEx {
    fn product(&self) -> usize {
        match self {
            DimsEx::LoneNum(n) => *n,
            DimsEx::One(dims) => dims.product(),
            DimsEx::Two(dims) => dims.product(),
            DimsEx::Three(dims) => dims.product(),
        }
    }
}

impl From<DimsEx> for Dims {
    fn from(dims: DimsEx) -> Dims {
        match dims {
            DimsEx::LoneNum(x) => Dims::One(x),
            DimsEx::One(OneDim(x)) => Dims::One(x),
            DimsEx::Two(TwoDims(x, y)) => Dims::Two(x, y),
            DimsEx::Three(ThreeDims(x, y, z)) => Dims::Three(x, y, z),
        }
    }
}

impl From<Dims> for DimsEx {
    fn from(dims: Dims) -> DimsEx {
        match dims {
            Dims::One(x) => DimsEx::One(OneDim(x)),
            Dims::Two(x, y) => DimsEx::Two(TwoDims(x, y)),
            Dims::Three(x, y, z) => DimsEx::Three(ThreeDims(x, y, z)),
        }
    }
}
