// use std::sync::RwLock;
// use std::fmt;

// use ndarray as nd;
// use rustler::resource::ResourceArc;
// use rustler::{Encoder, NifStruct, NifTuple, NifUntaggedEnum};
// use opencl_core::Dims;

// use crate::traits::NativeWrapper;

// #[derive(NifTuple)]
// pub struct ThreeDims(usize, usize, usize);

// #[derive(NifTuple)]
// pub struct TwoDims(usize, usize);

// #[derive(NifTuple)]
// pub struct OneDim(usize);

// #[derive(NifUntaggedEnum)]
// pub enum DimsEx {
//     LoneNum(usize),
//     One(OneDim),
//     Two(TwoDims),
//     Three(ThreeDims),
// }


// // impl From<DimsEx> for Dims {
// //     fn from(dims: DimsEx) -> Dims {
// //         match dims {
// //             DimsEx::LoneNum(x) => Dims::One(x),
// //             DimsEx::One(OneDim(x)) => Dims::One(x),
// //             DimsEx::Two(TwoDims(x, y)) => Dims::Two(x, y),
// //             DimsEx::Three(ThreeDims(x, y, z)) => Dims::Three(x, y, z),
// //         }
// //     }
// // }

// // impl From<Dims> for DimsEx {
// //     fn from(dims: Dims) -> DimsEx {
// //         match dims {
// //             Dims::One(x) => DimsEx::One(OneDim(x)),
// //             Dims::Two(x, y) => DimsEx::Two(TwoDims(x, y)),
// //             Dims::Three(x, y, z) => DimsEx::Three(ThreeDims(x, y, z)),
// //         }
// //     }
// // }


// #[derive(Debug)]
// pub struct Tensor {
//     pub data: RwLock<()>
// }

// impl Tensor {
//     pub fn new(data: Vec<f32>) -> Tensor {
//         Tensor {
            
//             data: RwLock::new(())
//         }
//     }

//     fn into_resource_arc(self) -> ResourceArc<Self> {
//         ResourceArc::new(self)
//     }
// }



// #[derive(NifStruct)]
// #[must_use]
// #[module = "OpenCL.Tensor"]
// pub struct TensorEx {
//     __native__: ResourceArc<Tensor>,
// }

// impl fmt::Debug for TensorEx {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "TensorEx {{ native: {:?} }}", self.native())
//     }
// }


// impl NativeWrapper<Tensor> for TensorEx {
//     fn native(&self) -> &Tensor {
//         &self.__native__
//     }
// }

// impl TensorEx {

//     pub fn new(tensor: Tensor) -> TensorEx {
//         TensorEx {
//             __native__: tensor.into_resource_arc(),
//         }
//     }

//     // pub fn dims(&self) -> Dims {
//     //     self.native().dims.clone()
//     // }

//     // pub fn cloned_vec(&self) -> Vec<f32> {
//     //     self.native().data.lock().unwrap().clone()
//     // }

//     // pub fn extend(&self, other: Vec<f32>) {
        
//     //     let mut data = self.native().data.lock().unwrap();
//     //     data.extend(other);
//     // }

//     // pub fn extend_from_slice(&self, slice: &[f32]) {
//     //     let mut data = self.native().data.lock().unwrap();
//     //     data.extend_from_slice(slice);
//     // }
// }

// #[rustler::nif]
// fn tensor_new(_dims: DimsEx, data: Vec<f32>) -> TensorEx {
//     TensorEx::new(Tensor::new(data))
// }

// // #[rustler::nif]
// // fn tensor_self_dims(tensor: TensorEx) -> DimsEx {
// //     tensor.dims().into()
// // }


