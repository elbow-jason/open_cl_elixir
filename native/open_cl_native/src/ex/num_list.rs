use crate::nif;
use crate::nif::{Decoder, Encoder, ErrorT};
use crate::type_id;
use crate::{NumExT, NumTypeEx};
use libc::c_void;
use open_cl_core::ll::{NumCastFrom, NumCastInto, NumberTypeError};
use open_cl_core::{BufferBuilder, MemConfig, NumberType, NumberTyped, NumberTypedT};

#[inline(always)]
unsafe fn _force_cast_vec<T: NumExT, U: NumExT>(v: Vec<T>) -> Vec<U> {
    let casted_vec = Vec::from_raw_parts(v.as_ptr() as *mut U, v.len(), v.capacity());
    std::mem::forget(v);
    casted_vec
}

#[inline(always)]
unsafe fn _force_cast_slice<T: NumExT, U: NumExT>(s: &[T]) -> &[U] {
    std::slice::from_raw_parts(s.as_ptr() as *mut U, s.len())
}

#[inline(always)]
fn _list_number_type<T: NumExT, U>(_val: &Vec<T>) -> NumberType {
    T::number_type()
}

#[inline(always)]
unsafe fn _slice_to_vec<T: NumExT, U: NumExT>(s: &[T]) -> Vec<U> {
    _force_cast_vec::<T, U>(s.to_vec())
}

#[inline(always)]
unsafe fn _vec_as_slice<T: NumExT, U: NumExT>(s: &Vec<T>) -> &[U] {
    _force_cast_slice::<T, U>(s)
}

#[inline(always)]
fn _vec_len<T: NumExT, U>(s: &Vec<T>) -> usize {
    s.len()
}

#[inline(always)]
fn _vec_capacity<T: NumExT, U>(s: &Vec<T>) -> usize {
    s.capacity()
}

pub trait VecOps<T>
where
    T: NumExT,
{
    fn push(&mut self, num: T) -> nif::Result<()>;
    fn extend(&mut self, other: Vec<T>) -> nif::Result<()>;
    fn extend_from_slice(&mut self, other: &[T]) -> nif::Result<()>;
}

pub trait VecProps {
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn _mismatch<T: NumberTyped, U: NumberTypedT>(item: &T) -> nif::Error {
    NumberTypeError::Mismatch(item.number_type(), U::number_type()).error()
}

macro_rules! apply_op_mut_num_list {
    ($list:expr, $t:ty, $op:ident, $( $arg:expr ),*) => {{
        let mut tmp = unsafe { $list.dangerously_owned_inner_vec::<$t>() };
        tmp.$op($( $arg ),*);
        $list.ptr = tmp.as_mut_ptr() as *mut c_void;
        $list.length = tmp.len();
        $list.capacity = tmp.capacity();
        std::mem::forget(tmp);
    }}
}

macro_rules! impl_mut_vec_ops {
    ($t:ty, $variant:ident) => {
        impl VecOps<$t> for NumList {
            fn push(&mut self, num: $t) -> nif::Result<()> {
                self.type_check_t::<$t>()?;
                Ok(apply_op_mut_num_list!(self, $t, push, num))
            }

            fn extend(&mut self, nums: Vec<$t>) -> nif::Result<()> {
                self.type_check_t::<$t>()?;
                Ok(apply_op_mut_num_list!(self, $t, extend, nums))
            }

            fn extend_from_slice(&mut self, nums: &[$t]) -> nif::Result<()> {
                self.type_check_t::<$t>()?;
                Ok(apply_op_mut_num_list!(self, $t, extend_from_slice, nums))
            }
        }
    };
}

unsafe fn _unchecked_num_cast_from_num_list<T, U>(list: NumList) -> Option<Vec<U>>
where
    T: NumExT,
    U: NumExT,
    Vec<T>: NumCastInto<Vec<U>>,
{
    list.into_inner::<T>().ok()?.num_cast_into()
}

macro_rules! impl_num_cast_from {
    ($t:ty) => {
        impl NumCastFrom<NumList> for Vec<$t> {
            fn num_cast_from(list: NumList) -> Option<Vec<$t>> {
                unsafe {
                    apply_type_id! {
                        type_id: list.tid(),
                        right_t: $t,
                        func: _unchecked_num_cast_from_num_list,
                        args: [list],
                        default: {
                            panic!("Failed to NumCastFrom due to unmatched type id");
                        }
                    }
                }
            }
        }
    };
}

struct Noop();

#[derive(Debug)]
pub struct NumList {
    t: NumberType,
    ptr: *mut c_void,
    length: usize,
    capacity: usize,
}

impl NumberTyped for NumList {
    fn number_type(&self) -> NumberType {
        self.t
    }
}

impl BufferBuilder for NumList {
    fn buffer_len(&self) -> usize {
        self.length
    }

