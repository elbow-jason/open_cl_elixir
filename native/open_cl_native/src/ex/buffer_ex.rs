use crate::nif;
use crate::nif::ErrorT;
use crate::{DeviceEx, MemConfigEx, NumTypeEx};
use open_cl_core::ll::ContextPtr;
use open_cl_core::{Buffer, Device, MemPtr, Number, NumberType, NumberTyped, NumberTypedT};
use std::fmt;

pub struct BufferWrapper(Buffer);

impl Clone for BufferWrapper {
    fn clone(&self) -> BufferWrapper {
        BufferWrapper(self.0.clone())
    }
}

impl BufferWrapper {
    pub fn new(buffer: Buffer) -> BufferWrapper {
        BufferWrapper(buffer)
    }

    #[inline]
    pub fn type_check<T: Number + NumberTypedT>(&self) -> nif::Result<()> {
        self.0
            .number_type()
            .type_check(&T::number_type())
            .map_err(|e| e.error())
    }

    pub fn into_buffer(self) -> Buffer {
        self.0
    }

    pub fn buffer(&self) -> &Buffer {
        &self.0
    }

    pub fn mut_buffer(&mut self) -> &mut Buffer {
        &mut self.0
    }

    pub fn size(&self) -> nif::Result<usize> {
        self.0.size().map_err(|e| e.error())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl NumberTyped for BufferWrapper {
    fn number_type(&self) -> NumberType {
        self.0.number_type()
    }
}

impl fmt::Debug for BufferWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BufferWrapper {{type: {:?}}}", self.number_type())
    }
}

macro_rules! impl_buffer_conv {
    ($t:ident) => {};
}

impl From<BufferWrapper> for Buffer {
    fn from(bw: BufferWrapper) -> Buffer {
        bw.into_buffer()
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

#[derive(nif::NifStruct)]
#[must_use]
#[module = "OpenCL.Buffer"]
pub struct BufferEx {
    __native__: nif::ResourceArc<BufferWrapper>,
}

impl Clone for BufferEx {
    fn clone(&self) -> BufferEx {
        BufferEx {
            __native__: self.__native__.clone(),
        }
    }
}

impl BufferEx {
    pub fn from_core_buffer(buf: Buffer) -> BufferEx {
        BufferEx::from_buffer_wrapper(BufferWrapper::new(buf))
    }

    pub fn from_buffer_wrapper(wrapper: BufferWrapper) -> BufferEx {
        BufferEx {
            __native__: nif::ResourceArc::new(wrapper),
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

#[rustler::nif(schedule = "DirtyCpu")]
pub fn buffer_self_length(buffer: BufferEx) -> usize {
    buffer.wrapper().len()
}

#[rustler::nif]
pub fn buffer_self_number_type(buffer: BufferEx) -> NumTypeEx {
    buffer.number_type().into()
}

#[rustler::nif]
pub fn buffer_self_mem_config(buffer: BufferEx) -> MemConfigEx {
    MemConfigEx::from_mem_config(buffer.wrapper().buffer().mem_config())
}

#[rustler::nif]
pub fn buffer_self_reference_count(buffer: BufferEx) -> nif::Result<u32> {
    let lock = buffer.wrapper().buffer().read_lock();
    unsafe { lock.reference_count() }.map_err(|e| e.error())
}

#[rustler::nif]
pub fn buffer_self_available_devices(buffer: BufferEx) -> nif::Result<Vec<DeviceEx>> {
    let lock = buffer.wrapper().buffer().read_lock();
    let ctx = unsafe { lock.context() }.map_err(|e| e.error())?;
    let ll_devices = unsafe { ctx.devices() }.map_err(|e| e.error())?;
    let devices = ll_devices
        .into_iter()
        .map(|ll_device| DeviceEx::new(Device::new(ll_device)))
        .collect();
    Ok(devices)
}
