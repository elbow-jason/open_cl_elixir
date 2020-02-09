use opencl_core::Error as OpenCLError;
use rustler::Error as RustlerError;
use rustler::{Encoder, Env, Term};

use crate::atoms;

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
    }
}

impl_error_ex_conv!(OpenCLError);
impl_error_ex_conv!(BufferError);
impl_error_ex_conv!(NumberTypeError);

impl Encoder for ErrorEx {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            ErrorEx::NumberTypeError(err) => {
                let message = format!("{:?}", err);
                (atoms::error(), message).encode(env)
            }

            ErrorEx::BufferError(err) => {
                let message = format!("{:?}", err);
                (atoms::error(), message).encode(env)
            }

            ErrorEx::OpenCLError(err) => {
                let message = format!("{:?}", err);
                (atoms::error(), message).encode(env)
            }
        }
    }
}
