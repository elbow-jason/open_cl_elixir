// use libc::c_void;
// use opencl_core::{MemConfig, ClNumber};
// use opencl_core::ll::{SizeAndPtr, BufferCreator};

// use crate::{OutputEx, NumberTypedT, NumberType, NumberTyped};

// #[derive(Debug, Clone)]
// pub struct MutSliceT<'a> {
//     t: NumberType,
//     ptr: &'a mut libc::c_void,
//     len: usize,
// }

// impl<'a> NumberTyped for MutSliceT<'a> {
//     fn number_type(&self) -> NumberType {
//         self.t
//     }
// }

// impl<'a> MutSliceT<'a> {
//      pub unsafe fn from_raw_parts(t: NumberType, ptr: &'a mut c_void, len: usize) -> MutSliceT<'a> {
//         MutSliceT { t, ptr, len }
//     }

//     pub fn try_clone_vec<T: ClNumber + NumberTypedT>(&self) -> OutputEx<Vec<T>> {
//         self.type_check(T::number_type_of())?;
//         Ok(unsafe { self.unchecked_vec() })
//     }

//     pub fn try_as_primitive_slice<T: ClNumber + NumberTypedT>(&self) -> OutputEx<&'a mut [T]> {
//         self.type_check(T::number_type_of())?;
//         Ok(unsafe { self.unchecked_as_slice() })
//     }

//     pub unsafe fn unchecked_as_primitive_slice_mut<T: ClNumber + NumberTypedT>(&self) -> &'a mut [T] {
//         std::slice::from_raw_parts_mut(self.ptr as *mut T, self.len)
//     }

//     pub unsafe fn unchecked_vec<T: ClNumber + NumberTypedT>(&self) -> Vec<T> {
//         let mut output: Vec<T> = Vec::with_capacity(self.len);    
//         let slc = std::slice::from_raw_parts(self.ptr as *const T, self.len);
//         output.clone_from_slice(slc);
//         output
//     }
// }


// impl<'a, T: ClNumber> BufferCreator<T> for MutSliceT<'a> {
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
