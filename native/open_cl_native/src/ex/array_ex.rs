use crate::nif;
use crate::nif::{Decoder, Encoder, ErrorT};
use crate::type_id;
use crate::{NumExT, NumList, NumTypeEx, VecOps, VecProps};
use open_cl_core::ll::NumCastFrom;
use open_cl_core::{NumberType, NumberTyped};
// use rustler::{Decoder, Encoder, Error, ListIterator, NifStruct, Term};
use std::fmt;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

// NOTE: implement Bool when supported by elbow-jason/open_cl_rust
// use open_cl_core::ll::numbers::Bool;

#[derive(Debug)]
pub struct Array {
    inner: RwLock<NumList>,
}

// macro_rules! apply_for_t {
//     ($t:ty, $func:ident, [ $( $arg:expr ),* ]) => {
//         $func::<$t>($( $arg ),*)
//     }
// }

// macro_rules! apply_type_id {
//     ($type_id:expr, $func:ident, [ $( $arg:ident ),* ]) => {
//         match $type_id {
//             type_id::U8 => apply_for_t!(u8, $func, [ $( $arg )*]),
//             type_id::I8 => apply_for_t!(i8, $func, [ $( $arg )*]),
//             type_id::U16 => apply_for_t!(u16, $func, [ $( $arg )*]),
//             type_id::I16 => apply_for_t!(i16, $func, [ $( $arg )*]),
//             type_id::U32 => apply_for_t!(u32, $func, [ $( $arg )*]),
//             type_id::I32 => apply_for_t!(i32, $func, [ $( $arg )*]),
//             type_id::F32 => apply_for_t!(f32, $func, [ $( $arg )*]),
//             type_id::U64 => apply_for_t!(u64, $func, [ $( $arg )*]),
//             type_id::I64 => apply_for_t!(i64, $func, [ $( $arg )*]),
//             type_id::F64 => apply_for_t!(f64, $func, [ $( $arg )*]),
//             type_id::USIZE => apply_for_t!(usize, $func, [ $( $arg )*])
//         }
//     }
// }

impl NumberTyped for Array {
    fn number_type(&self) -> NumberType {
        self.read_lock().number_type()
    }
}

unsafe impl Send for Array {}
unsafe impl Sync for Array {}

impl Array {
    pub fn new(list: NumList) -> Array {
        Array {
            inner: RwLock::new(list),
        }
    }

    pub fn rw_lock(&self) -> &RwLock<NumList> {
        &self.inner
    }

    pub fn read_lock(&self) -> RwLockReadGuard<NumList> {
        self.inner.read().unwrap()
    }

    pub fn write_lock(&self) -> RwLockWriteGuard<NumList> {
        self.inner.write().unwrap()
    }

    pub fn into_inner(self) -> NumList {
        self.inner.into_inner().unwrap()
    }
}

#[derive(nif::NifStruct)]
#[must_use]
#[module = "OpenCL.Array"]
pub struct ArrayEx {
    __native__: nif::ResourceArc<Array>,
}

impl fmt::Debug for ArrayEx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayEx {{ native: {:?} }}", *self.read_lock())
    }
}

impl ArrayEx {
    // pub fn try_cloned_vec<T: NumberEx>(self) -> OutputEx<Vec<T>> {
    //     let inner = self.read_lock();
    //     let slice = inner.try_as_slice()?;
    //     Ok(slice.to_vec())
    // }

    pub fn filled_with<T: NumExT>(number: T, count: usize) -> ArrayEx {
        ArrayEx::from(NumList::from_vec(vec![number; count]))
    }

    pub fn read_lock(&self) -> RwLockReadGuard<NumList> {
        self.__native__.read_lock()
    }

    pub fn write_lock(&self) -> RwLockWriteGuard<NumList> {
        self.__native__.write_lock()
    }

    pub fn len(&self) -> usize {
        self.read_lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.read_lock().len() == 0
    }

    pub fn is_same_array(&self, other: &ArrayEx) -> bool {
        std::ptr::eq(self.__native__.rw_lock(), other.__native__.rw_lock())
    }
}

impl VecOps<u32> for ArrayEx {
    fn extend(&mut self, other: Vec<u32>) -> nif::Result<()> {
        self.write_lock().extend(other)
    }

    fn push(&mut self, item: u32) -> nif::Result<()> {
        self.write_lock().push(item)
    }

