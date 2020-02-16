// use std::marker::PhantomData;
use rustler::{Encoder, NifMap, NifStruct, NifUntaggedEnum};

use opencl_core::ll::KernelArg;
use opencl_core::{ClNumber, CommandQueueOptions, KernelOpArg, KernelOperation, Work};

use crate::{BufferEx, DimsEx, NumEx, NumberEx, NumberType, NumberTyped, NumberTypedT, OutputEx};

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
            }
            ArgEx::Num(num) => {
                let t = T::number_type_of();
                t.type_check(num.number_type())?;
                Ok(KernelOpArg::Num((*num).into()))
            }
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
    pub fn into_kernel_operation<'a, T: NumberEx + From<NumEx> + KernelArg>(
        &'a self,
    ) -> OutputEx<KernelOperation<'a, T>> {
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
