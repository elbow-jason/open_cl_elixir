use crate::nif;
use open_cl_core::StatusCodeError;

#[derive(nif::NifUnitEnum, Debug)]
pub enum OnlyTrue {
    True,
}

#[derive(nif::NifStruct)]
#[must_use]
#[module = "OpenCL.StatusCodeError"]
pub struct StatusCodeErrorEx {
    status_code: i32,
    description: String,
    __exception__: OnlyTrue,
}

impl StatusCodeErrorEx {
    pub fn new(e: StatusCodeError) -> StatusCodeErrorEx {
        StatusCodeErrorEx {
            status_code: e.status_code,
            description: e.description().to_owned(),
            __exception__: OnlyTrue::True,
        }
    }
}
