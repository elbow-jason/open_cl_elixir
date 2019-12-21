use rustler::{NifUnitEnum, NifUntaggedEnum};

use opencl_core::kernel::kernel_arg::{KernelArg, KernelArgSizeAndPointer};

// use std::convert::TryFrom;
use num_traits::{FromPrimitive, NumCast, ToPrimitive};
use std::fmt::Debug;

pub unsafe trait Number:
    Debug
    + Clone
    + Copy
    + Default
    + PartialEq
    + Send
    + Sync
    + NumCast
    + FromPrimitive
    + ToPrimitive
    + 'static
{
}

macro_rules! impl_number_for {
    ($( $t:ty ),+) => {
        $(
            unsafe impl Number for $t {}
        )+
    }
}

impl_number_for!(u8, i8, u16, i16, u32, i32, f32, u64, i64, f64, usize, isize);

macro_rules! impl_number_typed_t {
    ($t:ident, $variant:ident) => {
        impl NumberTypedT for $t {
            fn number_type() -> NumberType {
                NumberType::$variant
            }
        }
    };
}

impl_number_typed_t!(u8, U8);
impl_number_typed_t!(i8, I8);
impl_number_typed_t!(u16, U16);
impl_number_typed_t!(i16, I16);
impl_number_typed_t!(u32, U32);
impl_number_typed_t!(i32, I32);
impl_number_typed_t!(f32, F32);
impl_number_typed_t!(u64, U64);
impl_number_typed_t!(i64, I64);
impl_number_typed_t!(f64, F64);
impl_number_typed_t!(usize, Usize);
impl_number_typed_t!(isize, Isize);

pub trait NumberTyped {
    fn number_type(&self) -> NumberType;
}

pub trait NumberTypedT {
    fn number_type() -> NumberType;
}

pub trait CastNumber {
    fn cast_number(&self, number_type: NumberType) -> Self;
}

#[derive(NifUnitEnum, Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumberType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    Usize,
    Isize,
}

#[derive(NifUntaggedEnum, Debug, Clone, Copy)]
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
    Isize(isize),
}

impl CastNumber for NumEx {
    fn cast_number(&self, number_type: NumberType) -> NumEx {
        use NumberType as NT;
        match number_type {
            NT::I8 => NumEx::I8(self.into()),
            NT::U8 => NumEx::U8(self.into()),
            NT::U16 => NumEx::U16(self.into()),
            NT::I16 => NumEx::I16(self.into()),
            NT::U32 => NumEx::U32(self.into()),
            NT::I32 => NumEx::I32(self.into()),
            NT::F32 => NumEx::F32(self.into()),
            NT::U64 => NumEx::U64(self.into()),
            NT::I64 => NumEx::I64(self.into()),
            NT::F64 => NumEx::F64(self.into()),
            NT::Usize => NumEx::Usize(self.into()),
            NT::Isize => NumEx::Isize(self.into()),
        }
    }
}

impl KernelArg for NumEx {
    unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
        match self {
            NumEx::I8(num) => num.as_kernel_arg(),
            NumEx::U8(num) => num.as_kernel_arg(),
            NumEx::U16(num) => num.as_kernel_arg(),
            NumEx::I16(num) => num.as_kernel_arg(),
            NumEx::U32(num) => num.as_kernel_arg(),
            NumEx::I32(num) => num.as_kernel_arg(),
            NumEx::F32(num) => num.as_kernel_arg(),
            NumEx::U64(num) => num.as_kernel_arg(),
            NumEx::I64(num) => num.as_kernel_arg(),
            NumEx::F64(num) => num.as_kernel_arg(),
            NumEx::Usize(num) => num.as_kernel_arg(),
            NumEx::Isize(num) => num.as_kernel_arg(),
        }
    }
}

