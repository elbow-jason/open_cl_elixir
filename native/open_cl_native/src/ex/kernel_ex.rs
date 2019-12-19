use opencl_core::{Kernel, KernelArg, KernelArgSizeAndPointer, Work};

use rustler::{NifUntaggedEnum, NifUnitEnum};
use rustler::types::Decoder;
use crate::ex::{NumEx, DeviceBufferEx, SessionEx, DimsEx};

#[derive(NifUntaggedEnum, Debug)]
pub enum KernelArgEx {
    Number(NumEx),
    Buffer(DeviceBufferEx),
}

impl KernelArg for KernelArgEx {
    unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
        use KernelArgEx as K;
        match &self {
            K::Number(num) => (*num).as_kernel_arg(),
            K::Buffer(buffer) => (*buffer).as_kernel_arg(),
        }
    }
}

#[derive(NifUnitEnum)]
pub enum KernelStatus {
    Ok,
    Error,
}



#[rustler::nif]
fn execute_kernel_sync(session: SessionEx, name: &str, dims: DimsEx) -> KernelStatus {
    let kernel = Kernel::create(session.program(), name).unwrap();
    // for (i, arg) in args.iter().enumerate() {
    //     kernel.set_arg(i, arg).unwrap();
    // }
    let work = Work::new(dims);
    KernelStatus::Ok
}