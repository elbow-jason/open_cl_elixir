use std::fmt;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use rustler::resource::ResourceArc;
use rustler::types::atom::Atom;
use rustler::{Encoder, NifStruct, Error, ListIterator, Decoder};

use crate::{
    atoms, CastNumber, NumEx, NumberEx, NumberListEx, NumberType, NumberTyped, OutputEx,
    RuntimeNumberList,
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
        ArrayEx {
            __native__: ResourceArc::new(arr),
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

impl CastNumber for ArrayEx {
    fn cast_number(&self, number_type: NumberType) -> ArrayEx {
        let arr1 = self.read_lock();
        ArrayEx::from((*arr1).cast_number(number_type))
    }
}

fn _list_iterator_to_vec<'a, T: NumberEx + Decoder<'a>>(iter: ListIterator<'a>) -> Result<Vec<T>, Error> {
    iter.map(|x| x.decode::<T>()).collect()
}

fn _list_iterator_to_rt_list<'a, T: NumberEx + Decoder<'a>>(iter: ListIterator<'a>) -> Result<RuntimeNumberList, Error> {
    let data: Vec<T> = _list_iterator_to_vec(iter)?;
    Ok(RuntimeNumberList::from_vec(data))
}

#[rustler::nif]
fn array_new<'a>(number_type: NumberType, iter: ListIterator<'a>) -> Result<ArrayEx, Error> {
    let rt_list = apply_number_type!(number_type, _list_iterator_to_rt_list, [iter])?;
    Ok(ArrayEx::from(rt_list))
}

fn _push_to_rt_list<T: NumberEx>(rt_list: &mut RuntimeNumberList, num: NumEx) {
    rt_list.push::<T>(From::from(num));
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_push(array: ArrayEx, item: NumEx) -> Atom {
    let number_type = array.number_type();
    let mut rt_list = array.write_lock();
    apply_number_type!(number_type, _push_to_rt_list, [&mut rt_list, item]);
    atoms::ok()
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_data(array: ArrayEx) -> NumberListEx {
    let rt_list = array.read_lock();
    NumberListEx::from(rt_list.clone())
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_length(array: ArrayEx) -> usize {
    array.read_lock().len()
}

fn _extend_rt_with_list_iterator<'a, T: NumberEx + Decoder<'a>>(rt_list: &mut RuntimeNumberList, iter: ListIterator<'a>) -> Result<(), Error> {
    let other = _list_iterator_to_vec::<T>(iter)?;
    Ok(_extend_rt_list_with_slice(rt_list, &other[..]))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_extend_from_list<'a>(array: ArrayEx, iter: ListIterator<'a>) -> Result<(), Error> {
    let number_type = array.number_type();
    let mut rt_list = array.write_lock();
    apply_number_type!(number_type, _extend_rt_with_list_iterator, [&mut rt_list, iter])
}

fn _extend_rt_list<T: NumberEx>(rt_list1: &mut RuntimeNumberList, rt_list2: &RuntimeNumberList) {
    _extend_rt_list_with_slice::<T>(rt_list1, rt_list2.force_as_slice())
}

fn _extend_same_rt_list<T: NumberEx>(rt_list: &mut RuntimeNumberList) {
    let v: Vec<T> = rt_list.force_cloned_vec();
    _extend_rt_list_with_slice(rt_list, &v[..])
}

fn _extend_rt_list_with_slice<T: NumberEx>(rt_list: &mut RuntimeNumberList, other: &[T]) {
    rt_list.extend_from_slice::<T>(other);
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_extend_from_array(array: ArrayEx, other: ArrayEx) -> OutputEx<()> {
    let number_type = array.number_type();
    number_type.type_check(other.number_type())?;
    let mut self_rt_list = array.write_lock();
    if array.is_same_array(&other) {
        apply_number_type!(number_type, _extend_same_rt_list, [&mut self_rt_list]);
        Ok(())
    } else {
        let other_rt_list = other.read_lock();
        apply_number_type!(number_type, _extend_rt_list, [&mut self_rt_list, &other_rt_list]);
        Ok(())
    }
    
    // match other_rt_list.number_type() {
    //     NumberType::U8 => self_rt_list.extend_from_slice::<u8>(other_rt_list.force_as_slice()),
    //     NumberType::I8 => self_rt_list.extend_from_slice::<i8>(other_rt_list.force_as_slice()),
    //     NumberType::U16 => self_rt_list.extend_from_slice::<u16>(other_rt_list.force_as_slice()),
    //     NumberType::I16 => self_rt_list.extend_from_slice::<i16>(other_rt_list.force_as_slice()),
    //     NumberType::U32 => self_rt_list.extend_from_slice::<u32>(other_rt_list.force_as_slice()),
    //     NumberType::I32 => self_rt_list.extend_from_slice::<i32>(other_rt_list.force_as_slice()),
    //     NumberType::F32 => self_rt_list.extend_from_slice::<f32>(other_rt_list.force_as_slice()),
    //     NumberType::U64 => self_rt_list.extend_from_slice::<u64>(other_rt_list.force_as_slice()),
    //     NumberType::I64 => self_rt_list.extend_from_slice::<i64>(other_rt_list.force_as_slice()),
    //     NumberType::F64 => self_rt_list.extend_from_slice::<f64>(other_rt_list.force_as_slice()),
    //     NumberType::Usize => {
    //         self_rt_list.extend_from_slice::<usize>(other_rt_list.force_as_slice())
    //     }
    //     NumberType::Isize => {
    //         self_rt_list.extend_from_slice::<isize>(other_rt_list.force_as_slice())
    //     }
    // }
    
}

fn _array_filled_with<T: NumberEx + Into<T>>(filler: NumEx, count: usize) -> ArrayEx {
    let num: T = filler.into();
    ArrayEx::filled_with::<T>(num, count)
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_new_filled_with(number_type: NumberType, filler: NumEx, count: usize) -> ArrayEx {
    apply_number_type!(number_type, _array_filled_with, [filler, count])
    // let casted = filler.cast_number(number_type);

    // match casted {
    //     NumEx::U8(number) => ArrayEx::filled_with::<u8>(number, count),
    //     NumEx::I8(number) => ArrayEx::filled_with::<i8>(number, count),
    //     NumEx::U16(number) => ArrayEx::filled_with::<u16>(number, count),
    //     NumEx::I16(number) => ArrayEx::filled_with::<i16>(number, count),
    //     NumEx::U32(number) => ArrayEx::filled_with::<u32>(number, count),
    //     NumEx::I32(number) => ArrayEx::filled_with::<i32>(number, count),
    //     NumEx::F32(number) => ArrayEx::filled_with::<f32>(number, count),
    //     NumEx::U64(number) => ArrayEx::filled_with::<u64>(number, count),
    //     NumEx::I64(number) => ArrayEx::filled_with::<i64>(number, count),
    //     NumEx::F64(number) => ArrayEx::filled_with::<f64>(number, count),
    //     NumEx::Usize(number) => ArrayEx::filled_with::<usize>(number, count),
    //     NumEx::Isize(number) => ArrayEx::filled_with::<isize>(number, count),
    // }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_number_type(array: ArrayEx) -> NumberType {
    array.number_type()
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_cast(array: ArrayEx, number_type: NumberType) -> ArrayEx {
    array.cast_number(number_type)
}