macro_rules! impl_num_ex_primitive_conversion {
    ($t:ty, $variant:ident) => {
        impl From<NumEx> for $t {
            fn from(num: NumEx) -> $t {
                match num {
                    NumEx::U8(val) => val as $t,
                    NumEx::I8(val) => val as $t,
                    NumEx::U16(val) => val as $t,
                    NumEx::I16(val) => val as $t,
                    NumEx::U32(val) => val as $t,
                    NumEx::I32(val) => val as $t,
                    NumEx::F32(val) => val as $t,
                    NumEx::U64(val) => val as $t,
                    NumEx::I64(val) => val as $t,
                    NumEx::F64(val) => val as $t,
                    NumEx::Usize(val) => val as $t,
                    NumEx::Isize(val) => val as $t,
                }
            }
        }

        impl From<&NumEx> for $t {
            fn from(num: &NumEx) -> $t {
                match num {
                    NumEx::U8(val) => *val as $t,
                    NumEx::I8(val) => *val as $t,
                    NumEx::U16(val) => *val as $t,
                    NumEx::I16(val) => *val as $t,
                    NumEx::U32(val) => *val as $t,
                    NumEx::I32(val) => *val as $t,
                    NumEx::F32(val) => *val as $t,
                    NumEx::U64(val) => *val as $t,
                    NumEx::I64(val) => *val as $t,
                    NumEx::F64(val) => *val as $t,
                    NumEx::Usize(val) => *val as $t,
                    NumEx::Isize(val) => *val as $t,
                }
            }
        }

        impl From<$t> for NumEx {
            fn from(num: $t) -> NumEx {
                NumEx::$variant(num)
            }
        }

        impl From<&$t> for NumEx {
            fn from(num: &$t) -> NumEx {
                NumEx::$variant(*num)
            }
        }
    };
}

impl_num_ex_primitive_conversion!(i8, I8);
impl_num_ex_primitive_conversion!(u8, U8);
impl_num_ex_primitive_conversion!(u16, U16);
impl_num_ex_primitive_conversion!(i16, I16);
impl_num_ex_primitive_conversion!(u32, U32);
impl_num_ex_primitive_conversion!(i32, I32);
impl_num_ex_primitive_conversion!(f32, F32);
impl_num_ex_primitive_conversion!(u64, U64);
impl_num_ex_primitive_conversion!(i64, I64);
impl_num_ex_primitive_conversion!(f64, F64);
impl_num_ex_primitive_conversion!(usize, Usize);
impl_num_ex_primitive_conversion!(isize, Isize);

impl NumberTyped for NumEx {
    fn number_type(&self) -> NumberType {
        match self {
            NumEx::U8(..) => NumberType::U8,
            NumEx::I8(..) => NumberType::I8,
            NumEx::U16(..) => NumberType::U16,
            NumEx::I16(..) => NumberType::I16,
            NumEx::U32(..) => NumberType::U32,
            NumEx::I32(..) => NumberType::I32,
            NumEx::F32(..) => NumberType::F32,
            NumEx::U64(..) => NumberType::U64,
            NumEx::I64(..) => NumberType::I64,
            NumEx::F64(..) => NumberType::F64,
            NumEx::Usize(..) => NumberType::Usize,
            NumEx::Isize(..) => NumberType::Isize,
        }
    }
}

// #[derive(Debug)]
#[derive(NifUntaggedEnum, Debug)]
pub enum NumberVector {
    // WTF is this syntax?
    U8(Vec::<u8>),
    I8(Vec::<i8>),
    U16(Vec::<u16>),
    I16(Vec::<i16>),
    U32(Vec::<u32>),
    I32(Vec::<i32>),
    F32(Vec::<f32>),
    U64(Vec::<u64>),
    I64(Vec::<i64>),
    F64(Vec::<f64>),
    Usize(Vec::<usize>),
    Isize(Vec::<isize>),
}

#[inline]
unsafe fn force_cast_vec<T, S>(mut v: Vec<T>) -> Vec<S> {
    let ptr = v.as_mut_ptr();
    let length = v.len();
    let capacity = v.capacity();
    std::mem::forget(v);
    Vec::from_raw_parts(ptr as *mut S, length, capacity)
}

