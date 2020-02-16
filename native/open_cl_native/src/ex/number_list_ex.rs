use std::marker::PhantomData;
// use std::ptr::Unique;

use rustler::NifUntaggedEnum;

use crate::{NumberEx, NumberType, NumberTyped, OutputEx, CastNumber, NumEx};
use NumberType as NT;

#[derive(Debug)]
pub struct RuntimeNumberList {
    _number_type: NumberType,
    _len: usize,
    _capacity: usize,
    _ptr: *mut libc::c_void,
}

fn _cast_to_type<T: NumberEx + Into<NumEx>>(rt_list: &RuntimeNumberList, other_type: NumberType) -> RuntimeNumberList {
    let v1: Vec<T> = unsafe { rt_list.borrow_vec() };
    let v2: Vec<NumEx> = v1.iter().map(|num| (*num).into()).collect();
    std::mem::forget(v1);
    apply_number_type!(other_type, cast_vec_to_rt_list, [v2])
}

fn cast_vec_to_rt_list<T: NumberEx + From<NumEx>>(data: Vec<NumEx>) -> RuntimeNumberList {
    RuntimeNumberList::from_vec(data.iter().map(|num| From::from(*num)).collect::<Vec<T>>())
}

impl CastNumber for RuntimeNumberList {
    fn cast_number(&self, number_type: NumberType) -> RuntimeNumberList {
        apply_number_type!(self._number_type, _cast_to_type, [self, number_type])
    }
}

impl RuntimeNumberList {
    pub fn len(&self) -> usize {
        self._len
    }

    pub fn is_empty(&self) -> bool {
        self._len == 0
    }

    pub fn capacity(&self) -> usize {
        self._capacity
    }

    pub fn push<N: NumberEx>(&mut self, item: N) {
        unsafe {
            let mut data: Vec<N> = self.borrow_vec();
            data.push(item);
            self.update_from_borrowed(data);
        }
    }

    pub fn extend<N: NumberEx>(&mut self, other: Vec<N>) {
        assert!(!std::ptr::eq(self._ptr as *const N, other.as_ptr()));
        unsafe {
            let mut data: Vec<N> = self.borrow_vec();
            data.extend(other);
            self.update_from_borrowed(data);
        }
    }

    pub fn extend_from_slice<N: NumberEx>(&mut self, other: &[N]) {
        assert!(!std::ptr::eq(self._ptr as *const N, other.as_ptr()));
        unsafe {
            let mut data: Vec<N> = self.borrow_vec();
            data.extend_from_slice(other);
            self.update_from_borrowed(data);
        }
    }

    pub unsafe fn borrow_vec<N: NumberEx>(&self) -> Vec<N> {
        assert_eq!(N::number_type_of(), self._number_type);
        Vec::from_raw_parts(self._ptr as *mut N, self._len, self._capacity)
    }

    unsafe fn update_from_borrowed<N: NumberEx>(&mut self, mut data: Vec<N>) {
        assert_eq!(N::number_type_of(), self._number_type);
        // assert!(
        //     std::ptr::eq(self._ptr as *const N, data.as_ptr()),
        //     "update_from_borrowed for number_type {:?} encountered invalid pointers {:?} did not match {:?}",
        //     N::number_type_of(),
        //     self._ptr as *const N,
        //     data.as_ptr()
        // );
        self._ptr = data.as_mut_ptr() as *mut libc::c_void;
        self._len = data.len();
        self._capacity = data.capacity();
        std::mem::forget(data);
    }

    unsafe fn unchecked_to_vec<N: NumberEx>(self) -> Vec<N> {
        let new_vec = self.borrow_vec();
        std::mem::forget(self);
        new_vec
    }

    unsafe fn unchecked_as_slice<N: NumberEx>(&self) -> &[N] {
        std::slice::from_raw_parts(self._ptr as *const N, self._len)
    }

    unsafe fn unchecked_as_slice_mut<N: NumberEx>(&mut self) -> &mut [N] {
        std::slice::from_raw_parts_mut(self._ptr as *mut N, self._len)
    }

    pub fn force_cloned_vec<N: NumberEx>(&self) -> Vec<N> {
        assert_eq!(N::number_type_of(), self._number_type);
        unsafe {
            let borrowed = self.borrow_vec();
            let cloned: Vec<N> = borrowed.clone();
            std::mem::forget(borrowed);
            cloned
        } 
    }

    pub fn force_to_vec<N: NumberEx>(self) -> Vec<N> {
        assert_eq!(N::number_type_of(), self._number_type);
        unsafe { self.unchecked_to_vec() }
    }

