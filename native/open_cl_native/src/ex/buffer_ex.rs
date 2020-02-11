use std::fmt;
use std::convert::TryFrom;

use rustler::{Encoder, NifStruct, NifUnitEnum, NifUntaggedEnum, NifMap};
use rustler::resource::ResourceArc;

use opencl_core::{Buffer, KernelAccess, HostAccess, MemLocation, MemConfig};
use opencl_core::ll::{ClNumber};

use crate::{
    OutputEx,
    NumberType,
    NumberTyped,
    NumberTypedT,
    ErrorEx,
    ArrayEx,
    NumberListEx,
    NumberEx,
    // RuntimeNumberList,
};


#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum BufferError {
    #[fail(display = "Buffer type cast mismatch - buffer_type: {:?}, cast_type: {:?}", _0, _1)]
    TypeMismatch(NumberType, NumberType)
}

/// A non-drop (because it lacks a T or a NumberType) pointer to a boxed buffer.
struct UntypedBuffer(*mut libc::c_void);

unsafe impl Send for UntypedBuffer {}
unsafe impl Sync for UntypedBuffer {}

impl UntypedBuffer {

    unsafe fn new<T: ClNumber + NumberTypedT>(buffer: Buffer<T>) -> UntypedBuffer {
        let ptr: *mut Buffer<T> = Box::into_raw(Box::new(buffer));
        UntypedBuffer(ptr as *mut libc::c_void)
    }

    unsafe fn into_buffer<T: ClNumber + NumberTypedT>(&self) -> Buffer<T> {
        *(Box::from_raw(self.0 as *mut Buffer<T>))
    }

    unsafe fn get_ref<T: ClNumber + NumberTypedT>(&self) -> &Buffer<T> {
        Box::leak(Box::from_raw(self.0 as *mut Buffer<T>))
    }
}

impl Clone for UntypedBuffer {
    fn clone(&self) -> UntypedBuffer {
        unsafe { 
            let b: Box<Buffer<usize>> = Box::from_raw(self.0 as *mut Buffer<usize>);
            UntypedBuffer::new((*b).clone())
        }
    }
    
}

pub struct BufferWrapper {
    t: NumberType,
    inner: UntypedBuffer,
}

impl Clone for BufferWrapper {
    fn clone(&self) -> BufferWrapper {
        BufferWrapper {
            t: self.t,
            inner: self.inner.clone(),
        }
    }
}

impl BufferWrapper {
    pub fn new<T: ClNumber + NumberTypedT>(buffer: Buffer<T>) -> BufferWrapper {
        BufferWrapper {
            t: T::number_type_of(),
            inner: unsafe { UntypedBuffer::new(buffer) },
        }
    }

    #[inline]
    pub fn type_check<T: ClNumber + NumberTypedT>(&self) -> OutputEx<()> {
        if self.t != T::number_type_of() {
            let buffer_error = BufferError::TypeMismatch(self.t, T::number_type_of());
            Err(buffer_error.into())
        } else {
            Ok(())
        }
    }

    pub fn into_buffer<T: ClNumber + NumberTypedT>(self) -> OutputEx<Buffer<T>> {
        self.type_check::<T>()?;
        let native_buffer = unsafe { self.inner.into_buffer::<T>() };
        std::mem::forget(self);
        Ok(native_buffer)
    }

    pub fn buffer<T: ClNumber + NumberTypedT>(&self) -> OutputEx<&Buffer<T>> {
        self.type_check::<T>()?;
        Ok(unsafe { self.inner.get_ref() })
    }

    pub fn size<T: NumberEx>(&self) -> OutputEx<usize> {
        let buf_ref: &Buffer<T> = unsafe { self.inner.get_ref() };
        buf_ref.size().map_err(From::from)
    }

    pub fn len<T: NumberEx>(&self) -> OutputEx<usize> {
        let size = self.size::<T>()?;
        Ok(self.t.size_of() * size)
    }
}

