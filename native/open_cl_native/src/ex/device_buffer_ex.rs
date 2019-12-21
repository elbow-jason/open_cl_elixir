use num_traits::Num;

use std::fmt;
use std::fmt::Debug;

use rustler::resource::ResourceArc;
use rustler::{Encoder, NifStruct, NifUnitEnum};

use crate::ex::array_ex::ArrayEx;
use crate::ex::number_ex::{Number, NumberType, NumberTyped, NumberVector};
use crate::ex::session_ex::SessionEx;
use crate::ex::DimsEx;
use crate::ex::ErrorEx;
use crate::traits::NativeWrapper;
use opencl_core::device_mem::flags::MemFlags;
use opencl_core::utils::vec_filled_with;
use opencl_core::{DeviceMem, Dims, KernelArg, KernelArgSizeAndPointer, Session};
// impl WrapperExResource for DeviceBuffer {}

#[derive(Debug)]
#[repr(C)]
pub struct BufferWrapper<T>
where
    T: Debug + Sync + Send + Number + Num,
{
    device_mem: DeviceMem<T>,
    session: Session,
    buffer_access: BufferAccess,
    dims: Dims,
}

impl<T> BufferWrapper<T>
where
    T: Debug + Sync + Send + Number + Num,
{
    pub fn new(
        session: Session,
        dims: Dims,
        data: Vec<T>,
        buffer_access: BufferAccess,
    ) -> Result<BufferWrapper<T>, ErrorEx>
    where
        T: Debug,
    {
        let device_mem = match buffer_access {
            BufferAccess::ReadOnly => {
                DeviceMem::create_read_only_from(session.context(), &data[..])
            }
            BufferAccess::WriteOnly => {
                DeviceMem::create_write_only_from(session.context(), &data[..])
            }
            BufferAccess::ReadWrite => {
                DeviceMem::create_read_write_from(session.context(), &data[..])
            }
        }?;

        Ok(BufferWrapper {
            device_mem,
            session,
            buffer_access,
            dims,
        })
    }

    pub fn read(&self) -> Vec<T> {
        let len = self.dims.n_items();
        let zero = T::zero();
        let mut data = vec_filled_with(zero, len);
        self.session
            .command_queue()
            .read_buffer(&self.device_mem, &mut data[..])
            .unwrap();
        data
    }
}

impl<T> KernelArg for BufferWrapper<T>
where
    T: Debug + Sync + Send + Number + Num,
{
    unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
        self.device_mem.as_kernel_arg()
    }
}

#[derive(Debug)]
pub enum DeviceBuffer {
    U8(BufferWrapper<u8>),
    // I8(DeviceMemBuffer<i8>),
    // U16(DeviceMemBuffer<u16>),
    // I16(DeviceMemBuffer<i16>),
    // U32(DeviceMemBuffer<u32>),
    // I32(DeviceMemBuffer<i32>),
    // F32(DeviceMemBuffer<f32>),
    // U64(DeviceMemBuffer<u64>),
    // I64(DeviceMemBuffer<i64>),
    // F64(DeviceMemBuffer<f64>),
    // Usize(DeviceMemBuffer<usize>),
    // Isize(DeviceMemBuffer<isize>),
}

impl NumberTyped for DeviceBuffer {
    fn number_type(&self) -> NumberType {
        use DeviceBuffer as D;
        use NumberType as NT;
        match self {
            D::U8(..) => NT::U8,
            // D::I8(..) => NT::I8,
            // D::U16(..) => NT::U16,
            // D::I16(..) => NT::I16,
            // D::U32(..) => NT::U32,
            // D::I32(..) => NT::I32,
            // D::F32(..) => NT::F32,
            // D::U64(..) => NT::U64,
            // D::I64(..) => NT::I64,
            // D::F64(..) => NT::F64,
            // D::Usize(..) => NT::Usize,
            // D::Isize(..) => NT::Isize,
        }
    }
}

// impl
// BufferWrapper::new::<u8>(session, dims, data, mem_flags).unwrap()

impl KernelArg for DeviceBuffer {
    unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
        use DeviceBuffer as D;
        match self {
            D::U8(d) => d.as_kernel_arg(),
        }
    }
}

impl From<DeviceBuffer> for NumberVector {
    fn from(d: DeviceBuffer) -> NumberVector {
        use DeviceBuffer as D;
        match d {
            D::U8(buff) => buff.read().into(),
        }
    }
}

impl From<&DeviceBuffer> for NumberVector {
    fn from(d: &DeviceBuffer) -> NumberVector {
        use DeviceBuffer as D;
        match d {
            D::U8(buff) => buff.read().into(),
        }
    }
}

impl DeviceBuffer {
    fn new(
        session: Session,
        dims: Dims,
        number_vector: NumberVector,
        buffer_access: BufferAccess,
    ) -> DeviceBuffer {
        use DeviceBuffer as B;
        use NumberVector as NV;
        match number_vector {
            NV::U8(data) => {
                let buffer = BufferWrapper::new(session, dims, data, buffer_access).unwrap();
                B::U8(buffer)
            }
            // Fix me unwrap vs result
            // NV::U8 => B::U8(),
            _ => panic!("NOOOOOOOPE"),
        }
    }
}

#[derive(NifStruct)]
#[must_use]
#[module = "OpenCL.DeviceBuffer"]
pub struct DeviceBufferEx {
    __native__: ResourceArc<DeviceBuffer>,
}

impl fmt::Debug for DeviceBufferEx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeviceBufferEx {{ native: {:?} }}", self.native())
    }
}

impl NativeWrapper<DeviceBuffer> for DeviceBufferEx {
    fn native(&self) -> &DeviceBuffer {
        &self.__native__
    }
}

impl DeviceBufferEx {
    pub fn new(device_buffer: DeviceBuffer) -> DeviceBufferEx {
        DeviceBufferEx {
            __native__: ResourceArc::new(device_buffer),
        }
    }

    pub fn to_array(&self) -> ArrayEx {
        let number_vector: NumberVector = self.native().into();
        ArrayEx::from_number_vector(number_vector)
    }
}

impl KernelArg for DeviceBufferEx {
    unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
        self.native().as_kernel_arg()
    }
}

#[derive(NifUnitEnum, Debug, PartialEq, Eq, Clone, Copy)]
pub enum BufferAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl From<BufferAccess> for MemFlags {
    fn from(access: BufferAccess) -> MemFlags {
        match access {
            BufferAccess::ReadOnly => MemFlags::READ_ONLY_ALLOC_HOST_PTR,
            BufferAccess::WriteOnly => MemFlags::WRITE_ONLY_ALLOC_HOST_PTR,
            BufferAccess::ReadWrite => MemFlags::READ_WRITE_ALLOC_HOST_PTR,
        }
    }
}

#[rustler::nif]
pub fn buffer_build_from_array(
    session: SessionEx,
    dims: DimsEx,
    number_type: NumberType,
    array: ArrayEx,
    access: BufferAccess,
) -> DeviceBufferEx {
    let buf = DeviceBuffer::new(
        session.into(),
        dims.into(),
        array.cast_to_number_vector(number_type),
        access.into(),
    );
    DeviceBufferEx::new(buf)
}

#[rustler::nif]
pub fn buffer_to_array(buffer: DeviceBufferEx) -> ArrayEx {
    buffer.to_array()
}
