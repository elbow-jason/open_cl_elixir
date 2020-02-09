use opencl_core::{Device, Platform, Session};

// HELPERS
pub mod array_ex;
pub mod dims_ex;
pub mod error_ex;
pub mod number_ex;
pub mod wrapper_ex;
pub mod number_list_ex;

// OPENCL OBJECT WRAPPERS
pub mod buffer_ex;
pub mod device_ex;
pub mod kernel_ex;
pub mod platform_ex;
pub mod session_ex;

pub use device_ex::DeviceEx;
pub use error_ex::{ErrorEx, OutputEx};
pub use platform_ex::PlatformEx;
pub use wrapper_ex::{WrapperEx, WrapperExResource};

pub use array_ex::{Array, ArrayEx};


pub use buffer_ex::*;
pub use dims_ex::{Dimension, DimsEx};
pub use kernel_ex::*;
pub use number_ex::*;
pub use number_list_ex::*;


pub use session_ex::SessionEx;

pub fn define_resources(env: rustler::Env) -> bool {
    rustler::resource!(WrapperEx<Platform>, env);
    rustler::resource!(WrapperEx<Device>, env);
    rustler::resource!(Array, env);
    rustler::resource!(WrapperEx<Session>, env);
    rustler::resource!(BufferWrapper, env);
    true
}
