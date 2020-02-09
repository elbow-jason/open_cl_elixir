use std::fmt;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use rustler::resource::ResourceArc;
use rustler::types::atom::Atom;
use rustler::{Encoder, NifStruct};

use crate::{
    atoms, CastNumber, NumEx, NumberType, NumberTyped,
    NumberListEx, OutputEx, RuntimeNumberList, NumberEx
};

#[derive(Debug)]
pub struct Array {
    inner: RwLock<RuntimeNumberList>,
}

impl NumberTyped for Array {
    fn number_type(&self) -> NumberType {
        self.read_lock().number_type()
    }
}

impl From<NumberListEx> for Array {
    fn from(nums: NumberListEx) -> Array {
        Array::new(RuntimeNumberList::from(nums))
    }
}

impl From<Array> for NumberListEx {
    fn from(a: Array) -> NumberListEx {
        NumberListEx::from(a.into_inner())
    }
}


unsafe impl Send for Array {}
unsafe impl Sync for Array {}

impl Array {
    pub fn new(data: RuntimeNumberList) -> Array {
        Array {
            inner: RwLock::new(data),
        }
    }

    pub fn rw_lock(&self) -> &RwLock<RuntimeNumberList> {
        &self.inner
    }

    pub fn read_lock(&self) -> RwLockReadGuard<RuntimeNumberList> {
        self.inner.read().unwrap()
    }

    pub fn write_lock(&self) -> RwLockWriteGuard<RuntimeNumberList> {
        self.inner.write().unwrap()
    }

    pub fn into_inner(self) -> RuntimeNumberList {
        self.inner.into_inner().unwrap()
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
        write!(f, "ArrayEx {{ native: {:?} }}", *self.read_lock())
    }
}

impl ArrayEx {
    pub fn try_cloned_vec<T: NumberEx>(self) -> OutputEx<Vec<T>> {
        let inner = self.read_lock();
        let slice = inner.try_as_slice()?;
        Ok(slice.to_vec())
    }

    pub fn filled_with<T: NumberEx>(number: T, count: usize) -> ArrayEx {
        let numbers: Vec<T> = std::iter::repeat(number).take(count).collect();
        ArrayEx::from(RuntimeNumberList::from_vec(numbers))
    }

    pub fn read_lock(&self) -> RwLockReadGuard<RuntimeNumberList> {
        self.__native__.read_lock()
    }

    pub fn write_lock(&self) -> RwLockWriteGuard<RuntimeNumberList> {
        self.__native__.write_lock()
    }

    pub fn len(&self) -> usize {
        self.read_lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.read_lock().len() == 0
    }

    // pub fn extend<T: NumberEx>(&self, other: Vec<T>) {
    //     self.write_lock().extend(other);
    // }

    // pub fn push<N: NumberEx>(&self, item: N) {
    //     self.write_lock().push(item);
    // }

    pub fn is_same_array(&self, other: &ArrayEx) -> bool {
        std::ptr::eq(self.__native__.rw_lock(), other.__native__.rw_lock())
    }

    // pub fn into_rt_list(self) -> RuntimeNumberList {
    //     self.read_lock().clone()
    // }
}

impl From<RuntimeNumberList> for ArrayEx {
    fn from(list: RuntimeNumberList) -> ArrayEx {
        ArrayEx::from(Array::new(list))
    }
}

impl From<Array> for ArrayEx {
    fn from(arr: Array) -> ArrayEx {
        ArrayEx{
            __native__: ResourceArc::new(arr)
        }
    }
}


impl NumberTyped for ArrayEx {
    fn number_type(&self) -> NumberType {
        self.read_lock().number_type()
    }
}

// #[derive(NifRecord)]
// #[tag = "u8"]
// pub struct U8(pub u8);

// #[derive(NifRecord)]
// #[tag = "i8"]
// pub struct i8(pub i8);

// #[derive(NifRecord)]
// #[tag = "i8"]
// pub struct i8(pub i8);




// impl CastNumber for ArrayEx {
//     fn cast_number(&self, number_type: NumberType) -> ArrayEx {
//         let arr1 = self.read_lock();
//         ArrayEx::from((*arr1).cast_number(number_type))
//     }
// }

