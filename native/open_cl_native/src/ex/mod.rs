use opencl_core::{Device, Platform, Session};

pub mod array_ex;
pub mod device_ex;
pub mod error_ex;
pub mod number_ex;
pub mod platform_ex;
pub mod session_ex;
pub mod device_buffer_ex;
pub mod dims_ex;
pub mod kernel_ex;
pub mod wrapper_ex;

pub use array_ex::{Array, ArrayEx};
pub use device_buffer_ex::{DeviceBuffer, DeviceBufferEx};
pub use device_ex::DeviceEx;
pub use dims_ex::{Dimension, DimsEx};
pub use error_ex::{ErrorEx, OutputEx};
pub use kernel_ex::kernel_execute_sync;
pub use number_ex::{
    CastNumber, NumEx, Number, NumberType, NumberTyped, NumberTypedT, NumberVector,
};
pub use platform_ex::PlatformEx;
pub use session_ex::SessionEx;
pub use wrapper_ex::{WrapperEx, WrapperExResource};

pub fn define_resources(env: rustler::Env) -> bool {
    rustler::resource!(WrapperEx<Platform>, env);
    rustler::resource!(WrapperEx<Device>, env);
    rustler::resource!(WrapperEx<Session>, env);
    rustler::resource!(Array, env);
    rustler::resource!(DeviceBuffer, env);
    true
}
