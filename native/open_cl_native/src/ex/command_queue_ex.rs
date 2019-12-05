// use std::fmt;
// use opencl_core::CommandQueue;

// use rustler::resource::ResourceArc;
// use rustler::{Encoder, NifStruct};

// use super::{
//     WrapperEx,
//     WrapperExResource,
//     OutputEx,
// };

// use crate::traits::NativeWrapper;

// use crate::device_ex::DeviceEx;
// use crate::context_ex::ContextEx;

// impl WrapperExResource for CommandQueue {}

// #[derive(NifStruct)]
// #[must_use]
// #[module = "OpenCL.CommandQueue"]
// pub struct CommandQueueEx {
//     __native__: ResourceArc<WrapperEx<CommandQueue>>,
// }

// impl fmt::Debug for CommandQueueEx {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "CommandQueueEx {{ native: {:?} }}", self.native())
//     }
// }

// impl NativeWrapper<CommandQueue> for CommandQueueEx {
//     fn native(&self) -> &CommandQueue {
//         &self.__native__.item
//     }
// }

// impl CommandQueueEx {
//     pub fn new(command_queue: CommandQueue) -> CommandQueueEx {
//         CommandQueueEx {
//             __native__: command_queue.into_resource_arc(),
//         }
//     }

//     pub fn create(context: ContextEx, device: DeviceEx) -> OutputEx<CommandQueueEx> {
//         let native_command_queue: CommandQueue = CommandQueue::create(context.native(), device.native(), None)?;
//         Ok(CommandQueueEx::new(native_command_queue))
//     }
// }


// // device
// #[rustler::nif]
// fn command_queue_create(context: ContextEx, device: DeviceEx) -> OutputEx<CommandQueueEx> {
//     CommandQueueEx::create(context, device)
// }

// impl_native_method_into_other_and_nif!(CommandQueueEx, command_queue, context, ContextEx);
// impl_native_method_into_other_and_nif!(CommandQueueEx, command_queue, device, DeviceEx);

// impl_native_method_and_nif!(CommandQueueEx, command_queue, reference_count, u32);

//     // pub fn properties(&self) -> Output<CommandQueueProperties> {
//     //     self.info(CQInfo::Properties).map(|ret| unsafe{ ret.cl_decode() })
//     // }