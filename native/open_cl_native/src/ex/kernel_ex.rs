// use std::fmt;
// use opencl_core::Kernel;

// use rustler::resource::ResourceArc;
// use rustler::{Encoder, NifStruct};

// use super::{
//     WrapperEx,
//     WrapperExResource,
//     OutputEx,
// };

// use crate::traits::NativeWrapper;

// use crate::device_ex::DeviceEx;

// impl WrapperExResource for Kernel {}

// #[derive(NifStruct)]
// #[must_use]
// #[module = "OpenCL.Kernel"]
// pub struct KernelEx {
//     name: String,
//     args: Vec<KernelExArg>,
// }

// impl fmt::Debug for KernelEx {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "KernelEx {{ native: {:?} }}", self.native())
//     }
// }

// // device
// #[rustler::nif]
// fn kernel_sync_execute(session: SessionEx, kernel: KernelEx) -> OutputEx<Atom> {
//     session.native()
// }