use opencl_core::Error as OpenCLError;
use opencl_core::ll::StatusCodeError;

use rustler::Error as RustlerError;
use rustler::{Encoder, Env, Term, NifStruct, NifUnitEnum};

use crate::ex::buffer_ex::BufferError;
use crate::number::NumberTypeError;

pub type OutputEx<T> = Result<T, ErrorEx>;

/// An error for the OpenCL to Elixir interface.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum ErrorEx {
    #[fail(display = "{:?}", _0)]
    OpenCLError(OpenCLError),

    #[fail(display = "{:?}", _0)]
    BufferError(BufferError),

    #[fail(display = "{:?}", _0)]
    NumberTypeError(NumberTypeError),
}

impl From<ErrorEx> for RustlerError {
    fn from(e: ErrorEx) -> RustlerError {
        RustlerError::Term(Box::new(e))
    }
}

macro_rules! impl_error_ex_conv {
    ($err:ident) => {
        impl From<$err> for ErrorEx {
            fn from(err: $err) -> ErrorEx {
                ErrorEx::$err(err)
            }
        }
    };
}

impl_error_ex_conv!(OpenCLError);
impl_error_ex_conv!(BufferError);
impl_error_ex_conv!(NumberTypeError);

impl Encoder for ErrorEx {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            ErrorEx::NumberTypeError(err) => format!("{:?}", err).encode(env),
            ErrorEx::BufferError(err) => format!("{:?}", err).encode(env),
            ErrorEx::OpenCLError(OpenCLError::StatusCodeError(err)) => {
                let err_ex = StatusCodeErrorEx::new(*err);
                err_ex.encode(env)
            },
            ErrorEx::OpenCLError(err) => format!("{:?}", err).encode(env),
        }
    }
}

#[derive(NifUnitEnum, Debug)]
pub enum OnlyTrue {
    True
}

#[derive(NifStruct)]
#[must_use]
#[module = "OpenCL.StatusCodeError"]
pub struct StatusCodeErrorEx {
    status_code: i32,
    description: String,
    __exception__: OnlyTrue
}

impl StatusCodeErrorEx {
    fn new(e: StatusCodeError) -> StatusCodeErrorEx {
        StatusCodeErrorEx{
            status_code: e.status_code,
            description: e.description().to_owned(),
            __exception__: OnlyTrue::True
        }
    }
}