impl NumberTyped for BufferWrapper {
    fn number_type(&self) -> NumberType {
        self.t
    }
}

impl Drop for BufferWrapper {
    fn drop(&mut self) {
        let _ = unsafe { self.inner.into_buffer::<usize>() };
    }
}

impl fmt::Debug for BufferWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let inner_ref = apply_number_type_func!(self.t, self.inner.get_ref);
        // let len = inner_ref.length();
        write!(f, "BufferWrapper {{type: {:?}}}", self.t)
    }
}

macro_rules! impl_buffer_conv {
    ($t:ident) => {
        impl TryFrom<BufferWrapper> for Buffer<$t> {
            type Error = ErrorEx;

            fn try_from(bw: BufferWrapper) -> Result<Buffer<$t>, ErrorEx> {
                bw.into_buffer::<$t>()
            }
        }
    }
}

impl_buffer_conv!(u8);
impl_buffer_conv!(i8);
impl_buffer_conv!(u16);
impl_buffer_conv!(i16);
impl_buffer_conv!(u32);
impl_buffer_conv!(i32);
impl_buffer_conv!(f32);
impl_buffer_conv!(u64);
impl_buffer_conv!(i64);
impl_buffer_conv!(f64);
impl_buffer_conv!(usize);
impl_buffer_conv!(isize);


#[derive(NifStruct)]
#[must_use]
#[module = "OpenCL.Buffer"]
pub struct BufferEx {
    __native__: ResourceArc<BufferWrapper>
}

impl Clone for BufferEx {
    fn clone(&self) -> BufferEx {
        let cloned_resource_arc = self.__native__.clone();
        BufferEx { __native__: cloned_resource_arc }
    }
}

impl BufferEx {
    pub fn from_core_buffer<T: ClNumber + NumberTypedT>(buf: Buffer<T>) -> BufferEx {
        BufferEx::from_buffer_wrapper(BufferWrapper::new(buf))
    }

    pub fn from_buffer_wrapper(wrapper: BufferWrapper) -> BufferEx {
        BufferEx{
            __native__: ResourceArc::new(wrapper)
        }
    }

    pub fn wrapper(&self) -> &BufferWrapper {
        &self.__native__
    }
}

impl NumberTyped for BufferEx {
    fn number_type(&self) -> NumberType {
        self.__native__.number_type()
    }
}


impl fmt::Debug for BufferEx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = self.number_type();
        write!(f, "BufferEx {{ {:?} }}", t)
    }
}

#[derive(NifUnitEnum, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum KernelAccessEx {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl From<KernelAccessEx> for KernelAccess {
    fn from(kernel_access: KernelAccessEx) -> KernelAccess {
        match kernel_access {
            KernelAccessEx::ReadOnly => KernelAccess::ReadOnly,
            KernelAccessEx::WriteOnly => KernelAccess::WriteOnly,
            KernelAccessEx::ReadWrite => KernelAccess::ReadWrite,
        }
    }
}

#[derive(NifUnitEnum, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum HostAccessEx {
    ReadOnly,
    WriteOnly,
    NoAccess,
    ReadWrite,
}

impl From<HostAccessEx> for HostAccess {
    fn from(host_access: HostAccessEx) -> HostAccess {
        match host_access {
            HostAccessEx::ReadOnly => HostAccess::ReadOnly,
            HostAccessEx::WriteOnly => HostAccess::WriteOnly,
            HostAccessEx::NoAccess => HostAccess::NoAccess,
            HostAccessEx::ReadWrite => HostAccess::ReadWrite,
        }
    }
}

#[derive(NifUnitEnum, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MemLocationEx {
    KeepInPlace,
    AllocOnDevice,
    CopyToDevice,
    ForceCopyToDevice,
}

