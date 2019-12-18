use opencl_core::{Kernel, KernelArg};

use rustler::{Encoder, NifUntaggedEnum, NifUnitEnum};
use crate::ex::{NumEx, DeviceBufferEx, SessionEx};

#[derive(NifUntaggedEnum)]
pub enum KernelArgEx {
    NumEx(NumEx),
    DeviceBufferEx(DeviceBufferEx),
}

impl KernelArg for KernelArgEx {

}

#[derive(NifUnitEnum)]
pub enum KernelStatus {
    Ok,
    Error,
}

#[rustler::nif]
fn execute_kernel_sync(session: SessionEx, name: &str, args: Vec<KernelArgEx>) -> KernelStatus {
    let kernel = Kernel::create(session.program(), name);
    kernel.a
    KernelStatus::Ok
}