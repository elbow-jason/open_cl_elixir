// use std::marker::PhantomData;
use crate::nif;

use crate::{BufferEx, DimsEx, NumEx};
use open_cl_core::ll::KernelArg as ClKernelArg;
use open_cl_core::{CommandQueueOptions, KernelArg, KernelOperation, Work};
use open_cl_core::{NumberType, NumberTyped};

#[derive(nif::NifUntaggedEnum, Debug)]
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
    fn into_kernel_op_arg<'a>(&'a self) -> KernelArg<'a> {
        match self {
            ArgEx::Buffer(buf) => KernelArg::Buffer(buf.wrapper().buffer()),
            ArgEx::Num(num) => KernelArg::Num(ClKernelArg::new(num)),
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

#[derive(nif::NifMap, Debug, Clone)]
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

#[derive(nif::NifMap, Debug)]
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

#[derive(nif::NifStruct, Debug)]
#[must_use]
#[module = "OpenCL.KernelOp"]
pub struct KernelOpEx {
    pub name: String,
    pub args: Vec<ArgEx>,
    pub work: WorkEx,
    pub command_queue_opts: Option<CommandQueueOptionsEx>,
}

impl KernelOpEx {
    pub fn into_kernel_operation<'a>(&'a self) -> KernelOperation<'a> {
        let mut op = KernelOperation::new(self.name.as_str()).with_work(self.work.clone());
        if let Some(opts) = &self.command_queue_opts {
            op = op.with_command_queue_options(opts.into());
        };
        for arg in self.args.iter() {
            op = op.add_arg(arg.into_kernel_op_arg());
        }
        op
    }
}
