use crate::nif;
use open_cl_core::ll::numbers::NumCastFrom;
use open_cl_core::ll::numbers::{
    Char, Double, Float, Int, Long, Short, SizeT, Uchar, Uint, Ulong, Ushort,
};
use open_cl_core::{KernelArgPtr, Number, NumberType, NumberTyped, NumberTypedT};
use std::fmt::Debug;

pub trait NumExT: Number + NumberTypedT + KernelArgPtr {}

// #[derive(nif::NifUnitEnum, Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
// pub enum CharTag {
//     Char,
// }
// #[derive(nif::NifTuple, Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
// pub struct CharEx(CharTag, i8);

macro_rules! defint {
    ($cl_pascal:ident, $rust_t:ty) => {
        paste::item! {
            #[derive(nif::NifUnitEnum, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub enum [<$cl_pascal Tag>] {
                $cl_pascal,
            }
            #[derive(nif::NifTuple, Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
            pub struct [<$cl_pascal Ex>]([<$cl_pascal Tag>], $rust_t);

            impl [<$cl_pascal Ex>] {
                pub fn into_num(self) -> $rust_t {
                    self.1
                }
                pub fn num_ptr(&self) -> *const $rust_t {
                    &self.1
                }
            }

            // #[derive(nif::NifTuple,)]
            // pub struct [<$cl_pascal ListEx>]([<$cl_pascal Tag>], Vec<$rust_t>);

            // impl [<$cl_pascal ListEx>] {
            //     pub fn into_vec(self) -> Vec<$rust_t> {
            //         self.1
            //     }

            //     pub fn as_slice(&self) -> &[$rust_t] {
            //         &self.1[..]
            //     }

            //     pub fn as_ptr(&self) -> *const $rust_t {
            //         self.1.as_ptr()
            //     }
            // }

        }
    };
}

macro_rules! deffloat {
    ($cl_pascal:ident, $rust_t:ty) => {
        paste::item! {
            #[derive(nif::NifUnitEnum, Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
            pub enum [<$cl_pascal Tag>] {
                $cl_pascal,
            }
            #[derive(nif::NifTuple, Clone, Copy, Debug, PartialEq, PartialOrd)]
            pub struct [<$cl_pascal Ex>]([<$cl_pascal Tag>], $rust_t);

            impl [<$cl_pascal Ex>] {
                pub fn into_num(self) -> $rust_t {
                    self.1
                }
                pub fn num_ptr(&self) -> *const $rust_t {
                    &self.1
                }
            }
        }
    };
}

defint!(Char, i8);
defint!(Uchar, u8);
defint!(Short, i16);
defint!(Ushort, u16);
defint!(Int, i32);
defint!(Uint, u32);
defint!(Long, i64);
defint!(Ulong, u64);
defint!(SizeT, usize);

deffloat!(Float, f32);
deffloat!(Double, f64);

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
    Char(CharEx),
    Uchar(UcharEx),
    Short(ShortEx),
    Ushort(UshortEx),
    Int(IntEx),
    Uint(UintEx),
    Long(LongEx),
    Ulong(UlongEx),
    SizeT(SizeTEx),
    Float(FloatEx),
    Double(DoubleEx),
    // Bool(BoolEx),
}

macro_rules! impl_num_cast_from {
    ($t:ty) => {
        impl NumCastFrom<NumEx> for $t {
            fn num_cast_from(num: NumEx) -> Option<$t> {
                match num {
                    NumEx::Char(val) => <$t>::num_cast_from(val.into_num()),
                    NumEx::Uchar(val) => <$t>::num_cast_from(val.into_num()),
                    NumEx::Short(val) => <$t>::num_cast_from(val.into_num()),
                    NumEx::Ushort(val) => <$t>::num_cast_from(val.into_num()),
                    NumEx::Int(val) => <$t>::num_cast_from(val.into_num()),
                    NumEx::Uint(val) => <$t>::num_cast_from(val.into_num()),
                    NumEx::Long(val) => <$t>::num_cast_from(val.into_num()),
                    NumEx::Ulong(val) => <$t>::num_cast_from(val.into_num()),
                    NumEx::SizeT(val) => <$t>::num_cast_from(val.into_num()),
                    NumEx::Float(val) => <$t>::num_cast_from(val.into_num()),
                    NumEx::Double(val) => <$t>::num_cast_from(val.into_num()),
                    // NumEx::Isize(val) => Char::num_cast_from(val),
                }
            }
        }
    };
}

impl_num_cast_from!(Char);
impl_num_cast_from!(Uchar);
impl_num_cast_from!(Short);
impl_num_cast_from!(Ushort);
impl_num_cast_from!(Int);
impl_num_cast_from!(Uint);
impl_num_cast_from!(Long);
impl_num_cast_from!(Ulong);
impl_num_cast_from!(SizeT);
impl_num_cast_from!(Float);
impl_num_cast_from!(Double);

