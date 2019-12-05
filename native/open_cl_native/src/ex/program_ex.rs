// use std::fmt;
// use opencl_core::Program;

// use rustler::resource::ResourceArc;
// use rustler::{Encoder, NifStruct};

// use super::{
//     WrapperEx,
//     WrapperExResource,
//     OutputEx,
// };

// use crate::traits::NativeWrapper;

// use crate::device_ex::DeviceEx;

// impl WrapperExResource for Program {}

// #[derive(NifStruct)]
// #[must_use]
// #[module = "OpenCL.Program"]
// pub struct ProgramEx {
//     __native__: ResourceArc<WrapperEx<Program>>,
// }

// impl fmt::Debug for ProgramEx {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "ProgramEx {{ native: {:?} }}", self.native())
//     }
// }

// impl NativeWrapper<Program> for ProgramEx {
//     fn native(&self) -> &Program {
//         &self.__native__.item
//     }
// }

// impl ProgramEx {
//     pub fn new(program: Program) -> ProgramEx {
//         ProgramEx {
//             __native__: program.into_resource_arc(),
//         }
//     }

//     pub fn create(context: &Context, src: &str) -> OutputEx<ProgramEx> {
//         let native_program: Program = Program::create(context.native(), &str)?;
//         Ok(ProgramEx::new(native_program))
//     }
// }


// // device
// #[rustler::nif]
// fn program_create(context: ContextEx, src: &str) -> OutputEx<ProgramEx> {
//     ProgramEx::create(&context, src)
// }