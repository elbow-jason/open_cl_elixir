// use std::marker::PhantomData;
use rustler::{Encoder, NifMap, NifUntaggedEnum, NifStruct};

use opencl_core::{ClNumber, CommandQueueOptions, Work, KernelOpArg, KernelOperation};
use opencl_core::ll::{KernelArg};

use crate::{BufferEx, DimsEx, NumEx, NumberTypedT, NumberType, NumberTyped, NumberEx, OutputEx};

#[derive(NifUntaggedEnum, Debug)]
pub enum ArgEx {
    Buffer(BufferEx),
    Num(NumEx),
}

impl Clone for ArgEx {
    fn clone(&self) -> ArgEx {
        match self {
            ArgEx::Buffer(buff) => ArgEx::Buffer((*buff).clone()),
            ArgEx::Num(num) => ArgEx::Num(num.clone()),
        }
    }
}

impl ArgEx {
    fn into_kernel_op_arg<'a, T: KernelArg + ClNumber + NumberTypedT + From<NumEx>>(
        &'a self,
    ) -> OutputEx<KernelOpArg<'a, T>> {
        match self {
            ArgEx::Buffer(buf) => {
                let buffer_t = buf.wrapper().buffer()?;
                Ok(KernelOpArg::Buffer(buffer_t))
            },
            ArgEx::Num(num) => {
                let t = T::number_type_of();
                t.type_check(num.number_type())?;
                Ok(KernelOpArg::Num((*num).into()))
            },
        }
    }
}

impl NumberTyped for ArgEx {
    fn number_type(&self) -> NumberType {
        match self {
            ArgEx::Buffer(buf) => buf.number_type(),
            ArgEx::Num(num) => num.number_type(),
        }
    }
}

#[derive(NifMap, Debug, Clone)]
pub struct WorkEx {
    global_work_size: DimsEx,
    global_work_offset: Option<DimsEx>,
    local_work_size: Option<DimsEx>,
}

impl From<WorkEx> for Work {
    fn from(w: WorkEx) -> Work {
        let mut work: Work = Work::new(w.global_work_size);
        if let Some(gws) = w.global_work_offset {
            work = work.with_global_offset(gws);
        };

        if let Some(lws) = w.local_work_size {
            work = work.with_local_size(lws);
        };
        work
    }
}

#[derive(NifMap, Debug)]
pub struct CommandQueueOptionsEx {
    is_blocking: Option<bool>,
    offset: Option<usize>,
}

impl From<CommandQueueOptionsEx> for CommandQueueOptions {
    fn from(opts: CommandQueueOptionsEx) -> CommandQueueOptions {
        CommandQueueOptions::from(&opts)
    }
}

impl From<&CommandQueueOptionsEx> for CommandQueueOptions {
    fn from(opts: &CommandQueueOptionsEx) -> CommandQueueOptions {
        let defaults = CommandQueueOptions::default();
        CommandQueueOptions {
            is_blocking: opts.is_blocking.unwrap_or(defaults.is_blocking),
            offset: opts.offset.unwrap_or(defaults.offset),
            ..defaults
        }
    }
}

#[derive(NifStruct, Debug)]
#[must_use]
#[module = "OpenCL.KernelOp"]
pub struct KernelOpEx {
    name: String,
    args: Vec<ArgEx>,
    work: WorkEx,
    returning: Option<usize>,
    command_queue_opts: Option<CommandQueueOptionsEx>,
}

impl KernelOpEx {
    pub fn returning_arg(&self) -> Option<ArgEx> {
        self.returning
            .and_then(|arg_index| self.args.get(arg_index))
            .map(|arg| (*arg).clone())
    }
}

impl KernelOpEx {
    pub fn into_kernel_operation<'a, T: NumberEx + From<NumEx> + KernelArg>(&'a self) -> OutputEx<KernelOperation<'a, T>> {
        let mut op = KernelOperation::new(self.name.as_str()).with_work(self.work.clone());
        if let Some(opts) = &self.command_queue_opts {
            op = op.with_command_queue_options(opts.into());
        };
        if let Some(ret) = self.returning {
            op = op.with_returning_arg(ret);
        };
        for arg in self.args.iter() {
            let cl_arg = arg.into_kernel_op_arg()?;
            op = op.add_arg(cl_arg);
        }
        Ok(op)
    }
}

impl NumberTyped for KernelOpEx {
    fn number_type(&self) -> NumberType {
        self.returning
            .map(|index| self.args.get(index).map(|arg| arg.number_type()))
            .unwrap_or(Some(NumberType::U8))
            .unwrap()
    }
}


