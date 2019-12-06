use std::fmt;
use std::sync::RwLock;

// use ndarray as nd;
use opencl_core::Dims;
use rustler::resource::ResourceArc;
use rustler::{Encoder, NifStruct, NifTuple, NifUntaggedEnum};

use crate::ex::array_ex::ArrayEx;
use crate::ex::number_ex::{CastNumber, NumberType, NumberTyped, NumberVector};
use crate::traits::NativeWrapper;

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

#[derive(Debug)]
pub struct Tensor {
    dims: Dims,
    data: RwLock<NumberVector>,
}

impl Tensor {
    pub fn new<D>(dims: D, number_vector: NumberVector) -> Tensor
    where
        D: Into<Dims> + Dimension + fmt::Debug,
    {
        if !dims.matches_length(number_vector.length()) {
            panic!(
                "Dimension mismatch error the dimensions {:?} do not match length {:?}",
                dims,
                number_vector.length()
            );
        }
        Tensor {
            dims: dims.into(),
            data: RwLock::new(number_vector),
        }
    }

    fn into_resource_arc(self) -> ResourceArc<Self> {
        ResourceArc::new(self)
    }

    pub fn dims(&self) -> &Dims {
        &self.dims
    }

    pub fn number_vector(&self) -> NumberVector {
        self.data.read().unwrap().clone()
    }
}

impl NumberTyped for Tensor {
    fn number_type(&self) -> NumberType {
        self.data.read().unwrap().number_type()
    }
}

#[derive(NifStruct)]
#[must_use]
#[module = "OpenCL.Tensor"]
pub struct TensorEx {
    __native__: ResourceArc<Tensor>,
}

impl fmt::Debug for TensorEx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TensorEx {{ native: {:?} }}", self.native())
    }
}

impl NativeWrapper<Tensor> for TensorEx {
    fn native(&self) -> &Tensor {
        &self.__native__
    }
}

impl TensorEx {
    pub fn new(tensor: Tensor) -> TensorEx {
        TensorEx {
            __native__: tensor.into_resource_arc(),
        }
    }

    pub fn dims(&self) -> Dims {
        self.native().dims().clone()
    }

    pub fn number_vector(&self) -> NumberVector {
        self.native().number_vector()
    }

    pub fn number_type(&self) -> NumberType {
        self.native().number_type()
    }

    // pub fn cloned_vec(&self) -> Vec<f32> {
    //     self.native().data.lock().unwrap().clone()
    // }

    // pub fn extend(&self, other: Vec<f32>) {

    //     let mut data = self.native().data.lock().unwrap();
    //     data.extend(other);
    // }

    // pub fn extend_from_slice(&self, slice: &[f32]) {
    //     let mut data = self.native().data.lock().unwrap();
    //     data.extend_from_slice(slice);
    // }
}
impl CastNumber for TensorEx {
    fn cast_number(&self, number_type: NumberType) -> TensorEx {
        let number_vector = self.native().data.read().unwrap().cast_number(number_type);
        let tensor = Tensor::new(DimsEx::from(self.dims()), number_vector);
        TensorEx::new(tensor)
    }
}

#[rustler::nif]
fn tensor_from_array(dims: DimsEx, array: ArrayEx) -> TensorEx {
    let tensor = Tensor::new(dims, array.number_vector());
    TensorEx::new(tensor)
}

#[rustler::nif]
fn tensor_from_number_vector(dims: DimsEx, number_vector: NumberVector) -> TensorEx {
    let tensor = Tensor::new(dims, number_vector);
    TensorEx::new(tensor)
}

#[rustler::nif]
fn tensor_self_dims(tensor: TensorEx) -> DimsEx {
    tensor.dims().into()
}

#[rustler::nif]
fn tensor_self_number_vector(tensor: TensorEx) -> NumberVector {
    tensor.number_vector()
}

#[rustler::nif]
fn tensor_self_number_type(tensor: TensorEx) -> NumberType {
    tensor.number_type()
}

#[rustler::nif]
fn tensor_self_cast_to_number_type(tensor: TensorEx, number_type: NumberType) -> TensorEx {
    tensor.cast_number(number_type)
}