    pub fn try_to_vec<N: NumberEx>(self) -> OutputEx<Vec<N>> {
        self._number_type.type_check(N::number_type_of())?;
        Ok(unsafe { self.unchecked_to_vec() })
    }

    pub fn force_as_slice<N: NumberEx>(&self) -> &[N] {
        assert_eq!(N::number_type_of(), self._number_type);
        unsafe { self.unchecked_as_slice() }
    }

    pub fn try_as_slice<N: NumberEx>(&self) -> OutputEx<&[N]> {
        self._number_type.type_check(N::number_type_of())?;
        Ok(unsafe { self.unchecked_as_slice() })
    }

    pub fn force_as_slice_mut<N: NumberEx>(&mut self) -> &mut [N] {
        assert_eq!(N::number_type_of(), self._number_type);
        unsafe { self.unchecked_as_slice_mut() }
    }

    pub fn try_as_slice_mut<'a, N: NumberEx>(&'a mut self) -> OutputEx<&'a mut [N]> {
        self._number_type.type_check(N::number_type_of())?;
        Ok(unsafe { self.unchecked_as_slice_mut() })
    }

    pub fn from_vec<N: NumberEx>(mut v: Vec<N>) -> RuntimeNumberList {
        let rt_list = RuntimeNumberList {
            _number_type: N::number_type_of(),
            _ptr: v.as_mut_ptr() as *mut libc::c_void,
            _len: v.len(),
            _capacity: v.capacity(),
        };
        std::mem::forget(v);
        rt_list
    }
}

impl NumberTyped for RuntimeNumberList {
    fn number_type(&self) -> NumberType {
        self._number_type
    }
}

impl<N: NumberEx> From<RuntimeNumberList> for NumberList<N> {
    fn from(rt_list: RuntimeNumberList) -> NumberList<N> {
        NumberList::from_rt_list(rt_list)
    }
}

impl<N: NumberEx> From<NumberList<N>> for RuntimeNumberList {
    fn from(list: NumberList<N>) -> RuntimeNumberList {
        list.into_rt_list()
    }
}

fn _clone_rt_list<N: NumberEx>(rt_list: &RuntimeNumberList) -> RuntimeNumberList {
    unsafe {
        let data: Vec<N> = rt_list.borrow_vec();
        let cloned_vec = data.clone();
        std::mem::forget(data);
        RuntimeNumberList::from_vec(cloned_vec)
    }
}

impl Clone for RuntimeNumberList {
    fn clone(&self) -> RuntimeNumberList {
        match self._number_type {
            NT::U8 => _clone_rt_list::<u8>(self),
            NT::I8 => _clone_rt_list::<i8>(self),
            NT::U16 => _clone_rt_list::<u16>(self),
            NT::I16 => _clone_rt_list::<i16>(self),
            NT::U32 => _clone_rt_list::<u32>(self),
            NT::I32 => _clone_rt_list::<i32>(self),
            NT::F32 => _clone_rt_list::<f32>(self),
            NT::U64 => _clone_rt_list::<u64>(self),
            NT::I64 => _clone_rt_list::<i64>(self),
            NT::F64 => _clone_rt_list::<f64>(self),
            NT::Usize => _clone_rt_list::<usize>(self),
            NT::Isize => _clone_rt_list::<isize>(self),
        }
    }
}

impl Drop for RuntimeNumberList {
    fn drop(&mut self) {
        unsafe {
            match self._number_type {
                NT::U8 => {
                    self.borrow_vec::<u8>();
                }
                NT::I8 => {
                    self.borrow_vec::<i8>();
                }
                NT::U16 => {
                    self.borrow_vec::<u16>();
                }
                NT::I16 => {
                    self.borrow_vec::<i16>();
                }
                NT::U32 => {
                    self.borrow_vec::<u32>();
                }
                NT::I32 => {
                    self.borrow_vec::<i32>();
                }
                NT::F32 => {
                    self.borrow_vec::<f32>();
                }
                NT::U64 => {
                    self.borrow_vec::<u64>();
                }
                NT::I64 => {
                    self.borrow_vec::<i64>();
                }
                NT::F64 => {
                    self.borrow_vec::<f64>();
                }
                NT::Usize => {
                    self.borrow_vec::<usize>();
                }
                NT::Isize => {
                    self.borrow_vec::<isize>();
                }
            }
        }
    }
}

struct NumberList<N: NumberEx> {
    _rt_list: RuntimeNumberList,
    _phantom: PhantomData<N>,
}

impl<N: NumberEx> NumberList<N> {
    pub fn len(&self) -> usize {
        self._rt_list.len()
    }

    pub fn is_empty(&self) -> bool {
        self._rt_list.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self._rt_list.capacity()
    }