impl From<MemLocationEx> for MemLocation {
    fn from(mem_location: MemLocationEx) -> MemLocation {
        match mem_location {
            MemLocationEx::KeepInPlace => MemLocation::KeepInPlace,
            MemLocationEx::AllocOnDevice => MemLocation::AllocOnDevice,
            MemLocationEx::CopyToDevice => MemLocation::CopyToDevice,
            MemLocationEx::ForceCopyToDevice => MemLocation::ForceCopyToDevice,
        }
    }
}

#[derive(NifUntaggedEnum, Debug)]
pub enum BufferCreatorEx {
    List(NumberListEx),
    Array(ArrayEx),
    Length(usize),
}

impl BufferCreatorEx {
    pub fn try_number_type(&self) -> Option<NumberType> {
        match self {
            BufferCreatorEx::List(list) => Some(list.number_type()),
            BufferCreatorEx::Array(arr) => Some(arr.number_type()),
            BufferCreatorEx::Length(_) => None,
        }
    }

    pub fn mem_location(&self) -> MemLocationEx {
        use BufferCreatorEx as B;
        match self {
            B::List(..) => MemLocationEx::CopyToDevice,
            B::Array(..) => MemLocationEx::CopyToDevice,
            B::Length(..) => MemLocationEx::AllocOnDevice,
        }
    }

    pub fn check_matches_type(&self, t: NumberType) -> OutputEx<()> {
        if let Some(creator_type) = self.try_number_type() {
            if !creator_type.matches(&t) {
                let e = BufferError::TypeMismatch(creator_type, t);
                return Err(e.into());
            }
        }
        Ok(())
    }
}

#[derive(NifMap, Debug, PartialEq, Eq, Hash, Clone)]
pub struct MemConfigBuilderEx {
    kernel_access: Option<KernelAccessEx>,
    host_access: Option<HostAccessEx>,
    mem_location: Option<MemLocationEx>,
}

impl MemConfigBuilderEx {
    pub fn with_mem_location_of_buffer_creator(mut self, b: &BufferCreatorEx) -> MemConfigBuilderEx {
        let loc = self.mem_location.unwrap_or_else(|| b.mem_location());
        self.mem_location = Some(loc);
        self
    }
}

impl Default for MemConfigBuilderEx {
    fn default() -> MemConfigBuilderEx {
        MemConfigBuilderEx{
            kernel_access: None,
            host_access: None,
            mem_location: None,
        }
    }
}

impl MemConfigBuilderEx {
    pub fn build(self) -> MemConfig {
        let mut cfg = MemConfig::default();
        if let Some(ha) = self.host_access {
            cfg.host_access = ha.into();
        };
        if let Some(ka) = self.kernel_access {
            cfg.kernel_access = ka.into();
        }
        if let Some(loc) = self.mem_location {
            cfg.mem_location = loc.into();
        };
        cfg
    }
}

#[derive(NifUnitEnum, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum NilOnly {
    Nil,
}

#[derive(NifUntaggedEnum, Debug, PartialEq, Eq, Hash, Clone)]
pub enum MemConfigEx{
    Nil(NilOnly),
    Builder(MemConfigBuilderEx),
}

impl MemConfigEx {
    pub fn into_builder(self) -> MemConfigBuilderEx {
        match self {
            MemConfigEx::Nil(..) => MemConfigBuilderEx::default(),
            MemConfigEx::Builder(b) => b,
        }
    }
}

fn _buffer_len<T: NumberEx>(buff: &BufferWrapper) -> OutputEx<usize> {
    buff.len::<T>()
}

#[rustler::nif]
pub fn buffer_length(buffer: BufferEx) -> OutputEx<usize> {
    let w = buffer.wrapper();
    apply_number_type!(w.t, _buffer_len, [w])
}

// use num_traits::Num;
// use std::fmt;
// use std::fmt::Debug;
// use std::sync::Arc;

// use rustler::resource::ResourceArc;
// use rustler::{Encoder, NifStruct, NifUnitEnum};

