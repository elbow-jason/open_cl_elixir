use rustler::Error as RustlerError;
use rustler::{Env, Term, Encoder};
use opencl_core::Error as OpenCLError;


use crate::atoms;

pub type OutputEx<T> = Result<T, ErrorEx>;

/// An error for the OpenCL to Elixir interface.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum ErrorEx {
    #[fail(display = "{:?}", _0)]
    OpenCLError(OpenCLError),
}

impl From<OpenCLError> for ErrorEx {
    fn from(err: OpenCLError) -> ErrorEx {
        ErrorEx::OpenCLError(err)
    }
}

impl From<ErrorEx> for RustlerError {
    fn from(e: ErrorEx) -> RustlerError {
        RustlerError::RaiseTerm(Box::new(e))
    }
}

impl Encoder for ErrorEx {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            ErrorEx::OpenCLError(OpenCLError::StatusCode(code, err)) => {
                let message = format!("{:?}", err);
                (atoms::status_code_error(), code, message).encode(env)
            },
            ErrorEx::OpenCLError(err) => {
                let message = format!("OpenCL Error {:?}", err);
                (atoms::error(), message).encode(env)
            },
        }
    }
}




