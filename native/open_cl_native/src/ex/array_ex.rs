use std::sync::RwLock;
use std::fmt;
use std::ops::Deref;

use rustler::types::atom::Atom;
use rustler::resource::ResourceArc;
use rustler::{Encoder, NifStruct};

use crate::traits::NativeWrapper;
use crate::atoms;
use crate::ex::number_ex::{
    NumEx,
    NumberVector,
    NumberTyped,
    NumberTypedT,
    NumberType,
    Number,
    CastNumber
};

#[derive(Debug)]
pub struct Array {
    data: RwLock<NumberVector>
}

impl NumberTyped for Array {
    fn number_type(&self) -> NumberType {
        self.data.read().unwrap().number_type()
    }
}

impl CastNumber for Array {
    fn cast_number(&self, number_type: NumberType) -> Array {
        let number_vector = self.data.read().unwrap().cast_number(number_type);
        Array::new(number_vector)
    }
}

impl Array {
    pub fn new(data: NumberVector) -> Array {
        Array {
            data: RwLock::new(data)
        }
    }

    pub fn length(&self) -> usize {
        self.data.read().unwrap().length()
    }

    pub fn clone_number_vector(&self) -> NumberVector {
        self.data.read().unwrap().clone()
    }

    pub fn extend(&self, number_vector: &NumberVector) {
        let mut data = self.data.write().unwrap();
        data.extend(number_vector);
    }

    fn into_resource_arc(self) -> ResourceArc<Self> {
        ResourceArc::new(self)
    }
}

#[derive(NifStruct)]
#[must_use]
#[module = "OpenCL.Array"]
pub struct ArrayEx {
    __native__: ResourceArc<Array>,
}

impl fmt::Debug for ArrayEx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayEx {{ native: {:?} }}", self.native())
    }
}

impl NativeWrapper<Array> for ArrayEx {
    fn native(&self) -> &Array {
        &self.__native__
    }
}

impl ArrayEx {
    pub fn filled_with<T>(number: T, count: usize) -> ArrayEx where T: NumberTypedT + Number, NumberVector: From<Vec<T>> {
        let numbers: Vec<T> = std::iter::repeat(number).take(count).collect();
        let number_vector = NumberVector::from(numbers);
        ArrayEx::from_number_vector(number_vector)
    }

    pub fn from_number_vector(number_vector: NumberVector) -> ArrayEx {
        ArrayEx::from_array(Array::new(number_vector))
    }

    pub fn from_array(array: Array) -> ArrayEx {
        ArrayEx {
            __native__: array.into_resource_arc(),
        }
    }

    pub fn length(&self) -> usize {
        self.native().length()
    }

    pub fn number_vector(&self) -> NumberVector {
        self.native().clone_number_vector()
    }

    pub fn extend(&self, number_vector: &NumberVector) {
        self.native().extend(number_vector)
    }

    pub fn extend_from_array(&self, other: &ArrayEx) {
        if self.is_same_array(other) {
            let mut data = self.native().data.write().unwrap();
            let copied = data.clone();
            data.extend(&copied);
        } else {
            let other_data = other.native().data.read().unwrap();
            let mut data = self.native().data.write().unwrap();
            data.extend(&other_data);
        }
    }
    
    pub fn push(&self, item: NumEx) {
        let mut data = self.native().data.write().unwrap();
        data.push(item);
    }

    pub fn is_same_array(&self, other: &ArrayEx) -> bool {
        std::ptr::eq(self.__native__.deref(), other.__native__.deref())
    }
}

impl NumberTyped for ArrayEx {
    fn number_type(&self) -> NumberType {
        self.native().number_type()
    }   
}

impl CastNumber for ArrayEx {
    fn cast_number(&self, number_type: NumberType) -> ArrayEx {
        let array = self.native().cast_number(number_type);
        ArrayEx::from_array(array)
    }
}

#[rustler::nif]
fn array_new(number_type: NumberType, number_vector: NumberVector) -> ArrayEx {
    // TODO fix me. The decoding should
    let casted = number_vector.cast_number(number_type);
    ArrayEx::from_number_vector(casted)
}

#[rustler::nif]
fn array_push(array: ArrayEx, item: NumEx) -> Atom {
    array.push(item);
    atoms::ok()
}

#[rustler::nif]
fn array_data(array: ArrayEx) -> NumberVector {
    array.number_vector()
}

#[rustler::nif]
fn array_length(array: ArrayEx) -> usize {
    array.length()
}

#[rustler::nif]
fn array_extend(array: ArrayEx, number_vector: NumberVector) -> Atom {
    array.extend(&number_vector);
    atoms::ok()
}

#[rustler::nif]
fn array_extend_from_array(array: ArrayEx, other: ArrayEx) -> Atom {
    array.extend_from_array(&other);
    atoms::ok()
}

#[rustler::nif]
fn array_new_filled_with(number_type: NumberType, filler: NumEx, count: usize) -> ArrayEx {
    let casted = filler.cast_number(number_type);
    match casted {
        NumEx::U8(number) => ArrayEx::filled_with::<u8>(number, count),
        NumEx::I8(number) => ArrayEx::filled_with::<i8>(number, count),
        NumEx::U16(number) => ArrayEx::filled_with::<u16>(number, count),
        NumEx::I16(number) => ArrayEx::filled_with::<i16>(number, count),
        NumEx::U32(number) => ArrayEx::filled_with::<u32>(number, count),
        NumEx::I32(number) => ArrayEx::filled_with::<i32>(number, count),
        NumEx::F32(number) => ArrayEx::filled_with::<f32>(number, count),
        NumEx::U64(number) => ArrayEx::filled_with::<u64>(number, count),
        NumEx::I64(number) => ArrayEx::filled_with::<i64>(number, count),
        NumEx::F64(number) => ArrayEx::filled_with::<f64>(number, count),
        NumEx::Usize(number) => ArrayEx::filled_with::<usize>(number, count),
        NumEx::Isize(number) => ArrayEx::filled_with::<isize>(number, count),
        got => panic!("ArrayEx::filled_with called with an invalid filler {:?}", got),
    }   
}

#[rustler::nif]
fn array_number_type(array: ArrayEx) -> NumberType {
    array.number_type()
}

#[rustler::nif]
fn array_cast(array: ArrayEx, number_type: NumberType) -> ArrayEx {
    array.cast_number(number_type)
}

