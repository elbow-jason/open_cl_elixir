// use std::fmt;
// use opencl_core::Context;

// use rustler::resource::ResourceArc;
// use rustler::{Encoder, NifStruct};

// use super::{
//     WrapperEx,
//     WrapperExResource,
//     OutputEx,
// };

// use crate::traits::NativeWrapper;

// use crate::device_ex::DeviceEx;

// impl WrapperExResource for Context {}

// #[derive(NifStruct)]
// #[must_use]
// #[module = "OpenCL.Context"]
// pub struct ContextEx {
//     __native__: ResourceArc<WrapperEx<Context>>,
// }

// impl fmt::Debug for ContextEx {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "ContextEx {{ native: {:?} }}", self.native())
//     }
// }

// impl NativeWrapper<Context> for ContextEx {
//     fn native(&self) -> &Context {
//         &self.__native__.item
//     }
// }

// impl ContextEx {
//     pub fn new(context: Context) -> ContextEx {
//         ContextEx {
//             __native__: context.into_resource_arc(),
//         }
//     }

//     pub fn create(device: &DeviceEx) -> OutputEx<ContextEx> {
//         let native_context: Context = Context::create(device.native())?;
//         Ok(ContextEx::new(native_context))
//     }
// }


// // device
// #[rustler::nif]
// fn context_create(device: DeviceEx) -> OutputEx<ContextEx> {
//     ContextEx::create(&device)
// }