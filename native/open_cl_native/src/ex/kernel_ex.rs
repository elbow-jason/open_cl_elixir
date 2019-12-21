use opencl_core::{Kernel, KernelArg, KernelArgSizeAndPointer, Work, Dims};

use crate::ex::{DeviceBufferEx, DimsEx, NumEx, SessionEx};

use rustler::{NifUnitEnum, NifUntaggedEnum};

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
pub fn kernel_execute_sync(
    session: SessionEx,
    name: &str,
    dims: DimsEx,
    args: Vec<KernelArgEx>,
) -> KernelStatus {
    let kernel = Kernel::create(session.program(), name).unwrap();
    for (i, arg) in args.iter().enumerate() {
        kernel.set_arg(i, arg).unwrap();
    };
    let work_dims: Dims = dims.into();
    let work = Work::new(work_dims);
    // let vol = work.global_work_size();
    let _event = session
        .command_queue()
        .sync_enqueue_kernel(&kernel, &work)
        .unwrap();
    KernelStatus::Ok
}

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
