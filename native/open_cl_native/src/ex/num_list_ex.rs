use crate::ex::number_ex::*;
use crate::nif;
use crate::NumList;

macro_rules! deflist {
    ($cl_pascal:ident, $rust_t:ty) => {
        paste::item! {
            #[derive(nif::NifTuple, Debug)]
            pub struct [<$cl_pascal ListEx>]([<$cl_pascal Tag>], Vec<$rust_t>);

            impl [<$cl_pascal ListEx>] {
                pub fn into_vec(self) -> Vec<$rust_t> {
                    self.1
                }

                pub fn as_slice(&self) -> &[$rust_t] {
                    &self.1[..]
                }

                pub fn as_ptr(&self) -> *const $rust_t {
                    self.1.as_ptr()
                }
            }

        }
    };
}

deflist!(Char, i8);
deflist!(Uchar, u8);
deflist!(Short, i16);
deflist!(Ushort, u16);
deflist!(Int, i32);
deflist!(Uint, u32);
deflist!(Long, i64);
deflist!(Ulong, u64);
deflist!(SizeT, usize);
deflist!(Float, f32);
deflist!(Double, f64);

#[derive(nif::NifUntaggedEnum, Debug)]
pub enum NumListEx {
    Char(CharListEx),
    Uchar(UcharListEx),
    Short(ShortListEx),
    Ushort(UshortListEx),
    Int(IntListEx),
    Uint(UintListEx),
    Long(LongListEx),
    Ulong(UlongListEx),
    SizeT(SizeTListEx),
    Float(FloatListEx),
    Double(DoubleListEx),
}

impl NumListEx {
    pub fn into_num_list(self) -> NumList {
        match self {
            NumListEx::Char(val) => NumList::from_vec(val.into_vec()),
            NumListEx::Uchar(val) => NumList::from_vec(val.into_vec()),
            NumListEx::Short(val) => NumList::from_vec(val.into_vec()),
            NumListEx::Ushort(val) => NumList::from_vec(val.into_vec()),
            NumListEx::Int(val) => NumList::from_vec(val.into_vec()),
            NumListEx::Uint(val) => NumList::from_vec(val.into_vec()),
            NumListEx::Long(val) => NumList::from_vec(val.into_vec()),
            NumListEx::Ulong(val) => NumList::from_vec(val.into_vec()),
            NumListEx::SizeT(val) => NumList::from_vec(val.into_vec()),
            NumListEx::Float(val) => NumList::from_vec(val.into_vec()),
            NumListEx::Double(val) => NumList::from_vec(val.into_vec()),
        }
    }
}

// macro_rules! deffloat {
//     ($cl_pascal:ident, $rust_t:ty) => {
//         paste::item! {
//             #[derive(nif::NifUnitEnum, Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
//             pub enum [<$cl_pascal Tag>] {
//                 $cl_pascal,
//             }
//             #[derive(nif::NifTuple, Clone, Copy, Debug, PartialEq, PartialOrd)]
//             pub struct [<$cl_pascal Ex>]([<$cl_pascal Tag>], $rust_t);

//             impl [<$cl_pascal Ex>] {
//                 pub fn into_num(self) -> $rust_t {
//                     self.1
//                 }
//                 pub fn num_ptr(&self) -> *const $rust_t {
//                     &self.1
//                 }
//             }
//         }
//     };
// }
