// use std::fmt::{Display, Debug};
// use std::ops::*;
// use std::iter::{Sum, Product};
// use num_traits::{NumCast, FromPrimitive, ToPrimitive, Zero, One};
// // use num_complex::{Complex32, Complex64};

// // Implements an unsafe trait for a list of types.
// // macro_rules! impl_unsafe {
// //     ($trait_name:ident: $( $t:ident ),+) => {

// //     }
// // }

// /// A primitive type usable within `OpenCL` kernels.
// ///
// /// Includes all of the signed, unsigned, and floating point 8 bit - 64 bit
// /// scalar primitives (ex.: cl_char, cl_uint, cl_double) (exception: cl_half)
// /// and their vector counterparts (ex.: cl_int4, cl_float3, cl_short16);
// ///
// /// Can also be implemented for custom types as long as layout and
// /// alignment are conserved between Rust and OpenCL (repr "C").

// // impl_unsafe!(PrimitiveNum:
// //     u8, i8,
// //     u16, i16,
// //     u32, i32, f32, Complex32,
// //     u64, i64, f64, Complex64,
// //     usize, isize
// // );

// // /// A set of traits common to numeric built-in OpenCL scalar and vector
// // /// primitives.
// // ///
// // /// To describe the contents of buffers, etc., prefer using the more general
// // /// `PrimitiveNum` trait unless numeric operations are required.
// // pub unsafe trait BuiltInNum: Debug + Display + Clone + Copy + Default + PartialOrd +
// //     Zero<Output=Self> + One<Output=Self> + Add<Self, Output=Self> + Sub<Self, Output=Self> +
// //     Mul<Self, Output=Self> + Div<Self, Output=Self> + Rem<Self, Output=Self> + PartialEq<Self>
// //     + AddAssign<Self> + SubAssign<Self> + MulAssign<Self> + DivAssign<Self> + RemAssign<Self> +
// //     Sum<Self> + Product<Self> + Send + Sync + 'static {}

// // impl_unsafe!(BuiltInNum: u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);

// /// A scalar type usable within OpenCL kernels.
// ///
// /// To describe the contents of buffers, etc., prefer using the more general
// /// `OclPrm` trait unless numeric operations are required.
// ///