// #[derive(Debug, Fail, PartialEq, Eq, Clone)]
// pub enum WorkError {
//     #[fail(display = "global_work_size is a required work builder parameter")]
//     GlobalWorkSizeIsRequired,
// }

// #[derive(NifRecord)]
// #[tag = "global_work_size"]
// pub struct GlobalWorkSize(pub DimsEx);

// #[derive(NifRecord)]
// #[tag = "global_work_offset"]
// pub struct GlobalWorkOffset(pub DimsEx);

// #[derive(NifRecord)]
// #[tag = "local_work_size"]
// pub struct LocalWorkSize(pub DimsEx);

// #[derive(NifUntaggedEnum)]
// pub enum WorkParam {
//     GWS(GlobalWorkSize),
//     GWO(GlobalWorkOffset),
//     LWS(LocalWorkSize),
// }

// pub struct WorkBuilder {
//     global_work_size: Option<DimsEx>,
//     global_work_offset: Option<DimsEx>,
//     local_work_size: Option<DimsEx>,
// }

// impl Default for WorkBuilder {
//     fn default() -> WorkBuilder {
//         WorkBuilder {
//             global_work_size: None,
//             global_work_offset: None,
//             local_work_size: None,
//         }
//     }
// }

// impl From<Vec<WorkParam>> for WorkBuilder {
//     fn from(params: Vec<WorkParam>) -> WorkBuilder {
//         let mut builder = WorkBuilder::default();
//         for param in params.into_iter() {
//             match param {
//                 WorkParam::GWO(gwo) => builder.global_work_offset = Some(gwo.0),
//                 WorkParam::GWS(gws) => builder.global_work_size = Some(gws.0),
//                 WorkParam::LWS(lws) => builder.local_work_size = Some(lws.0),
//             }
//         }
//         builder
//     }
// }

// impl WorkBuilder {
//     pub fn build(self) -> OutputEx<Work> {
//
//     }
// }

// #[derive(NifRecord)]
// #[tag = "offset"]
// pub struct Offset(pub usize);

// #[derive(NifRecord)]
// #[tag = "is_blocking"]
// pub struct IsBlocking(pub bool);

// #[derive(NifRecord)]
// #[tag = "returning"]
// pub struct ReturningArgIndex(pub usize);

// pub enum CommandQueueOptionsParam {
//     IsBlocking(IsBlocking),
//     Offset(Offset),
// }

// pub struct CommandQueueOptionsBuilder {
//     is_blocking: Option<bool>,
//     offset: Option<usize>,
// }

// impl Default for CommandQueueOptionsBuilder {
//     fn default() -> WorkBuilder {
//         WorkBuilder {
//             is_blocking: None,
//             offset: None,
//         }
//     }
// }

// impl From<Vec<CommandQueueOptionsParam>> for CommandQueueOptionsBuilder {
//     fn from(params: Vec<CommandQueueOptionsParam>) -> CommandQueueOptionsBuilder {
//         let mut builder = WorkBuilder::default();
//         for param in params.into_iter() {
//             match param {
//                 CommandQueueOptionsParam::IsBlocking(is_blocking) => builder.is_blocking = Some(is_blocking.0),
//                 CommandQueueOptionsParam::Offset(offset) => builder.offset = Some(offset.0),
//             }
//         }
//         builder
//     }
// }

// impl CommandQueueOptionsBuilder {
//     pub fn build(&self) -> CommandQueueOptions {
//         let defaults = CommandQueueOptions::default();
//         let mut builder = CommandQueueOptionsBuilder {
//             is_blocking: Some(defaults.is_blocking),
//             offset: Some(defaults.offset),
//         };
//         if let Some(is_blocking) = self.is_blocking {
//             builder.is_blocking = Some(is_blocking);
//         };
//         if let Some(offset) = self.offset {
//             builder.offset = Some(offset);
//         };
//         builder
//     }
// }

// pub enum KernelOpParam {
//     Arg(ArgEx),
//     WorkParam(WorkParam),
//     Returning(usize),
//     CommandQueueOptionsParam(CommandQueueOptionsParam),
// }

// pub enum WorkSource {
//     Work(Work),
//     Params(Vec<WorkParam>),
// }

// impl From<Work> for WorkSource {
//     fn from(w: Work) -> WorkSource {
//         WorkSource::Work(w)
//     }
// }

// impl From<Vec<WorkParam>> for WorkSource {
//     fn from(params: Vec<WorkParam>) -> WorkSource {
//         WorkSource::Params(params)
//     }
// }

// pub enum CommandQueueOptionsSource {
//     Opts(CommandQueueOptions),
//     Params(Vec<CommandQueueOptionsParam>),
// }

// impl From<CommandQueueOptions> for CommandQueueOptionsSource {
//     fn from(opts: CommandQueueOptions) -> CommandQueueOptionsSource {
//         CommandQueueOptionsSource::Opts(opts)
//     }
// }