    fn buffer_ptr(&self) -> *mut libc::c_void {
        self.ptr // as *mut libc::c_void
    }

    fn mem_config(&self) -> MemConfig {
        MemConfig::for_data()
    }
}

unsafe fn _unchecked_drop_num_list<T: NumExT, U>(list: &mut NumList) {
    list.dangerously_owned_inner_vec::<T>();
}

impl Drop for NumList {
    fn drop(&mut self) {
        unsafe {
            apply_type_id! {
                type_id: self.tid(),
                right_t: Noop,
                func: _unchecked_drop_num_list,
                args: [self],
                default: {
                    panic!("Failed to drop NumList due to unhandled type id");
                }
            }
        }
    }
}

impl VecProps for NumList {
    fn len(&self) -> usize {
        self.length
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

macro_rules! impl_all {
    ( $( $t:ty => $variant:ident ),*) => {
        $(
             impl_mut_vec_ops!($t, $variant);
             impl_num_cast_from!($t);
            // impl_from_vec!($t, $variant);
        )*
    }
}

impl_all! {
    u8 => U8,
    i8 => I8,
    u16 => U16,
    i16 => I16,
    u32 => U32,
    i32 => I32,
    f32 => F32,
    u64 => U64,
    i64 => I64,
    f64 => F64,
    usize => Usize
}

fn list_iterator_to_vec<'a, T: NumExT + Decoder<'a>>(
    list_iterator: nif::ListIterator<'a>,
) -> nif::Result<Vec<T>> {
    list_iterator
        .map(|x| x.decode::<T>())
        .collect::<nif::Result<Vec<T>>>()
}

fn _num_list_from_iter<'a, T: NumExT + Decoder<'a>>(
    iter: nif::ListIterator<'a>,
) -> nif::Result<NumList> {
    Ok(NumList::from_vec(list_iterator_to_vec::<'a, T>(iter)?))
}

unsafe fn _clone_inner<T: NumExT, U: NumExT>(list: &NumList) -> Vec<U> {
    let data = list.dangerously_owned_inner_vec::<T>();
    let cloned = data.clone();
    std::mem::forget(data);
    _force_cast_vec(cloned)
}

unsafe fn _take_inner<T: NumExT, U: NumExT>(list: NumList) -> Vec<U> {
    let data = list.dangerously_owned_inner_vec::<T>();
    std::mem::forget(list);
    _force_cast_vec(data)
}

unsafe fn _num_list_slice<T: NumExT, U: NumExT>(list: &NumList) -> &[U] {
    let mut data = list.dangerously_owned_inner_vec::<T>();
    let slc: &[T] = std::slice::from_raw_parts(data.as_mut_ptr(), data.len());
    std::mem::forget(data);
    _force_cast_slice(slc)
}

unsafe fn _extend_from_list<T, U>(l1: &mut NumList, l2: &NumList) -> nif::Result<()>
where
    T: NumExT,
    NumList: VecOps<T>,
{
    let l2vec = l2.dangerously_owned_inner_vec::<T>();
    let result = l1.extend_from_slice(&l2vec[..]);
    std::mem::forget(l2vec);
    result
}

fn _push_term<'a, T: NumExT + Decoder<'a>, U>(
    list: &mut NumList,
    term: nif::Term<'a>,
) -> nif::Result<()>
where
    T: NumExT,
    NumList: VecOps<T>,
{
    let num = term.decode::<T>()?;
    unsafe {
        let mut inner = list.dangerously_owned_inner_vec::<T>();
        inner.push(num);
        list.capacity = inner.capacity();
        list.length = inner.len();
        list.ptr = inner.as_mut_ptr() as *mut c_void;
        std::mem::forget(inner);
    }
    Ok(())
}

impl NumList {
    pub unsafe fn from_raw_parts(
        t: NumberType,
        ptr: *mut c_void,
        length: usize,
        capacity: usize,
    ) -> NumList {
        NumList {
            t,
            ptr,
            length,
            capacity,
        }
    }

    pub fn tid(&self) -> std::any::TypeId {
        self.number_type().number_type_id()
    }

    unsafe fn dangerously_owned_inner_vec<T>(&self) -> Vec<T> {
        Vec::from_raw_parts(self.ptr as *mut T, self.length, self.capacity)
    }

    pub fn type_check_t<T: NumberTypedT>(&self) -> nif::Result<()> {
        self.t
            .type_check(&T::number_type())
            .map_err(nif::ErrorT::error)
    }

    pub fn from_vec<T: NumExT>(mut nums: Vec<T>) -> NumList {
        let list = NumList {
            t: T::number_type(),
            ptr: nums.as_mut_ptr() as *mut c_void,
            length: nums.len(),
            capacity: nums.capacity(),
        };
        std::mem::forget(nums);
        list
    }

    pub fn from_num_typed_iter<'a>(
        num_type_ex: NumTypeEx,
        iter: nif::ListIterator<'a>,
    ) -> nif::Result<NumList> {
        apply_num_type_ex1!(num_type_ex, _num_list_from_iter, [iter])
    }

    pub fn clone_inner<T: NumExT>(&self) -> nif::Result<Vec<T>> {
        self.type_check_t::<T>()?;
        Ok(unsafe {
            apply_type_id! {
                type_id: self.tid(),
                right_t: T,
                func: _clone_inner,
                args: [self],
                default: panic!("Unmatched type_id during clone_inner")
            }
        })
    }
    pub fn into_inner<T: NumExT>(self) -> nif::Result<Vec<T>> {
        self.type_check_t::<T>()?;
        Ok(unsafe {
            apply_type_id! {
            type_id: self.tid(),
            right_t: T,
            func: _take_inner,
            args: [self],
            default: panic!("Unmatched type_id during into_inner")
            }
        })
    }

    pub fn as_slice<T: NumExT>(&self) -> nif::Result<&[T]> {
        self.type_check_t::<T>()?;
        Ok(unsafe { std::slice::from_raw_parts(self.ptr as *const T, self.length) })
    }

    pub fn as_mut_slice<T: NumExT>(&mut self) -> nif::Result<&mut [T]> {
        self.type_check_t::<T>()?;
        Ok(unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut T, self.length) })
    }

    pub fn extend_from_list(&mut self, other: &NumList) -> nif::Result<()> {
        self.t.type_check(&other.t).map_err(|_e| {
            nif::error_string(format!(
                "NumberType mismatch for extend_with_list {:?} vs {:?}",
                self.t, other.t
            ))
        })?;
        Ok(unsafe {
            apply_type_id! {
                type_id: self.tid(),
                right_t: Noop,
                func: _extend_from_list,
                args: [self, other],
                default: Err(nif::error_string("Unmatched type_id during extend_from_list".to_owned()))
            }
        })?
    }

    pub fn push_term(&mut self, term: nif::Term) -> nif::Result<()> {
        apply_type_id! {
            type_id: self.tid(),
            right_t: Noop,
            func: _push_term,
            args: [self, term],
            default: Err(nif::error_string("Unmatched type_id during push_term".to_owned()))
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut c_void {
        self.ptr
    }

    pub fn as_ptr(&mut self) -> *const c_void {
        self.ptr as *const c_void
    }
}

unsafe fn _unchecked_encode_num_list<'a, T, U>(list: &NumList, env: nif::Env<'a>) -> nif::Term<'a>
where
    T: NumExT,
    Vec<T>: nif::Encoder,
{
    let data = list.dangerously_owned_inner_vec::<T>();
    let term = data.encode(env);
    std::mem::forget(data);
    term
}

impl nif::Encoder for NumList {
    fn encode<'a>(&self, env: nif::Env<'a>) -> nif::Term<'a> {
        unsafe {
            apply_type_id! {
                type_id: self.tid(),
                right_t: Noop,
                func: _unchecked_encode_num_list,
                args: [self, env],
                default: panic!("Unexpected type_id during encode")
            }
        }
    }
}