impl NumberVector {
    pub fn new<T>(data: Vec<T>) -> NumberVector
    where
        T: NumberTypedT,
    {
        use NumberType as NT;
        use NumberVector as NV;
        // these casts are safe because they actually "change" T to T.
        match T::number_type() {
            NT::U8 => NV::U8(unsafe { force_cast_vec::<T, u8>(data) }),
            NT::I8 => NV::I8(unsafe { force_cast_vec::<T, i8>(data) }),
            NT::U16 => NV::U16(unsafe { force_cast_vec::<T, u16>(data) }),
            NT::I16 => NV::I16(unsafe { force_cast_vec::<T, i16>(data) }),
            NT::U32 => NV::U32(unsafe { force_cast_vec::<T, u32>(data) }),
            NT::I32 => NV::I32(unsafe { force_cast_vec::<T, i32>(data) }),
            NT::F32 => NV::F32(unsafe { force_cast_vec::<T, f32>(data) }),
            NT::U64 => NV::U64(unsafe { force_cast_vec::<T, u64>(data) }),
            NT::I64 => NV::I64(unsafe { force_cast_vec::<T, i64>(data) }),
            NT::F64 => NV::F64(unsafe { force_cast_vec::<T, f64>(data) }),
            NT::Usize => NV::Usize(unsafe { force_cast_vec::<T, usize>(data) }),
            NT::Isize => NV::Isize(unsafe { force_cast_vec::<T, isize>(data) }),
        }
    }

    pub fn length(&self) -> usize {
        use NumberVector as NV;
        match self {
            NV::U8(v) => v.len(),
            NV::I8(v) => v.len(),
            NV::U16(v) => v.len(),
            NV::I16(v) => v.len(),
            NV::U32(v) => v.len(),
            NV::I32(v) => v.len(),
            NV::F32(v) => v.len(),
            NV::U64(v) => v.len(),
            NV::I64(v) => v.len(),
            NV::F64(v) => v.len(),
            NV::Usize(v) => v.len(),
            NV::Isize(v) => v.len(),
        }
    }

    pub fn push(&mut self, num_ex: NumEx) {
        use NumberVector as NV;
        match self {
            NV::U8(ref mut this_vec) => this_vec.push(num_ex.into()),
            NV::I8(ref mut this_vec) => this_vec.push(num_ex.into()),
            NV::U16(ref mut this_vec) => this_vec.push(num_ex.into()),
            NV::I16(ref mut this_vec) => this_vec.push(num_ex.into()),
            NV::U32(ref mut this_vec) => this_vec.push(num_ex.into()),
            NV::I32(ref mut this_vec) => this_vec.push(num_ex.into()),
            NV::F32(ref mut this_vec) => this_vec.push(num_ex.into()),
            NV::U64(ref mut this_vec) => this_vec.push(num_ex.into()),
            NV::I64(ref mut this_vec) => this_vec.push(num_ex.into()),
            NV::F64(ref mut this_vec) => this_vec.push(num_ex.into()),
            NV::Usize(ref mut this_vec) => this_vec.push(num_ex.into()),
            NV::Isize(ref mut this_vec) => this_vec.push(num_ex.into()),
        }
    }