// impl From<Vec<CommandQueueOptionsParam>> for CommandQueueOptionsSource {
//     fn from(params: Vec<CommandQueueOptionsParam>) -> CommandQueueOptionsSource {
//         CommandQueueOptionsSource::Params(params)
//     }
// }

// #[derive(Debug, Fail, PartialEq, Eq, Clone)]
// pub enum KernelOperationError {
//     #[fail(display = "Returning index {} was out of range - length is {}", _0, _1)]
//     ReturningIndexIsOutOfRange(usize, usize),
// }

// pub struct KernelOpBuilder<T: ClNumber + KernelArg> {
//     name: String,
//     args: Vec<ArgEx>,
//     work: Option<WorkSource>,
//     returning: Option<usize>,
//     command_queue_opts: Option<CommandQueueOptionsSource>,
//     phantom: PhantomData<T>,
// }

// impl KernelOpBuilder<T: ClNumber + KernelArg> {
//     pub fn new(name: String) -> KernelOpBuilder<T> {
//         KernelOpBuilder {
//             name,
//             args: Vec::new(),
//             work: None,
//             returning: None,
//             command_queue_opts: None,
//         }
//     }

//     pub fn add_arg(mut self, arg: ArgEx) -> KernelOpBuilder<T> {
//         self.args.push(arg);
//         self
//     }

//     pub fn with_args(mut self, args: Vec<ArgEx>) -> KernelOpBuilder<T> {
//         self.args.extend(args);
//         self
//     }

//     pub fn with_command_queue_opts<C: Into<CommandQueueOptionsSource>>(mut self, c: C) -> KernelOpBuilder<T> {
//         self.command_queue_opts = Some(c.into());
//         self
//     }

//     pub fn with_work<W: Into<WorkSource>>(mut self, w: W) -> KernelOpBuilder<T> {
//         self.work = Some(w.into());
//         self
//     }

//     pub fn returning_arg(mut self, arg_index: usize) -> KernelOpBuilder<T> {
//         self.returning = Some(arg_index);
//         self
//     }

//     fn check_returning_index(&self) -> OutputEx<()> {
//         if let Some(ref arg_index) = self.returning {
//             if arg_index >= self.args.len() {
//                 let len = self.len.args();
//                 let e = KernelOpError::ArgIndexOutOfRange(arg_index, len);
//                 return Err(e);
//             }
//         }
//         Ok(())
//     }

//     pub fn build(self) -> OutputEx<KernelOperation<T>> {
//         self.check_returning_index()?;

//     }
// }

// use opencl_core::Kernel;
// use opencl_core::ll::{Dims, KernelArg, Work};
// use crate::ex::{DimsEx, NumEx, SessionEx};
// // use crate::ex::{DeviceBufferEx};

// use rustler::{NifUnitEnum, NifUntaggedEnum};

// #[derive(NifUntaggedEnum, Debug)]
// pub enum KernelArgEx {
//     Number(NumEx),
//     Buffer(DeviceBufferEx),
// }

// impl KernelArg for KernelArgEx {
//     unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
//         use KernelArgEx as K;
//         match &self {
//             K::Number(num) => (*num).as_kernel_arg(),
//             K::Buffer(buffer) => (*buffer).as_kernel_arg(),
//         }
//     }
// }

// #[derive(NifUnitEnum)]
// pub enum KernelStatus {
//     Ok,
//     Error,
// }

// #[rustler::nif]
// pub fn kernel_execute_sync(
//     session: SessionEx,
//     name: &str,
//     dims: DimsEx,
//     args: Vec<KernelArgEx>,
// ) -> KernelStatus {
//     let kernel = Kernel::create(session.program(), name).unwrap();
//     for (i, arg) in args.iter().enumerate() {
//         kernel.set_arg(i, arg).unwrap();
//     }
//     let work_dims: Dims = dims.into();
//     let work = Work::new(work_dims);
//     // let vol = work.global_work_size();
//     let _event = session
//         .command_queue()
//         .sync_enqueue_kernel(&kernel, &work)
//         .unwrap();
//     KernelStatus::Ok
// }

// #[rustler::nif]
// pub fn kernel_execute_sync(_session: SessionEx) -> KernelStatus {
//     // let kernel = Kernel::create(session.program(), &name[..]).unwrap();
//     // for (i, arg) in args.iter().enumerate() {
//     //     kernel.set_arg(i, arg).unwrap();
//     // }
//     // let work = Work::new(dims);
//     // let _event = session.command_queue().sync_enqueue_kernel(&kernel, &work).unwrap();
//     KernelStatus::Ok
// }
