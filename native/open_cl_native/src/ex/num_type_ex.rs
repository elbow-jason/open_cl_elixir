use crate::nif;
use crate::type_id;

use open_cl_core::{NumberType, NumberTyped, NumberTypedT};

#[derive(nif::NifUnitEnum)]
pub enum NumTypeEx {
    Char,
    Uchar,
    Short,
    Ushort,
    Int,
    Uint,
    Long,
    Ulong,
    Float,
    Double,
    SizeT,
    // ClBool,
    // ClHalf,
}

impl From<NumberType> for NumTypeEx {
    fn from(val: NumberType) -> NumTypeEx {
        match val.number_type_id() {
            type_id::I8 => NumTypeEx::Char,
            type_id::U8 => NumTypeEx::Uchar,
            type_id::I16 => NumTypeEx::Short,
            type_id::U16 => NumTypeEx::Ushort,
            type_id::I32 => NumTypeEx::Int,
            type_id::U32 => NumTypeEx::Uint,
            type_id::I64 => NumTypeEx::Long,
            type_id::U64 => NumTypeEx::Ulong,
            type_id::F32 => NumTypeEx::Float,
            type_id::F64 => NumTypeEx::Double,
            type_id::USIZE => NumTypeEx::SizeT,
            _ => panic!("Unimplemented number type {:?}", val)
            // i8 =>
            // bool => NumTypeEx::ClBool,
        }
    }
}

impl NumberTyped for NumTypeEx {
    fn number_type(&self) -> NumberType {
        match self {
            NumTypeEx::Char => i8::number_type(),
            NumTypeEx::Uchar => u8::number_type(),
            NumTypeEx::Short => i16::number_type(),
            NumTypeEx::Ushort => u16::number_type(),
            NumTypeEx::Int => i32::number_type(),
            NumTypeEx::Uint => u32::number_type(),
            NumTypeEx::Long => i64::number_type(),
            NumTypeEx::Ulong => u64::number_type(),
            NumTypeEx::Float => f32::number_type(),
            NumTypeEx::Double => f64::number_type(),
            NumTypeEx::SizeT => usize::number_type(),
        }
    }
}