    pub fn extend(&mut self, other: &NumberVector) {
        use NumberVector as NV;
        match self {
            NV::U8(ref mut this_vec) => this_vec.extend(Vec::<u8>::from(other)),
            NV::I8(ref mut this_vec) => this_vec.extend(Vec::<i8>::from(other)),
            NV::U16(ref mut this_vec) => this_vec.extend(Vec::<u16>::from(other)),
            NV::I16(ref mut this_vec) => this_vec.extend(Vec::<i16>::from(other)),
            NV::U32(ref mut this_vec) => this_vec.extend(Vec::<u32>::from(other)),
            NV::I32(ref mut this_vec) => this_vec.extend(Vec::<i32>::from(other)),
            NV::F32(ref mut this_vec) => this_vec.extend(Vec::<f32>::from(other)),
            NV::U64(ref mut this_vec) => this_vec.extend(Vec::<u64>::from(other)),
            NV::I64(ref mut this_vec) => this_vec.extend(Vec::<i64>::from(other)),
            NV::F64(ref mut this_vec) => this_vec.extend(Vec::<f64>::from(other)),
            NV::Usize(ref mut this_vec) => this_vec.extend(Vec::<usize>::from(other)),
            NV::Isize(ref mut this_vec) => this_vec.extend(Vec::<isize>::from(other)),
        }
    }
}

impl Clone for NumberVector {
    fn clone(&self) -> NumberVector {
        use NumberVector as NV;
        match self {
            NV::U8(v) => NV::U8(v.clone()),
            NV::I8(v) => NV::I8(v.clone()),
            NV::U16(v) => NV::U16(v.clone()),
            NV::I16(v) => NV::I16(v.clone()),
            NV::U32(v) => NV::U32(v.clone()),
            NV::I32(v) => NV::I32(v.clone()),
            NV::F32(v) => NV::F32(v.clone()),
            NV::U64(v) => NV::U64(v.clone()),
            NV::I64(v) => NV::I64(v.clone()),
            NV::F64(v) => NV::F64(v.clone()),
            NV::Usize(v) => NV::Usize(v.clone()),
            NV::Isize(v) => NV::Isize(v.clone()),
        }
    }
}

macro_rules! define_slice_of_t {
    ($t:ident, $variant:ident) => {
        paste::item! {
            impl NumberVector {
                pub fn [<slice_ $t>]<'a>(&'a self) -> Option<&'a[$t]> {
                    if let NumberVector::$variant(data) = self {
                        Some(&data[..])
                    } else {
                        None
                    }
                }
            }
        }
    };
}

define_slice_of_t!(u8, U8);
define_slice_of_t!(i8, I8);
define_slice_of_t!(u16, U16);
define_slice_of_t!(i16, I16);
define_slice_of_t!(u32, U32);
define_slice_of_t!(i32, I32);
define_slice_of_t!(f32, F32);
define_slice_of_t!(u64, U64);
define_slice_of_t!(i64, I64);
define_slice_of_t!(f64, F64);
define_slice_of_t!(usize, Usize);
define_slice_of_t!(isize, Isize);

macro_rules! impl_number_typed_for_vec {
    ($t:ty, $variant:ident) => {
        impl NumberTyped for Vec<$t> {
            fn number_type(&self) -> NumberType {
                NumberType::$variant
            }
        }

        impl NumberTypedT for Vec<$t> {
            fn number_type() -> NumberType {
                NumberType::$variant
            }
        }
    };
}

impl_number_typed_for_vec!(u8, U8);
impl_number_typed_for_vec!(i8, I8);
impl_number_typed_for_vec!(u16, U16);
impl_number_typed_for_vec!(i16, I16);
impl_number_typed_for_vec!(u32, U32);
impl_number_typed_for_vec!(i32, I32);
impl_number_typed_for_vec!(f32, F32);
impl_number_typed_for_vec!(u64, U64);
impl_number_typed_for_vec!(i64, I64);
impl_number_typed_for_vec!(f64, F64);
impl_number_typed_for_vec!(usize, Usize);
impl_number_typed_for_vec!(isize, Isize);

impl NumberTyped for NumberVector {
    fn number_type(&self) -> NumberType {
        use NumberType as NT;
        use NumberVector as NV;
        match self {
            NV::U8(..) => NT::U8,
            NV::I8(..) => NT::I8,
            NV::U16(..) => NT::U16,
            NV::I16(..) => NT::I16,
            NV::U32(..) => NT::U32,
            NV::I32(..) => NT::I32,
            NV::F32(..) => NT::F32,
            NV::U64(..) => NT::U64,
            NV::I64(..) => NT::I64,
            NV::F64(..) => NT::F64,
            NV::Usize(..) => NT::Usize,
            NV::Isize(..) => NT::Isize,
        }
    }
}

