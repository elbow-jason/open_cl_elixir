// use libc::c_void;

// use opencl_core::{BufferCreator, MemConfig, ClNumber};
// use opencl_core::ll::{SizeAndPtr};

// use crate::{OutputEx, NumberType, NumberTyped, NumberTypedT, MutSliceT, SliceT};

// pub struct VecT {
//     t: NumberType,
//     ptr: *mut libc::c_void,
//     len: usize,
//     cap: usize,
// }

// impl NumberTyped for VecT {
//     fn number_type(&self) -> NumberType {
//         self.t
//     }
// }

// impl<T: ClNumber> BufferCreator<T> for VecT {
//     fn buffer_size_and_ptr(&mut self) -> SizeAndPtr<*mut c_void> {
//         SizeAndPtr(
//             self.t.size_of() * self.len(),
//             self.ptr as *mut c_void,
//         )
//     }

//     fn mem_config(&self) -> MemConfig {
//         MemConfig::for_data()
//     }
// }


// impl<T: ClNumber + NumberTypedT> From<Vec<T>> for VecT {
//     fn from(mut v: Vec<T>) -> VecT {
//         unsafe {
//             let vec_t = VecT::from_raw_parts(
//                 T::number_type_of(),
//                 v.as_mut_ptr() as *mut libc::c_void,
//                 v.len(),
//                 v.capacity(),
//             );
//             std::mem::forget(v);
//             vec_t
//         }
//     }
// }

// impl VecT {
//     pub fn len(&self) -> usize {
//         self.len
//     }

//     pub unsafe fn set_len(&mut self, len: usize) {
//         self.len = len;
//     }

//     pub fn capacity(&self) -> usize {
//         self.cap
//     }

//     pub unsafe fn set_capacity(&mut self, cap: usize) {
//         self.cap = cap;
//     }

//     pub fn as_mut_ptr<T: ClNumber + NumberTypedT>(&mut self) -> OutputEx<*mut T> {
//         self.type_check::<T>()?;
//         Ok(self.ptr as *mut T)
//     }

//     pub fn as_ptr<T: ClNumber + NumberTypedT>(&self) -> OutputEx<*const T> {
//         self.type_check::<T>()?;
//         Ok(self.ptr as *const T)
//     }

//     pub unsafe fn from_raw_parts(t: NumberType, ptr: *mut libc::c_void, len: usize, capacity: usize) -> VecT {
//         VecT{t, ptr, len, cap: capacity}
//     }

//     pub fn type_check<T: ClNumber + NumberTypedT>(&self) -> OutputEx<()> {
//         self.t.type_check(T::number_type_of())
//     }

//     pub fn try_to_vec<T: ClNumber + NumberTypedT>(self) -> OutputEx<Vec<T>> {
//         self.type_check::<T>()?;
//         let v: Vec<T> = unsafe {
//             Vec::from_raw_parts(self.ptr as *mut T, self.len, self.cap)
//         };
//         std::mem::forget(self);
//         Ok(v)
//     }

//     pub unsafe fn unchecked_vec<T: ClNumber + NumberTypedT>(&self) -> Vec<T> {
//         Vec::from_raw_parts(self.ptr as *mut T, self.len, self.cap)
//     }

//     pub unsafe fn unchecked_primitive_slice<T: ClNumber + NumberTypedT>(&self) -> &[T] {
//         unsafe { std::slice::from_raw_parts(self.ptr as *const T, self.len) }
//     }

//     pub unsafe fn unchecked_primitive_slice_mut<T: ClNumber + NumberTypedT>(&mut self) -> &[T] {
//         unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut T, self.len) }
//     }

//     pub fn as_slice_t(&self) -> SliceT {
//         SliceT { t: self.t, ptr: self.ptr as &libc::c_void, len: self.len }
//     }

//     pub fn as_mut_slice_t(&mut self) -> MutSliceT {
//         MutSliceT {
//             t: self.t,
//             ptr: self.ptr as &mut libc::c_void,
//             len: self.len,
//         }
//     }
    
//     pub fn try_as_primitive_slice_mut<T: ClNumber + NumberTypedT>(&self) -> OutputEx<&mut [T]> {
//         self.type_check::<T>()?;
//         Ok(unsafe { self.unchecked_primitive_slice_mut() })
//     }

//     pub fn try_push<T: ClNumber + NumberTypedT>(&mut self, item: T) -> OutputEx<()> {
//         self.with_vec(|mut v| {
//             v.push(item);
//             ((), v)
//         })
//     }

//     pub fn try_extend<T: ClNumber + NumberTypedT>(&mut self, items: Vec<T>) -> OutputEx<()> {
//         self.with_vec(|mut v| {
//             v.extend(items);
//             ((), v)
//         })
//     }

//     pub fn try_with_vec<T: ClNumber + NumberTypedT, R, F: FnOnce(Vec<T>) -> (R, Vec<T>)>(&mut self, f: F) -> OutputEx<R> {
//         self.type_check::<T>()?;
//         let (returning, v) = f(unsafe { self.unchecked_vec() });
//         self.len = v.len();
//         self.cap = v.capacity();
//         std::mem::forget(v);
//         Ok(returning)
//     }
// }

// impl Drop for VecT {
//     fn drop(&mut self) {
//         let size_of_t = self.t.size_of();
//         let byte_cap = self.cap * size_of_t;
//         let byte_len = self.len * size_of_t;
//         unsafe {
//             Vec::from_raw_parts(self.ptr as *mut u8, byte_len, byte_cap);
//         }
        
//     }
// }

// impl Clone for VecT {
//     fn clone(&self) -> VecT {
//         use NumberType as NT;
//         match self.t {
//             NT::U8 => VecT::from(self.as_slice::<u8>().unwrap().to_vec()),
//             NT::I8 => VecT::from(self.as_slice::<i8>().unwrap().to_vec()),
//             NT::U16 => VecT::from(self.as_slice::<u16>().unwrap().to_vec()),
//             NT::I16 => VecT::from(self.as_slice::<i16>().unwrap().to_vec()),
//             NT::U32 => VecT::from(self.as_slice::<u32>().unwrap().to_vec()),
//             NT::I32 => VecT::from(self.as_slice::<i32>().unwrap().to_vec()),
//             NT::F32 => VecT::from(self.as_slice::<f32>().unwrap().to_vec()),
//             NT::U64 => VecT::from(self.as_slice::<u64>().unwrap().to_vec()),
//             NT::I64 => VecT::from(self.as_slice::<i64>().unwrap().to_vec()),
//             NT::F64 => VecT::from(self.as_slice::<f64>().unwrap().to_vec()),
//             NT::Usize => VecT::from(self.as_slice::<usize>().unwrap().to_vec()),
//             NT::Isize => VecT::from(self.as_slice::<isize>().unwrap().to_vec()),
//         }
//     }
// }
