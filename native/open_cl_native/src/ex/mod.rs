use open_cl_core::{Device, Platform, Session};

pub mod status_code_error_ex;
pub use status_code_error_ex::StatusCodeErrorEx;

pub mod wrapper_ex;
pub use wrapper_ex::{WrapperEx, WrapperExResource};

pub mod platform_ex;
pub use platform_ex::PlatformEx;

pub mod device_ex;
pub use device_ex::DeviceEx;

pub mod number_ex;
pub use number_ex::*;

pub mod array_ex;
pub use array_ex::{Array, ArrayEx};

pub mod num_list;
pub use num_list::{NumList, VecOps, VecProps};

pub mod num_type_ex;
pub use num_type_ex::*;

pub mod dims_ex;
pub use dims_ex::{Dimension, DimsEx};

pub mod mem_config_ex;
pub use mem_config_ex::*;

pub mod buffer_ex;
pub use buffer_ex::*;

pub mod kernel_ex;
pub use kernel_ex::*;

pub mod command_queue_props_ex;
pub use command_queue_props_ex::*;

pub mod session_ex;
pub use session_ex::SessionEx;

pub fn define_resources(env: rustler::Env) -> bool {
    rustler::resource!(WrapperEx<Platform>, env);
    rustler::resource!(WrapperEx<Device>, env);
    rustler::resource!(Array, env);
    rustler::resource!(BufferWrapper, env);
    rustler::resource!(WrapperEx<Session>, env);
    true
}