    pub fn as_mut_ptr(&mut self) -> *mut N {
        self._rt_list._ptr as *mut N
    }

    pub fn as_ptr(&self) -> *const N {
        self._rt_list._ptr as *const N
    }

    pub fn from_vec(mut v: Vec<N>) -> NumberList<N> {
        let rt_list = RuntimeNumberList {
            _number_type: N::number_type_of(),
            _ptr: v.as_mut_ptr() as *mut libc::c_void,
            _len: v.len(),
            _capacity: v.capacity(),
        };
        std::mem::forget(v);

        NumberList::from_rt_list(rt_list)
    }

    pub fn from_rt_list(rt_list: RuntimeNumberList) -> NumberList<N> {
        assert_eq!(N::number_type_of(), rt_list._number_type);
        NumberList {
            _rt_list: rt_list,
            _phantom: PhantomData,
        }
    }

    pub fn into_rt_list(self) -> RuntimeNumberList {
        self._rt_list
    }

    pub fn to_vec(self) -> Vec<N> {
        self._rt_list.force_to_vec()
    }

    pub fn as_slice(&self) -> &[N] {
        unsafe { self._rt_list.unchecked_as_slice() }
    }

    pub fn as_slice_mut(&mut self) -> &mut [N] {
        unsafe { self._rt_list.unchecked_as_slice_mut() }
    }
}

