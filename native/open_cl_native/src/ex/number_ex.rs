use crate::nif;
use open_cl_core::ll::numbers::Char;
use open_cl_core::ll::numbers::NumCastFrom;
use open_cl_core::{KernelArgPtr, Number, NumberType, NumberTyped, NumberTypedT};
use std::fmt::Debug;

pub trait NumExT: Number + NumberTypedT + KernelArgPtr {}

impl NumExT for u8 {}
impl NumExT for i8 {}
impl NumExT for u16 {}
impl NumExT for i16 {}
impl NumExT for u32 {}
impl NumExT for i32 {}
impl NumExT for f32 {}
impl NumExT for u64 {}
impl NumExT for i64 {}
impl NumExT for f64 {}
impl NumExT for usize {}
// impl NumberEx for Bool {}
// impl NumberEx for isize {}

#[derive(nif::NifUntaggedEnum, Debug, Clone, Copy)]
pub enum NumEx {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    F32(f32),
    U64(u64),
    I64(i64),
    F64(f64),
    Usize(usize),
    // Bool(u32),
    // Isize(isize),
}

impl NumCastFrom<NumEx> for Char {
    fn num_cast_from(num: NumEx) -> Option<Char> {
        match num {
            NumEx::U8(val) => Char::num_cast_from(val),
            NumEx::I8(val) => Char::num_cast_from(val),
            NumEx::U16(val) => Char::num_cast_from(val),
            NumEx::I16(val) => Char::num_cast_from(val),
            NumEx::U32(val) => Char::num_cast_from(val),
            NumEx::I32(val) => Char::num_cast_from(val),
            NumEx::F32(val) => Char::num_cast_from(val),
            NumEx::U64(val) => Char::num_cast_from(val),
            NumEx::I64(val) => Char::num_cast_from(val),
            NumEx::F64(val) => Char::num_cast_from(val),
            NumEx::Usize(val) => Char::num_cast_from(val),
            // NumEx::Isize(val) => Char::num_cast_from(val),
        }
    }
}

unsafe impl KernelArgPtr for NumEx {
    fn kernel_arg_size(&self) -> usize {
        match self {
            NumEx::I8(..) => std::mem::size_of::<i8>(),
            NumEx::U8(..) => std::mem::size_of::<u8>(),
            NumEx::U16(..) => std::mem::size_of::<u16>(),
            NumEx::I16(..) => std::mem::size_of::<i16>(),
            NumEx::U32(..) => std::mem::size_of::<u32>(),
            NumEx::I32(..) => std::mem::size_of::<i32>(),
            NumEx::F32(..) => std::mem::size_of::<f32>(),
            NumEx::U64(..) => std::mem::size_of::<u64>(),
            NumEx::I64(..) => std::mem::size_of::<i64>(),
            NumEx::F64(..) => std::mem::size_of::<f64>(),
            NumEx::Usize(..) => std::mem::size_of::<usize>(),
        }
    }

    fn kernel_arg_number_type(&self) -> NumberType {
        self.number_type()
    }

    unsafe fn kernel_arg_ptr(&self) -> *const libc::c_void {
        match self {
            NumEx::I8(n) => n as *const _ as *const libc::c_void,
            NumEx::U8(n) => n as *const _ as *const libc::c_void,
            NumEx::U16(n) => n as *const _ as *const libc::c_void,
            NumEx::I16(n) => n as *const _ as *const libc::c_void,
            NumEx::U32(n) => n as *const _ as *const libc::c_void,
            NumEx::I32(n) => n as *const _ as *const libc::c_void,
            NumEx::F32(n) => n as *const _ as *const libc::c_void,
            NumEx::U64(n) => n as *const _ as *const libc::c_void,
            NumEx::I64(n) => n as *const _ as *const libc::c_void,
            NumEx::F64(n) => n as *const _ as *const libc::c_void,
            NumEx::Usize(n) => n as *const _ as *const libc::c_void,
        }
    }

    unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut libc::c_void {
        self.kernel_arg_ptr() as *mut libc::c_void
    }
}

impl NumberTyped for NumEx {
    fn number_type(&self) -> NumberType {
        match self {
            NumEx::U8(..) => u8::number_type(),
            NumEx::I8(..) => i8::number_type(),
            NumEx::U16(..) => u16::number_type(),
            NumEx::I16(..) => i16::number_type(),
            NumEx::U32(..) => u32::number_type(),
            NumEx::I32(..) => i32::number_type(),
            NumEx::F32(..) => f32::number_type(),
            NumEx::U64(..) => u64::number_type(),
            NumEx::I64(..) => i64::number_type(),
            NumEx::F64(..) => f64::number_type(),
            NumEx::Usize(..) => usize::number_type(),
        }
    }
}
