
use std::fmt;
use std::fmt::Debug;
use std::sync::RwLock;

use opencl_core::DeviceMem;
use rustler::resource::ResourceArc;
use rustler::{Encoder, NifStruct, NifUnitEnum};

use opencl_core::{Dims, Session};
use opencl_core::device_mem::flags::MemFlags;
use crate::ex::ErrorEx;
use crate::ex::session_ex::SessionEx;
use crate::ex::number_ex::{NumberTyped, NumberType, NumberVector};
use crate::ex::{DimsEx};
use crate::traits::NativeWrapper;
// impl WrapperExResource for DeviceBuffer {}

#[derive(Debug)]
pub struct BufferWrapper<T> where T: Debug + Sync + Send {
    device_mem: RwLock<DeviceMem<T>>,
    session: Session,
    mem_flags: MemFlags,
    dims: Dims
}

impl<T> BufferWrapper<T> where T: Debug + Sync + Send {
    pub fn new(session: Session, dims: Dims, data: &[T], mem_flags: MemFlags) -> Result<BufferWrapper<T>, ErrorEx> {
        let device_mem = DeviceMem::create_from(session.context(), mem_flags, data)?;
        
        Ok(BufferWrapper {
            device_mem: RwLock::new(device_mem),
            session,
            mem_flags,
            dims
        })
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

// macro_rules! create_device_buffer {
//     ($variant:ident, $t:ty, $session:ident, $dims:ident, $mem_flags:ident, $data:ident) => {
//         DeviceBuffer::$variant{
//                 session: $session.clone(),
//                 mem_flags: $mem_flags,
//                 dims: $dims.into(),
//                 device_mem: ,
//             }
//     }
// } 

impl DeviceBuffer {
    fn new(
        session: Session,
        dims: Dims,
        number_vector: NumberVector,
        mem_flags: MemFlags,
    ) -> DeviceBuffer {
        use DeviceBuffer as B;
        use NumberVector as NV;
        match number_vector {
            // Fix me unwrap vs result
            NV::U8(data) => B::U8(BufferWrapper::new(session, dims, &data[..], mem_flags).unwrap()),
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
fn buffer_new(
    session: SessionEx,
    dims: DimsEx,
    number_vector: NumberVector,
    access: BufferAccess,
) -> DeviceBufferEx {
    let buf = DeviceBuffer::new(session.into(), dims.into(), number_vector, access.into());
    DeviceBufferEx::new(buf)
}