#[derive(NifUntaggedEnum, Debug)]
pub enum NumberListEx {
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

impl NumberTyped for NumberListEx {
    fn number_type(&self) -> NumberType {
        match self {
            NumberListEx::U8(..) => NumberType::U8,
            NumberListEx::I8(..) => NumberType::I8,
            NumberListEx::U16(..) => NumberType::U16,
            NumberListEx::I16(..) => NumberType::I16,
            NumberListEx::U32(..) => NumberType::U32,
            NumberListEx::I32(..) => NumberType::I32,
            NumberListEx::F32(..) => NumberType::F32,
            NumberListEx::U64(..) => NumberType::U64,
            NumberListEx::I64(..) => NumberType::I64,
            NumberListEx::F64(..) => NumberType::F64,
            NumberListEx::Usize(..) => NumberType::Usize,
            NumberListEx::Isize(..) => NumberType::Isize,
        }
    }
}

impl From<NumberListEx> for RuntimeNumberList {
    fn from(nl_ex: NumberListEx) -> RuntimeNumberList {
        use NumberListEx as NE;

        match nl_ex {
            NE::U8(data) => RuntimeNumberList::from_vec::<u8>(data),
            NE::I8(data) => RuntimeNumberList::from_vec::<i8>(data),
            NE::U16(data) => RuntimeNumberList::from_vec::<u16>(data),
            NE::I16(data) => RuntimeNumberList::from_vec::<i16>(data),
            NE::U32(data) => RuntimeNumberList::from_vec::<u32>(data),
            NE::I32(data) => RuntimeNumberList::from_vec::<i32>(data),
            NE::F32(data) => RuntimeNumberList::from_vec::<f32>(data),
            NE::U64(data) => RuntimeNumberList::from_vec::<u64>(data),
            NE::I64(data) => RuntimeNumberList::from_vec::<i64>(data),
            NE::F64(data) => RuntimeNumberList::from_vec::<f64>(data),
            NE::Usize(data) => RuntimeNumberList::from_vec::<usize>(data),
            NE::Isize(data) => RuntimeNumberList::from_vec::<isize>(data),
        }
    }
}

impl From<RuntimeNumberList> for NumberListEx {
    fn from(rt_nl: RuntimeNumberList) -> NumberListEx {
        use NumberListEx as NE;
        unsafe {
            match rt_nl._number_type {
                NT::U8 => NE::U8(rt_nl.unchecked_to_vec()),
                NT::I8 => NE::I8(rt_nl.unchecked_to_vec()),
                NT::U16 => NE::U16(rt_nl.unchecked_to_vec()),
                NT::I16 => NE::I16(rt_nl.unchecked_to_vec()),
                NT::U32 => NE::U32(rt_nl.unchecked_to_vec()),
                NT::I32 => NE::I32(rt_nl.unchecked_to_vec()),
                NT::F32 => NE::F32(rt_nl.unchecked_to_vec()),
                NT::U64 => NE::U64(rt_nl.unchecked_to_vec()),
                NT::I64 => NE::I64(rt_nl.unchecked_to_vec()),
                NT::F64 => NE::F64(rt_nl.unchecked_to_vec()),
                NT::Usize => NE::Usize(rt_nl.unchecked_to_vec()),
                NT::Isize => NE::Isize(rt_nl.unchecked_to_vec()),
            }
        }
    }
}

// impl NumberListEx {
//     pub fn type_check<T: NumberTypedT>(&self) -> OutputEx<()> {
//         self.number_type().type_check(T::number_type())
//     }

//     pub fn try_into_number_list<N: NumberEx>(self) -> OutputEx<NumberList<N>> {
//         self.type_check::<T>()?;
//     }

//     pub fn try_as_slice<T: ClNumber + NumberTypedT>(&self) -> OutputEx<&[T]> {
//         self.type_check::<T>()?;
//         Ok(self.unchecked_as_slice())
//     }

//     pub unsafe fn unchecked_as_slice<T: ClNumber + NumberTypedT>(&self) -> &[T] {
//         use NumberListEx as L;
//         match self {
//             L::U8(v) => force_cast_slice::<u8, T>(&v[..]),
//             L::I8(v) => force_cast_slice::<i8, T>(&v[..]),
//             L::U16(v) => force_cast_slice::<u16, T>(&v[..]),
//             L::I16(v) => force_cast_slice::<i16, T>(&v[..]),
//             L::U32(v) => force_cast_slice::<u32, T>(&v[..]),
//             L::I32(v) => force_cast_slice::<i32, T>(&v[..]),
//             L::F32(v) => force_cast_slice::<f32, T>(&v[..]),
//             L::U64(v) => force_cast_slice::<u64, T>(&v[..]),
//             L::I64(v) => force_cast_slice::<i64, T>(&v[..]),
//             L::F64(v) => force_cast_slice::<f64, T>(&v[..]),
//             L::Usize(v) => force_cast_slice::<usize, T>(&v[..]),
//             L::Isize(v) => force_cast_slice::<isize, T>(&v[..]),
//         }
//     }

//     pub fn try_iter<T: ClNumber + NumberTypedT>(&self) -> OutputEx<std::slice::Iter<T>> {
//         self.try_as_slice().map(|s| s.iter())
//     }
// }

// #[inline]
// unsafe fn force_cast_slice<T, U>(slc: &[T]) -> &[U] {
//     std::slice::from_raw_parts(slc.as_ptr() as *const U, slc.len())
// }

// impl<T: ClNumber + NumberTypedT> From<Vec<T>> for NumberListEx {
//     fn from(data: Vec<T>) -> NumberListEx {
//         use NumberListEx as L;
//         use NumberType as NT;
//         match T::number_type_of() {
//             NT::U8 => L::U8(data),
//             NT::I8 => L::I8(data),
//             NT::U16 => L::U16(data),
//             NT::I16 => L::I16(data),
//             NT::U32 => L::U32(data),
//             NT::I32 => L::I32(data),
//             NT::F32 => L::F32(data),
//             NT::U64 => L::U64(data),
//             NT::I64 => L::I64(data),
//             NT::F64 => L::F64(data),
//             NT::Usize => L::Usize(data),
//             NT::Isize => L::Isize(data),
//         }
//     }
// }

// macro_rules! cast_primitive_vec {
//     ($t:ty, $data:ident) => {
//         $data.iter().map(|num| NumCast::(*num).unwrap()).collect()
//     };
// }

// impl<T> From<NumberListEx> for Vec<T> {
//     fn f(list: NumberListEx) -> Vec<T> {
//         use NumberListEx as L;
//         match list {
//             L::U8(v) => v.into_iter().map(|num| NumCast::from(num)).collect(),
//             L::I8(v) => v.into_iter().map(|num| NumCast::from(num)).collect(),
//             L::U16(v) => v.into_iter().map(|num| NumCast::from(num)).collect(),
//             L::I16(v) => v.into_iter().map(|num| NumCast::from(num)).collect(),
//             L::U32(v) => v.into_iter().map(|num| NumCast::from(num)).collect(),
//             L::I32(v) => v.into_iter().map(|num| NumCast::from(num)).collect(),
//             L::F32(v) => v.into_iter().map(|num| NumCast::from(num)).collect(),
//             L::U64(v) => v.into_iter().map(|num| NumCast::from(num)).collect(),
//             L::I64(v) => v.into_iter().map(|num| NumCast::from(num)).collect(),
//             L::F64(v) => v.into_iter().map(|num| NumCast::from(num)).collect(),
//             L::Usize(v) => v.into_iter().map(|num| NumCast::from(num)).collect(),
//             L::Isize(v) => v.into_iter().map(|num| NumCast::from(num)).collect(),
//         }
//     }
// }

// impl NumberListEx {
//     fn cast_into_vec<T: NumberTypedT + NumCast + ClNumber>(&self) -> Vec<T> {
//         use NumberListEx as L;
//         if list.matches_t::<T>() {
//             unsafe { self.unchecked_as_slice().to_vec() }
//         } else {
//             match self {
//                 L::U8(v) => v.iter().map(|num| NumCast::from(*num).unwrap()).collect(),
//                 L::I8(v) => v.iter().map(|num| NumCast::from(*num).unwrap()).collect(),
//                 L::U16(v) => v.iter().map(|num| NumCast::from(*num).unwrap()).collect(),
//                 L::I16(v) => v.iter().map(|num| NumCast::from(*num).unwrap()).collect(),
//                 L::U32(v) => v.iter().map(|num| NumCast::from(*num).unwrap()).collect(),
//                 L::I32(v) => v.iter().map(|num| NumCast::from(*num).unwrap()).collect(),
//                 L::F32(v) => v.iter().map(|num| NumCast::from(*num).unwrap()).collect(),
//                 L::U64(v) => v.iter().map(|num| NumCast::from(*num).unwrap()).collect(),
//                 L::I64(v) => v.iter().map(|num| NumCast::from(*num).unwrap()).collect(),
//                 L::F64(v) => v.iter().map(|num| NumCast::from(*num).unwrap()).collect(),
//                 L::Usize(v) => v.iter().map(|num| NumCast::from(*num).unwrap()).collect(),
//                 L::Isize(v) => v.iter().map(|num| NumCast::from(*num).unwrap()).collect(),
//             }
//         }
//     }
// }

// impl<T: NumberEx> From<Vec<T>> for NumberListEx {
//     fn from(v: Vec<T>) -> NumberListEx {
//         use NumberType as NT;
//         use NumberListEx as L;
//         unsafe {
//             match T::number_type_of() {
//                 NT::U8 => L::U8(force_cast_vec::<T, u8>(v)),
//                 NT::I8 => L::I8(force_cast_vec::<T, i8>(v)),
//                 NT::U16 => L::U16(force_cast_vec::<T, u16>(v)),
//                 NT::I16 => L::I16(force_cast_vec::<T, i16>(v)),
//                 NT::U32 => L::U32(force_cast_vec::<T, u32>(v)),
//                 NT::I32 => L::I32(force_cast_vec::<T, i32>(v)),
//                 NT::F32 => L::F32(force_cast_vec::<T, f32>(v)),
//                 NT::U64 => L::U64(force_cast_vec::<T, u64>(v)),
//                 NT::I64 => L::I64(force_cast_vec::<T, i64>(v)),
//                 NT::F64 => L::F64(force_cast_vec::<T, f64>(v)),
//                 NT::Usize => L::Usize(force_cast_vec::<T, usize>(v)),
//                 NT::Isize => L::Isize(force_cast_vec::<T, isize>(v)),
//             }
//         }
//     }
// }

// fn force_cast_vec<T: NumberTypedT + ClNumber, U: NumberTypedT + ClNumber>(vec_of_t: Vec<T>) -> Vec<U> {
//     assert_eq!(T::number_type_of(), U::number_type_of());
//     let vec_of_u = unsafe {
//         Vec::from_raw_parts(
//             vec_of_t.as_mut_ptr() as *mut U,
//             vec_of_t.len(),
//             vec_of_t.capacity(),
//         )
//     };
//     std::mem::forget(vec_of_t);
//     vec_of_u
// }

// #[inline]
// fn convert_to_vec<T: ToPrimitive, U: NumCast>(data: &[T]) -> Vec<U> {
//     data.iter().map(|num| NumCast::from(*num).unwrap()).collect()
// }

// impl CastNumber for NumberListEx {
//     fn cast_number(&self, number_type: NumberType) -> NumberListEx {
//         use NumberType as NT;
//         use NumberListEx as L;
//         match number_type {
//             NT::U8 => L::U8(self.cast_into_vec()),
//             NT::I8 => L::I8(self.cast_into_vec()),
//             NT::U16 => L::U16(self.cast_into_vec()),
//             NT::I16 => L::I16(self.cast_into_vec()),
//             NT::U32 => L::U32(self.cast_into_vec()),
//             NT::I32 => L::I32(self.cast_into_vec()),
//             NT::F32 => L::F32(self.cast_into_vec()),
//             NT::U64 => L::U64(self.cast_into_vec()),
//             NT::I64 => L::I64(self.cast_into_vec()),
//             NT::F64 => L::F64(self.cast_into_vec()),
//             NT::Usize => L::Usize(self.cast_into_vec()),
//             NT::Isize => L::Isize(self.cast_into_vec()),
//         }
//     }
// }