// use crate::ex::array_ex::ArrayEx;
// use crate::ex::number_ex::{Number, NumberType, NumberTyped, NumberVector};
// use crate::ex::session_ex::SessionEx;
// use crate::ex::DimsEx;
// use crate::ex::ErrorEx;
// use crate::traits::NativeWrapper;
// // use opencl_core::device_mem::flags::MemFlags;
// // use opencl_core::utils::vec_filled_with;
// use opencl_core::ll::utils::vec_filled_with;
// use opencl_core::ll::{Dims, KernelArg};
// use opencl_core::session::Session;
// use opencl_core::Buffer;
// // impl WrapperExResource for DeviceBuffer {}

// #[derive(Debug)]
// pub struct BufferWrapper<T>
// where
//     T: Debug + Sync + Send + Number + Num,
// {
//     device_mem: DeviceMem<T>,
//     session: Session,
//     buffer_access: BufferAccess>,
//     dims: Dims,
// }

// impl<T> BufferWrapper<T>
// where
//     T: Debug + Sync + Send + Number + Num,
// {
//     pub fn new(
//         session: Session,
//         dims: Dims,
//         data: Vec<T>,
//         buffer_access: BufferAccess,
//     ) -> Result<BufferWrapper<T>, ErrorEx>
//     where
//         T: Debug,
//     {
//         let device_mem = match buffer_access {
//             BufferAccess::ReadOnly => {
//                 DeviceMem::create_read_only_from(session.context(), &data[..])
//             }
//             BufferAccess::WriteOnly => {
//                 DeviceMem::create_write_only_from(session.context(), &data[..])
//             }
//             BufferAccess::ReadWrite => {
//                 DeviceMem::create_read_write_from(session.context(), &data[..])
//             }
//         }?;

//         Ok(BufferWrapper {
//             device_mem,
//             session,
//             buffer_access,
//             dims,
//         })
//     }

//     pub fn read(&self) -> Vec<T> {
//         let len = self.dims.n_items();
//         let zero = T::zero();
//         let mut data = vec_filled_with(zero, len);
//         self.session
//             .command_queue()
//             .read_buffer(&self.device_mem, &mut data[..])
//             .unwrap();
//         data
//     }
// }

// impl<T> KernelArg for BufferWrapper<T>
// where
//     T: Debug + Sync + Send + Number + Num,
// {
//     unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
//         self.device_mem.as_kernel_arg()
//     }
// }

// #[derive(Debug)]
// pub enum DeviceBuffer {
//     U8(BufferWrapper<u8>),
//     // I8(DeviceMemBuffer<i8>),
//     // U16(DeviceMemBuffer<u16>),
//     // I16(DeviceMemBuffer<i16>),
//     // U32(DeviceMemBuffer<u32>),
//     // I32(DeviceMemBuffer<i32>),
//     // F32(DeviceMemBuffer<f32>),
//     // U64(DeviceMemBuffer<u64>),
//     // I64(DeviceMemBuffer<i64>),
//     // F64(DeviceMemBuffer<f64>),
//     // Usize(DeviceMemBuffer<usize>),
//     // Isize(DeviceMemBuffer<isize>),
// }

// impl NumberTyped for DeviceBuffer {
//     fn number_type(&self) -> NumberType {
//         use DeviceBuffer as D;
//         use NumberType as NT;
//         match self {
//             D::U8(..) => NT::U8,
//             // D::I8(..) => NT::I8,
//             // D::U16(..) => NT::U16,
//             // D::I16(..) => NT::I16,
//             // D::U32(..) => NT::U32,
//             // D::I32(..) => NT::I32,
//             // D::F32(..) => NT::F32,
//             // D::U64(..) => NT::U64,
//             // D::I64(..) => NT::I64,
//             // D::F64(..) => NT::F64,
//             // D::Usize(..) => NT::Usize,
//             // D::Isize(..) => NT::Isize,
//         }
//     }
// }

// // impl
// // BufferWrapper::new::<u8>(session, dims, data, mem_flags).unwrap()