impl_num_cast_from!(i8);
impl_num_cast_from!(u8);
impl_num_cast_from!(i16);
impl_num_cast_from!(u16);
impl_num_cast_from!(i32);
impl_num_cast_from!(u32);
impl_num_cast_from!(i64);
impl_num_cast_from!(u64);
impl_num_cast_from!(usize);
impl_num_cast_from!(f32);
impl_num_cast_from!(f64);

// impl NumCastFrom<NumEx> for Char {
//     fn num_cast_from(num: NumEx) -> Option<Char> {
//         match num {
//             NumEx::Char(val) => Char::num_cast_from(val.into_num()),
//             NumEx::Uchar(val) => Char::num_cast_from(val.into_num()),
//             NumEx::Short(val) => Char::num_cast_from(val.into_num()),
//             NumEx::Ushort(val) => Char::num_cast_from(val.into_num()),
//             NumEx::Int(val) => Char::num_cast_from(val.into_num()),
//             NumEx::Uint(val) => Char::num_cast_from(val.into_num()),
//             NumEx::Long(val) => Char::num_cast_from(val.into_num()),
//             NumEx::Ulong(val) => Char::num_cast_from(val.into_num()),
//             NumEx::SizeT(val) => Char::num_cast_from(val.into_num()),
//             NumEx::Float(val) => Char::num_cast_from(val.into_num()),
//             NumEx::Double(val) => Char::num_cast_from(val.into_num()),
//             // NumEx::Isize(val) => Char::num_cast_from(val),
//         }
//     }
// }

unsafe impl KernelArgPtr for NumEx {
    fn kernel_arg_size(&self) -> usize {
        match self {
            NumEx::Char(..) => std::mem::size_of::<i8>(),
            NumEx::Uchar(..) => std::mem::size_of::<u8>(),
            NumEx::Short(..) => std::mem::size_of::<i16>(),
            NumEx::Ushort(..) => std::mem::size_of::<u16>(),
            NumEx::Int(..) => std::mem::size_of::<i32>(),
            NumEx::Uint(..) => std::mem::size_of::<u32>(),
            NumEx::Long(..) => std::mem::size_of::<i64>(),
            NumEx::Ulong(..) => std::mem::size_of::<u64>(),
            NumEx::SizeT(..) => std::mem::size_of::<usize>(),
            NumEx::Float(..) => std::mem::size_of::<f32>(),
            NumEx::Double(..) => std::mem::size_of::<f64>(),
        }
    }

    fn kernel_arg_number_type(&self) -> NumberType {
        self.number_type()
    }

    unsafe fn kernel_arg_ptr(&self) -> *const libc::c_void {
        match self {
            NumEx::Char(n) => n.num_ptr() as *const _ as *const libc::c_void,
            NumEx::Uchar(n) => n.num_ptr() as *const _ as *const libc::c_void,
            NumEx::Short(n) => n.num_ptr() as *const _ as *const libc::c_void,
            NumEx::Ushort(n) => n.num_ptr() as *const _ as *const libc::c_void,
            NumEx::Int(n) => n.num_ptr() as *const _ as *const libc::c_void,
            NumEx::Uint(n) => n.num_ptr() as *const _ as *const libc::c_void,
            NumEx::Long(n) => n.num_ptr() as *const _ as *const libc::c_void,
            NumEx::Ulong(n) => n.num_ptr() as *const _ as *const libc::c_void,
            NumEx::SizeT(n) => n.num_ptr() as *const _ as *const libc::c_void,
            NumEx::Float(n) => n.num_ptr() as *const _ as *const libc::c_void,
            NumEx::Double(n) => n.num_ptr() as *const _ as *const libc::c_void,
        }
    }

    unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut libc::c_void {
        self.kernel_arg_ptr() as *mut libc::c_void
    }
}

impl NumberTyped for NumEx {
    fn number_type(&self) -> NumberType {
        match self {
            NumEx::Char(..) => i8::number_type(),
            NumEx::Uchar(..) => u8::number_type(),
            NumEx::Short(..) => i16::number_type(),
            NumEx::Ushort(..) => u16::number_type(),
            NumEx::Int(..) => i32::number_type(),
            NumEx::Uint(..) => u32::number_type(),
            NumEx::Long(..) => i64::number_type(),
            NumEx::Ulong(..) => u64::number_type(),
            NumEx::SizeT(..) => usize::number_type(),
            NumEx::Float(..) => f32::number_type(),
            NumEx::Double(..) => f64::number_type(),
        }
    }
}
