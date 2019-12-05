use rustler::{NifUntaggedEnum, NifUnitEnum};
use std::convert::TryFrom;
use std::fmt::Debug;
use num_traits::{NumCast, FromPrimitive, ToPrimitive};

pub unsafe trait Number: Debug + Clone + Copy + Default + PartialEq + Send + Sync +
    NumCast + FromPrimitive + ToPrimitive + 'static {}

macro_rules! impl_number_for {
    ($( $t:ty ),+) => {
        $(
            unsafe impl Number for $t {}
        )+
    }
}

impl_number_for!(u8, i8, u16, i16, u32, i32, f32, u64, i64, f64, usize, isize);

macro_rules! impl_number_typed_t {
    ($t:ident, $variant:ident) => {
        impl NumberTypedT for $t {
            fn number_type() -> NumberType {
                NumberType::$variant
            }
        }
    }
}

impl_number_typed_t!(u8, U8);
impl_number_typed_t!(i8, I8);

pub trait NumberTyped {
    fn number_type(&self) -> NumberType;
}

pub trait NumberTypedT {
    fn number_type() -> NumberType;
}

pub trait CastNumber {
    fn cast_number(&self, number_type: NumberType) -> Self;
}


#[derive(NifUnitEnum, Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumberType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    Usize,
    Isize,
}

#[derive(NifUntaggedEnum, Debug, Clone, Copy)]
pub enum NumEx {
    U8(u8),
    I8(i8),
    Other(isize),
    // U16(u16),
    // I16(i16),
    // U32(u32),
    // I32(i32),
    // F32(f32),
    // U64(u64),
    // I64(i64),
    // F64(f64),
    // Usize(usize),
    // Isize(isize),
}



impl CastNumber for NumEx {
    fn cast_number(&self, number_type: NumberType) -> NumEx {
        use NumberType as T;
        use NumEx as N;
        match (number_type, self) {
            (T::U8, N::U8(num)) => N::U8(*num),
            (T::U8, N::I8(num)) => N::U8(*num as u8),
            (T::I8, N::U8(num)) => N::I8(*num as i8),
            (T::I8, N::I8(num)) => N::I8(*num),
            (num_type, num_ex) => panic!("Failed to cast_number {:?} to type {:?}", num_ex, num_type),
        }
    }
}

impl From<NumEx> for i8 {
    fn from(num: NumEx) -> i8 {
        match num {
            NumEx::I8(val) => val as i8,
            NumEx::U8(val) => val as i8,
            num2 => panic!("Failed to cast {:?} to i8", num2),
        }
    }
}

impl From<NumEx> for u8 {
    fn from(num: NumEx) -> u8 {
        match num {
            NumEx::I8(val) => val as u8,
            NumEx::U8(val) => val as u8,
            num2 => panic!("Failed to cast {:?} to u8", num2),
        }
    }
}

impl From<NumEx> for isize {
    fn from(num: NumEx) -> isize {
        match num {
            NumEx::I8(val) => val as isize,
            NumEx::U8(val) => val as isize,
            num2 => panic!("Failed to cast {:?} to isize", num2),
        }
    }
}

impl NumberTyped for NumEx {
   fn number_type(&self) -> NumberType {
        match self {
            NumEx::U8(..) => NumberType::U8,
            NumEx::I8(..) => NumberType::I8,
            _ => panic!("NOOOOASDASSP"),
            // NumSize::Isize(..) => NumberType::Isize,
        }
    }
}

#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum NumberError {
    #[fail(display = "Failed to cast number of type {:?} into number of type {:?}", _0, _1)]
    FailedVectorCast(NumberType, NumberType)
}


// #[derive(Debug)]
#[derive(NifUntaggedEnum, Debug)]
pub enum NumberVector {
    // WTF is this syntax?
    U8(Vec::<u8>),
    I8(Vec::<i8>),
    NotAThing(String)
    
    // U16(Vec<u16>),
    // I16(Vec<i16>),
    // U32(Vec<u32>),
    // I32(Vec<i32>),
    // F32(Vec<f32>),
    // U64(Vec<u64>),
    // I64(Vec<i64>),
    // F64(Vec<f64>),
}

unsafe fn force_cast_vec<T, S>(mut v: Vec<T>) -> Vec<S> {
    let ptr = v.as_mut_ptr();
    let length = v.len();
    let capacity = v.capacity();
    std::mem::forget(v);
    Vec::from_raw_parts(ptr as *mut S, length, capacity)
}

impl NumberVector {
    pub fn new<T>(data: Vec<T>) -> NumberVector where T: NumberTypedT {
        use NumberType as N;
        match T::number_type() {
            N::U8 => NumberVector::U8(unsafe{ force_cast_vec::<T, u8>(data) }),
            N::I8 => NumberVector::I8(unsafe{ force_cast_vec::<T, i8>(data) }),
            got => panic!("Failed to crate NumberVector from vec of type {:?}", got)
        }
    }
    
