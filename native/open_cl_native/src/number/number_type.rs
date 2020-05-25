// use open_cl_core::{Error, Number};

// use rustler::NifUnitEnum;

// use crate::OutputEx;

// #[derive(Debug, Error, PartialEq, Eq, Clone)]
// pub enum NumberTypeError {
//     #[error("Number Type Mismatch - {:?} vs {:?}", 0, 1)]
//     TypeMismatch(NumberType, NumberType),
// }

// #[derive(NifUnitEnum, Debug, PartialEq, Eq, Clone, Copy)]
// pub enum NumberTypeEx {
//     U8,
//     I8,
//     U16,
//     I16,
//     U32,
//     I32,
//     F32,
//     U64,
//     I64,
//     F64,
//     Usize,
//     Isize,
// }

// impl NumberTyped for NumberTypeEx {
//     pub fn type_check(&self, t: NumberType) -> OutputEx<NumberType> {
//         if *self != t {
//             return Err((*self).mismatch_error(t).into());
//         }
//         Ok(t)
//     }

//     pub fn mismatch_error(self, t: NumberType) -> NumberTypeError {
//         NumberTypeError::TypeMismatch(self, t)
//     }

//     pub fn cast<F, T>(&self, f: F) -> Option<T>
//     where
//         F: FnOnce() -> T,
//         T: Number + NumberTypedT,
//     {
//         if T::number_type_of() == *self {
//             Some(f())
//         } else {
//             None
//         }
//     }

//     pub fn size_of(&self) -> usize {
//         use std::mem::size_of;
//         match self {
//             NumberType::U8 => size_of::<u8>(),
//             NumberType::I8 => size_of::<i8>(),
//             NumberType::U16 => size_of::<u16>(),
//             NumberType::I16 => size_of::<i16>(),
//             NumberType::U32 => size_of::<u32>(),
//             NumberType::I32 => size_of::<i32>(),
//             NumberType::F32 => size_of::<f32>(),
//             NumberType::U64 => size_of::<u64>(),
//             NumberType::I64 => size_of::<i64>(),
//             NumberType::F64 => size_of::<f64>(),
//             NumberType::Usize => size_of::<usize>(),
//             NumberType::Isize => size_of::<isize>(),
//         }
//     }
// }

// pub trait NumberTyped {
//     fn number_type(&self) -> NumberType;

//     fn matches_t<T>(&self) -> bool
//     where
//         T: NumberTypedT,
//     {
//         self.number_type() == T::number_type_of()
//     }

//     fn matches<T>(&self, other: &T) -> bool
//     where
//         T: NumberTyped,
//     {
//         self.number_type() == other.number_type()
//     }

//     fn type_check(&self, t: NumberType) -> OutputEx<NumberType> {
//         self.number_type().type_check(t)
//     }
// }

// pub trait NumberTypedT {
//     fn number_type_of() -> NumberType;

//     fn type_check(t: NumberType) -> OutputEx<NumberType> {
//         Self::number_type_of().type_check(t)
//     }
// }

// macro_rules! impl_number_typed_t {
//     ($t:ident, $variant:ident) => {
//         impl NumberTypedT for $t {
//             fn number_type_of() -> NumberType {
//                 NumberType::$variant
//             }
//         }
//     };
// }

// impl_number_typed_t!(u8, U8);
// impl_number_typed_t!(i8, I8);
// impl_number_typed_t!(u16, U16);
// impl_number_typed_t!(i16, I16);
// impl_number_typed_t!(u32, U32);
// impl_number_typed_t!(i32, I32);
// impl_number_typed_t!(f32, F32);
// impl_number_typed_t!(u64, U64);
// impl_number_typed_t!(i64, I64);
// impl_number_typed_t!(f64, F64);
// impl_number_typed_t!(usize, Usize);
// impl_number_typed_t!(isize, Isize);

// impl NumberTyped for NumberType {
//     fn number_type(&self) -> NumberType {
//         *self
//     }
// }
