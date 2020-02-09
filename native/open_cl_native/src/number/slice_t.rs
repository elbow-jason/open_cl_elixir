// use libc::c_void;

// use crate::{NumberType, NumberTyped, NumberTypedT, OutputEx};
// use opencl_core::ll::{BufferCreator, ClNumber, SizeAndPtr, MemConfig};

// #[derive(Debug, Clone)]
// pub struct SliceT<'a> {
//     t: NumberType,
//     ptr: &'a c_void,
//     len: usize
// }

// impl<'a> NumberTyped for SliceT<'a> {
//     fn number_type(&self) -> NumberType {
//         self.t
//     }
// }

// impl<'a> SliceT<'a> {
    
//      pub unsafe fn from_raw_parts(t: NumberType, ptr: &'a c_void, len: usize) -> SliceT<'a> {
//         SliceT { t, ptr, len }
//     }

//     pub fn try_clone_vec<T: ClNumber + NumberTypedT>(&self) -> OutputEx<Vec<T>> {
//         self.type_check(T::number_type_of())?;
//         Ok(self.unchecked_vec())
//     }

//     pub fn try_as_primitive_slice<T: ClNumber + NumberTypedT>(&self) -> OutputEx<&'a[T]> {
//         self.type_check(T::number_type_of())?;
//         Ok(self.unchecked_as_primitive_slice())
//     }

//     pub unsafe fn unchecked_as_primitive_slice<T: ClNumber + NumberTypedT>(&self) -> &'a[T] {
//         std::slice::from_raw_parts(self.ptr, self.len)
//     }

//     pub unsafe fn unchecked_vec<T: ClNumber + NumberTypedT>(&self) -> Vec<T> {
//         let mut output: Vec<T> = Vec::with_capacity(self.len);    
//         output.copy_from_slice(self.unchecked_as_slice());
//         output
//     }
// }

// impl<'a, T: ClNumber + NumberTypedT> From<&'a [T]> for SliceT<'a> {
//     fn from(nums: &'a [T]) -> SliceT<'a> {
//         unsafe {
//             SliceT::from_raw_parts(
//                 T::number_type_of(),
//                 nums.as_ptr() as &'a c_void,
//                 nums.len()
//             )
//         }
//     }
// }


// impl<'a, T: ClNumber> BufferCreator<T> for SliceT<'a> {
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