    fn extend_from_slice(&mut self, other: &[u32]) -> nif::Result<()> {
        self.write_lock().extend_from_slice(other)
    }
}

impl VecProps for ArrayEx {
    fn len(&self) -> usize {
        self.read_lock().len()
    }
    fn capacity(&self) -> usize {
        self.read_lock().capacity()
    }
}

impl From<NumList> for ArrayEx {
    fn from(list: NumList) -> ArrayEx {
        ArrayEx::from(Array::new(list))
    }
}

impl From<Array> for ArrayEx {
    fn from(arr: Array) -> ArrayEx {
        ArrayEx {
            __native__: nif::ResourceArc::new(arr),
        }
    }
}

impl NumberTyped for ArrayEx {
    fn number_type(&self) -> NumberType {
        self.read_lock().number_type()
    }
}

fn _iter_to_vec<'a, T: NumExT + nif::Decoder<'a>>(
    iter: nif::ListIterator<'a>,
) -> nif::Result<Vec<T>> {
    let mut output: Vec<T> = Vec::new();
    for n in iter {
        output.push(n.decode()?)
    }
    Ok(output)
}

#[rustler::nif]
fn array_new<'a>(num_type_ex: NumTypeEx, iter: nif::ListIterator<'a>) -> nif::Result<ArrayEx> {
    let num_list = NumList::from_num_typed_iter(num_type_ex, iter)?;
    Ok(ArrayEx::from(num_list))
}