    pub fn length(&self) -> usize {
        use NumberVector as N;
        match self {
            N::U8(v) => v.len(),
            _ => panic!("Nope!!!!")
        }
    }

    pub fn clone(&self) -> NumberVector {
        use NumberVector as V;
        match self {
            V::U8(v) => V::U8(v.clone()),
            V::I8(v) => V::I8(v.clone()),
            _ => panic!("Nope!!!!")
        }
    }

    pub fn push(&mut self, num_ex: NumEx) {
        use NumberVector as V;
        use NumEx as N;
        match (self, num_ex) {
            (V::U8(ref mut this_vec), N::U8(num)) => this_vec.push(num),
            (this, _) => {
                panic!(
                    "Failed to push {:?} to NumberVector of type {:?} due to type mismatch",
                    num_ex,
                    this.number_type()
                );
            }
        }
    }
    pub fn extend(&mut self, other: &NumberVector) {
        use NumberVector as V;
        match (self, other) {
            (V::U8(ref mut this_vec), V::U8(other_vec)) => this_vec.extend_from_slice(&other_vec[..]),
            (this, _) => {
                panic!(
                    "Failed to extend NumberVector of type {:?} with type {:?} due to type mismatch",
                    this.number_type(),
                    other.number_type()
                );
            }
        }
    }
}

// macro_rules! impl_number_vector_from {
//     ($primitive_t:ty, $variant:ident) => {
       
//     }
// }

// impl_number_vector_from!(u8, U8);
// impl_number_vector_from!(i8, I8);

impl<T> From<Vec<T>> for NumberVector where T: NumberTypedT {
    fn from(v: Vec<T>) -> NumberVector {
        NumberVector::new(v)        
    }
}



impl NumberTyped for NumberVector {
    fn number_type(&self) -> NumberType {
        use NumberVector as NV;
        use NumberType as NT;
        match self {
            NV::U8(..) => NT::U8,
            NV::I8(..) => NT::I8,
            _ => panic!("Nooooope!"),
            // V::I8(..) => N::I8,
            // V::U16(..) => N:U16,
            // V::I16(..) => N:I16,
            // V::U32(..) => N:U32,
            // V::I32(..) => N:I32,
            // V::F32(..) => N:F32,
            // V::U64(..) => N:U64,
            // V::I64(..) => N:I64,
            // V::F64(..) => N:F64,
        }
    }
}

macro_rules! cast_vec_primitive {
    ($t:ty, $vector:ident) => {
        $vector.iter().map(|val| *val as $t).collect()
    } 
}
// fn cast_slice<'a, T, S>(slice: &[T]) -> Vec<S> where T: Copy {
//     slice.iter().map(|val| *val as S).collect()
// }

impl CastNumber for NumberVector {
    fn cast_number(&self, number_type: NumberType) -> NumberVector {
        use NumberType as T;
        use NumberVector as N;
        match (number_type, self) {
            (T::U8, N::U8(v)) => N::U8(v.clone()),
            (T::U8, N::I8(v)) => N::U8(cast_vec_primitive!(u8, v)),
            (T::I8, N::U8(v)) => N::I8(cast_vec_primitive!(i8, v)),
            (T::I8, N::I8(v)) => N::I8(v.clone()),
            (num_type, vector) => {
                let vector_type = vector.number_type();
                panic!("Failed to convert vector of type {:?} to type {:?}", vector_type, num_type);
            }
        }
    }
}


macro_rules! impl_vector_conversion {
    ($t_ex:ident, $t_rs:ty) => {
        impl TryFrom<NumberVector> for Vec<$t_rs> {
            type Error = NumberError;

            fn try_from(vector: NumberVector) -> Result<Vec<$t_rs>, Self::Error> {
                match vector {
                    NumberVector::$t_ex(data) => Ok(data.clone()),
                    _ => {
                        let conv_type = NumberType::$t_ex;
                        let vector_type =  vector.number_type();
                        Err(NumberError::FailedVectorCast(conv_type, vector_type))
                    }
                }
            }
        }

        // impl From<NumberVector> for Vec<$t_rs> {
        //     fn from(vector: NumberVector) -> Vec<$t_rs> {
        //         match vector {
        //             NumberVector::$t_ex(data) => Ok(data.clone()),
        //             _ => {
        //                 let vector_type = vector.number_type();
        //                 let conv_type = stringify!(conv_type);
        //                 panic!("Failed to convert number from {:?} to {}", vector_type, conv_type);
        //             }
        //         }
        //     }
        // }
    }
}

impl_vector_conversion!(U8, u8);




