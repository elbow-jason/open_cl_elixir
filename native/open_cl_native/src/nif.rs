pub use rustler::{
    resource::ResourceArc, types::atom::Atom, Decoder, Encoder, Env, Error, ListIterator, NifMap,
    NifRecord, NifResult as Result, NifStruct as Struct, NifStruct, NifTuple, NifUnitEnum,
    NifUntaggedEnum, Term,
};

use crate::ex::StatusCodeErrorEx;
use open_cl_core::ll::numbers::NumberTypeError;
use open_cl_core::Error as OpenCLError;
use open_cl_core::StatusCodeError;
use std::convert::AsRef;

pub trait ErrorT {
    fn error(self) -> Error;
}

impl ErrorT for Error {
    fn error(self) -> Error {
        self
    }
}

pub fn error_string<T: AsRef<str>>(e: T) -> Error {
    Error::Term(Box::new(e.as_ref().to_owned()))
}

impl ErrorT for StatusCodeError {
    fn error(self) -> Error {
        StatusCodeErrorEx::new(self).error()
    }
}

impl ErrorT for StatusCodeErrorEx {
    fn error(self) -> Error {
        Error::Term(Box::new(self))
    }
}

impl ErrorT for OpenCLError {
    fn error(self) -> Error {
        Error::Term(Box::new(format!("{:?}", self)))
    }
}

impl ErrorT for NumberTypeError {
    fn error(self) -> Error {
        Error::Term(Box::new(format!("{:?}", self)))
    }
}