fn _push_term<'a, T, U>(num_list: &mut NumList, term: nif::Term<'a>) -> nif::Result<()>
where
    T: nif::Decoder<'a> + NumExT,
    NumList: VecOps<T>,
{
    let num = term.decode::<T>()?;
    num_list.push(num)
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_push(array: ArrayEx, term: nif::Term) -> nif::Result<()> {
    let mut num_list = array.write_lock();
    num_list.push_term(term)
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_data(env: nif::Env, array: ArrayEx) -> nif::Term {
    (*array.read_lock()).encode(env)
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_length(array: ArrayEx) -> usize {
    array.read_lock().len()
}

struct Noop();

fn _extend_with_iter<'a, T>(list: &mut NumList, iter: nif::ListIterator<'a>) -> nif::Result<()>
where
    T: NumExT + nif::Decoder<'a>,
    NumList: VecOps<T>,
{
    list.extend(_iter_to_vec::<'a, T>(iter).map_err(|e| nif::ErrorT::error(e))?)
        .map_err(|e| e.into())
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_extend_from_list<'a>(array: ArrayEx, iter: nif::ListIterator<'a>) -> nif::Result<()> {
    let mut list = array.write_lock();
    let other = NumList::from_num_typed_iter(NumTypeEx::from(list.number_type()), iter)?;
    list.extend_from_list(&other)
}

fn _extend_same_list<T: NumExT, U>(list: &mut NumList) -> nif::Result<()>
where
    T: NumExT,
    NumList: VecOps<T>,
{
    let v: Vec<T> = list.clone_inner()?;
    _extend_with_slice::<T, Noop>(list, &v[..])
}

fn _extend_with_slice<T, U>(list: &mut NumList, other: &[T]) -> nif::Result<()>
where
    T: NumExT,
    NumList: VecOps<T>,
{
    list.extend_from_slice(other)
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_extend_from_array(array: ArrayEx, other: ArrayEx) -> nif::Result<()> {
    let number_type = array.number_type();
    number_type
        .type_check(&other.number_type())
        .map_err(ErrorT::error)?;
    let mut self_list = array.write_lock();
    if array.is_same_array(&other) {
        apply_type_id! {
            type_id: number_type.number_type_id(),
            right_t: Noop,
            func: _extend_same_list,
            args: [&mut self_list],
            default: Err(nif::error_string("Unmatched type_id during array_extend_from_array with same array"))
        }
    } else {
        let other_list = other.read_lock();
        apply_type_id!(
            type_id: number_type.number_type_id(),
            right_t: Noop,
            func: _extend_with_slice,
            args: [&mut self_list, (*other_list).as_slice()?],
            default: Err(nif::error_string("Unmatched type_id during array_extend_from_array with other array"))
        )
    }
}

fn _array_filled_with<'a, T: NumExT + Decoder<'a>>(
    filler: nif::Term<'a>,
    count: usize,
) -> nif::Result<ArrayEx> {
    let num: T = filler.decode()?;
    Ok(ArrayEx::filled_with::<T>(num, count))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_new_filled_with(
    num_type_ex: NumTypeEx,
    filler: nif::Term,
    count: usize,
) -> nif::Result<ArrayEx> {
    apply_num_type_ex1!(num_type_ex, _array_filled_with, [filler, count])
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_number_type(array: ArrayEx) -> NumTypeEx {
    array.number_type().into()
}

macro_rules! apply_num_type_ex_and_type_id_matrix {
    {
        num_type_ex: $num_type_ex:expr,
        type_id: $type_id:expr,
        func: $func:ident,
        default: $default:expr,
        tag_pairs: { $( $num_type_ex_key:ident => $num_type_ex_t:ty ),* },
        args: $args:tt,
    } => {
        match ($num_type_ex, $type_id) {
            $(
                (NumTypeEx::$num_type_ex_key, type_id::U8) => apply_2_generics!(u8, $num_type_ex_t, $func, $args),
                (NumTypeEx::$num_type_ex_key, type_id::I8) => apply_2_generics!(i8, $num_type_ex_t, $func, $args),
                (NumTypeEx::$num_type_ex_key, type_id::U16) => apply_2_generics!(u16, $num_type_ex_t, $func, $args),
                (NumTypeEx::$num_type_ex_key, type_id::I16) => apply_2_generics!(i16, $num_type_ex_t, $func, $args),
                (NumTypeEx::$num_type_ex_key, type_id::U32) => apply_2_generics!(u32, $num_type_ex_t, $func, $args),
                (NumTypeEx::$num_type_ex_key, type_id::I32) => apply_2_generics!(i32, $num_type_ex_t, $func, $args),
                (NumTypeEx::$num_type_ex_key, type_id::F32) => apply_2_generics!(f32, $num_type_ex_t, $func, $args),
                (NumTypeEx::$num_type_ex_key, type_id::U64) => apply_2_generics!(u64, $num_type_ex_t, $func, $args),
                (NumTypeEx::$num_type_ex_key, type_id::I64) => apply_2_generics!(i64, $num_type_ex_t, $func, $args),
                (NumTypeEx::$num_type_ex_key, type_id::F64) => apply_2_generics!(f64, $num_type_ex_t, $func, $args),
                (NumTypeEx::$num_type_ex_key, type_id::USIZE) => apply_2_generics!(usize, $num_type_ex_t, $func, $args),
            )*
            _ => $default,
        }
    };
}

macro_rules! apply_num_type_ex_and_type_id {
    {
        num_type_ex: $num_type_ex:expr,
        type_id: $type_id:expr,
        func: $func:ident,
        default: $default:expr,
        args: [ $( $arg:expr ),* ],
     } => {
        apply_num_type_ex_and_type_id_matrix! {
            num_type_ex: $num_type_ex,
            type_id: $type_id,
            func: $func,
            default: $default,
            tag_pairs: {
                Char => i8,
                Uchar => u8,
                Short => i16,
                Ushort => u16,
                Uint => u32,
                Int => i32,
                Float => f32,
                Ulong => u64,
                Long => i64,
                Double => f64,
                SizeT => usize
            },
            args: [ $( $arg ),* ],
        }
    };
}

fn _cast_num_list3<T, U>(num_list: &NumList) -> nif::Result<NumList>
where
    T: NumExT,
    U: NumExT + NumCastFrom<T>,
{
    let slice_t: &[T] = num_list.as_slice()?;
    let mut vec_u: Vec<U> = Vec::with_capacity(num_list.len());
    // let mut casted = U::zero();
    for num1 in slice_t.iter() {
        let casted =
            U::num_cast_from(*num1).ok_or_else(|| nif::error_string("failed to cast num_list"))?;
        vec_u.push(casted);
    }
    Ok(NumList::from_vec(vec_u))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn array_cast(array: ArrayEx, num_type_ex: NumTypeEx) -> nif::Result<ArrayEx> {
    let write_locked = array.write_lock();
    let tid = write_locked.tid();
    let num_list2: NumList = apply_num_type_ex_and_type_id! {
        num_type_ex: num_type_ex,
        type_id: tid,
        func: _cast_num_list3,
        default: Err(nif::error_string("failed to array_cast")),
        args: [&write_locked],
    }?;
    Ok(ArrayEx::from(num_list2))
}