#[rustler::nif]
fn array_new(number_type: NumberType, list: NumberListEx) -> ArrayEx {
    let rt_list = RuntimeNumberList::from(list);
    assert_eq!(number_type, rt_list.number_type());
    ArrayEx::from(rt_list)
}

#[rustler::nif]
fn array_push(array: ArrayEx, item: NumEx) -> Atom {
    let mut rt_list = array.write_lock();
    match item {
        NumEx::U8(number) => rt_list.push(number),
        NumEx::I8(number) => rt_list.push(number),
        NumEx::U16(number) => rt_list.push(number),
        NumEx::I16(number) => rt_list.push(number),
        NumEx::U32(number) => rt_list.push(number),
        NumEx::I32(number) => rt_list.push(number),
        NumEx::F32(number) => rt_list.push(number),
        NumEx::U64(number) => rt_list.push(number),
        NumEx::I64(number) => rt_list.push(number),
        NumEx::F64(number) => rt_list.push(number),
        NumEx::Usize(number) => rt_list.push(number),
        NumEx::Isize(number) => rt_list.push(number),
    };
    atoms::ok()
}

#[rustler::nif]
fn array_data(array: ArrayEx) -> NumberListEx {
    let rt_list = array.read_lock();
    NumberListEx::from(rt_list.clone())
}

#[rustler::nif]
fn array_length(array: ArrayEx) -> usize {
    array.read_lock().len()
}

#[rustler::nif]
fn array_extend_from_list(array: ArrayEx, list: NumberListEx) -> Atom {
    use NumberListEx as L;
    let mut rt_list = array.write_lock();
    match list {
        L::U8(data) => rt_list.extend(data),
        L::I8(data) => rt_list.extend(data),
        L::U16(data) => rt_list.extend(data),
        L::I16(data) => rt_list.extend(data),
        L::U32(data) => rt_list.extend(data),
        L::I32(data) => rt_list.extend(data),
        L::F32(data) => rt_list.extend(data),
        L::U64(data) => rt_list.extend(data),
        L::I64(data) => rt_list.extend(data),
        L::F64(data) => rt_list.extend(data),
        L::Usize(data) => rt_list.extend(data),
        L::Isize(data) => rt_list.extend(data),
    };
    atoms::ok()
}

#[rustler::nif]
fn array_extend_from_array(array: ArrayEx, other: ArrayEx) -> Atom {
    if array.is_same_array(&other) {
        return atoms::same_array();
    }
    let mut self_rt_list = array.write_lock();
    let other_rt_list = other.read_lock();
    match other_rt_list.number_type() {
        NumberType::U8 => self_rt_list.extend_from_slice::<u8>(other_rt_list.force_as_slice()),
        NumberType::I8 => self_rt_list.extend_from_slice::<i8>(other_rt_list.force_as_slice()),
        NumberType::U16 => self_rt_list.extend_from_slice::<u16>(other_rt_list.force_as_slice()),
        NumberType::I16 => self_rt_list.extend_from_slice::<i16>(other_rt_list.force_as_slice()),
        NumberType::U32 => self_rt_list.extend_from_slice::<u32>(other_rt_list.force_as_slice()),
        NumberType::I32 => self_rt_list.extend_from_slice::<i32>(other_rt_list.force_as_slice()),
        NumberType::F32 => self_rt_list.extend_from_slice::<f32>(other_rt_list.force_as_slice()),
        NumberType::U64 => self_rt_list.extend_from_slice::<u64>(other_rt_list.force_as_slice()),
        NumberType::I64 => self_rt_list.extend_from_slice::<i64>(other_rt_list.force_as_slice()),
        NumberType::F64 => self_rt_list.extend_from_slice::<f64>(other_rt_list.force_as_slice()),
        NumberType::Usize => self_rt_list.extend_from_slice::<usize>(other_rt_list.force_as_slice()),
        NumberType::Isize => self_rt_list.extend_from_slice::<isize>(other_rt_list.force_as_slice()),
    }
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
    }
}

#[rustler::nif]
fn array_number_type(array: ArrayEx) -> NumberType {
    array.number_type()
}

// #[rustler::nif]
// fn array_cast(array: ArrayEx, number_type: NumberType) -> ArrayEx {
//     array.cast_number(number_type)
// }