// impl KernelArg for DeviceBuffer {
//     unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
//         use DeviceBuffer as D;
//         match self {
//             D::U8(d) => d.as_kernel_arg(),
//         }
//     }
// }

// impl From<DeviceBuffer> for NumberVector {
//     fn from(d: DeviceBuffer) -> NumberVector {
//         use DeviceBuffer as D;
//         match d {
//             D::U8(buff) => buff.read().into(),
//         }
//     }
// }

// impl From<&DeviceBuffer> for NumberVector {
//     fn from(d: &DeviceBuffer) -> NumberVector {
//         use DeviceBuffer as D;
//         match d {
//             D::U8(buff) => buff.read().into(),
//         }
//     }
// }

// pub trait HasDeviceMem<T> where T: Debug + Sync + Send + Num {
//     fn get_device_mem(&self) -> &DeviceMem<T>;
// }

// impl DeviceBuffer {
//     fn new(
//         session: Session,
//         dims: Dims,
//         number_vector: NumberVector,
//         buffer_access: BufferAccess,
//     ) -> DeviceBuffer {
//         use DeviceBuffer as B;
//         use NumberVector as NV;
//         match number_vector {
//             NV::U8(data) => {
//                 let buffer = BufferWrapper::new(session, dims, data, buffer_access).unwrap();
//                 B::U8(buffer)
//             }
//             // Fix me unwrap vs result
//             // NV::U8 => B::U8(),
//             _ => panic!("NOOOOOOOPE"),
//         }
//     }

//     pub fn reference_count<T>(&self) -> u32 where Self: HasDeviceMem<T>, T: Debug + Sync + Send + Num {
//         self.get_device_mem()
//             .reference_count()
//             .unwrap_or_else(|e| panic!("Failed to retrieve buffer reference count: {:?}", e))
//     }
// }

// impl HasDeviceMem<u8> for DeviceBuffer {
//     fn get_device_mem(&self) -> &DeviceMem<u8> {
//         use DeviceBuffer as DB;
//         match self {
//             DB::U8(buffer_wrapper) => &buffer_wrapper.device_mem,
//         }
//     }
// }

// #[derive(NifStruct)]
// #[must_use]
// #[module = "OpenCL.DeviceBuffer"]
// pub struct DeviceBufferEx {
//     __native__: ResourceArc<DeviceBuffer>,
// }

// impl fmt::Debug for DeviceBufferEx {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "DeviceBufferEx {{ native: {:?} }}", self.native())
//     }
// }

// impl NativeWrapper<DeviceBuffer> for DeviceBufferEx {
//     fn native(&self) -> &DeviceBuffer {
//         &self.__native__
//     }
// }

// impl DeviceBufferEx {
//     pub fn new(device_buffer: DeviceBuffer) -> DeviceBufferEx {
//         DeviceBufferEx {
//             __native__: ResourceArc::new(device_buffer),
//         }
//     }

//     pub fn to_array(&self) -> ArrayEx {
//         let number_vector: NumberVector = self.native().into();
//         ArrayEx::from_number_vector(number_vector)
//     }
// }

// impl KernelArg for DeviceBufferEx {
//     unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
//         self.native().as_kernel_arg()
//     }
// }

// #[derive(NifUnitEnum, Debug, PartialEq, Eq, Clone, Copy)]
// pub enum BufferAccess {
//     ReadOnly,
//     WriteOnly,
//     ReadWrite,
// }

// impl From<BufferAccess> for MemFlags {
//     fn from(access: BufferAccess) -> MemFlags {
//         match access {
//             BufferAccess::ReadOnly => MemFlags::READ_ONLY_ALLOC_HOST_PTR,
//             BufferAccess::WriteOnly => MemFlags::WRITE_ONLY_ALLOC_HOST_PTR,
//             BufferAccess::ReadWrite => MemFlags::READ_WRITE_ALLOC_HOST_PTR,
//         }
//     }
// }