macro_rules! cast_primitive_vec {
    ($t:ty, $data:ident) => {
        $data.iter().map(|num| *num as $t).collect()
    };
}

macro_rules! impl_from_for_vec {
    ($t:ty) => {
        impl From<NumberVector> for Vec<$t> {
            fn from(number_vector: NumberVector) -> Vec<$t> {
                use NumberVector as NV;
                match number_vector {
                    NV::U8(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::I8(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::U16(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::I16(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::U32(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::I32(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::F32(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::U64(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::I64(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::F64(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::Usize(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::Isize(v) => v.iter().map(|num| *num as $t).collect(),
                }
            }
        }

        impl From<&NumberVector> for Vec<$t> {
            fn from(number_vector: &NumberVector) -> Vec<$t> {
                use NumberVector as NV;
                match number_vector {
                    NV::U8(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::I8(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::U16(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::I16(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::U32(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::I32(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::F32(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::U64(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::I64(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::F64(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::Usize(v) => v.iter().map(|num| *num as $t).collect(),
                    NV::Isize(v) => v.iter().map(|num| *num as $t).collect(),
                }
            }
        }

        impl From<Vec<$t>> for NumberVector {
            fn from(v: Vec<$t>) -> NumberVector
            where
                Vec<$t>: NumberTyped,
            {
                use NumberType as NT;
                use NumberVector as NV;
                match v.number_type() {
                    NT::U8 => NV::U8(cast_primitive_vec!(u8, v)),
                    NT::I8 => NV::I8(cast_primitive_vec!(i8, v)),
                    NT::U16 => NV::U16(cast_primitive_vec!(u16, v)),
                    NT::I16 => NV::I16(cast_primitive_vec!(i16, v)),
                    NT::U32 => NV::U32(cast_primitive_vec!(u32, v)),
                    NT::I32 => NV::I32(cast_primitive_vec!(i32, v)),
                    NT::F32 => NV::F32(cast_primitive_vec!(f32, v)),
                    NT::U64 => NV::U64(cast_primitive_vec!(u64, v)),
                    NT::I64 => NV::I64(cast_primitive_vec!(i64, v)),
                    NT::F64 => NV::F64(cast_primitive_vec!(f64, v)),
                    NT::Usize => NV::Usize(cast_primitive_vec!(usize, v)),
                    NT::Isize => NV::Isize(cast_primitive_vec!(isize, v)),
                }
            }
        }
    };
}

impl_from_for_vec!(u8);
impl_from_for_vec!(i8);
impl_from_for_vec!(u16);
impl_from_for_vec!(i16);
impl_from_for_vec!(u32);
impl_from_for_vec!(i32);
impl_from_for_vec!(f32);
impl_from_for_vec!(u64);
impl_from_for_vec!(i64);
impl_from_for_vec!(f64);
impl_from_for_vec!(usize);
impl_from_for_vec!(isize);

impl CastNumber for NumberVector {
    fn cast_number(&self, number_type: NumberType) -> NumberVector {
        use NumberType as NT;
        use NumberVector as NV;
        match number_type {
            NT::U8 => NV::U8(self.into()),
            NT::I8 => NV::I8(self.into()),
            NT::U16 => NV::U16(self.into()),
            NT::I16 => NV::I16(self.into()),
            NT::U32 => NV::U32(self.into()),
            NT::I32 => NV::I32(self.into()),
            NT::F32 => NV::F32(self.into()),
            NT::U64 => NV::U64(self.into()),
            NT::I64 => NV::I64(self.into()),
            NT::F64 => NV::F64(self.into()),
            NT::Usize => NV::Usize(self.into()),
            NT::Isize => NV::Isize(self.into()),
        }
    }
}
