use std::convert::TryFrom;
use std::fmt;

use rustler::resource::ResourceArc;
use rustler::{Encoder, NifMap, NifStruct, NifUnitEnum, NifUntaggedEnum};

use opencl_core::ll::ClNumber;
use opencl_core::{Buffer, HostAccess, KernelAccess, MemConfig, MemLocation};

use crate::{
    ArrayEx,
    ErrorEx,
    NumberEx,
    // RuntimeNumberList,
    NumberListEx,
    NumberType,
    NumberTyped,
    NumberTypedT,
    OutputEx,
};

#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum BufferError {
    #[fail(
        display = "Buffer type cast mismatch - buffer_type: {:?}, cast_type: {:?}",
        _0, _1
    )]
    TypeMismatch(NumberType, NumberType),
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
    };
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
    __native__: ResourceArc<BufferWrapper>,
}

impl Clone for BufferEx {
    fn clone(&self) -> BufferEx {
        let cloned_resource_arc = self.__native__.clone();
        BufferEx {
            __native__: cloned_resource_arc,
        }
    }
}

impl BufferEx {
    pub fn from_core_buffer<T: ClNumber + NumberTypedT>(buf: Buffer<T>) -> BufferEx {
        BufferEx::from_buffer_wrapper(BufferWrapper::new(buf))
    }

    pub fn from_buffer_wrapper(wrapper: BufferWrapper) -> BufferEx {
        BufferEx {
            __native__: ResourceArc::new(wrapper),
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
    pub fn with_mem_location_of_buffer_creator(
        mut self,
        b: &BufferCreatorEx,
    ) -> MemConfigBuilderEx {
        let loc = self.mem_location.unwrap_or_else(|| b.mem_location());
        self.mem_location = Some(loc);
        self
    }
}

impl Default for MemConfigBuilderEx {
    fn default() -> MemConfigBuilderEx {
        MemConfigBuilderEx {
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
pub enum MemConfigEx {
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
