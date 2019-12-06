use opencl_core::{
    Platform,
    Device,
    Session,
};

pub mod error_ex;
pub mod platform_ex;
pub mod device_ex;
pub mod session_ex;
pub mod tensor_ex;
pub mod array_ex;
pub mod number_ex;
pub mod wrapper_ex;

pub use error_ex::{ErrorEx, OutputEx};
pub use platform_ex::PlatformEx;
pub use device_ex::DeviceEx;
pub use tensor_ex::{Tensor};
pub use array_ex::{ArrayEx, Array};
pub use session_ex::SessionEx;

pub use wrapper_ex::{WrapperEx, WrapperExResource};

pub fn define_resources(env: rustler::Env) -> bool {
    rustler::resource!(WrapperEx<Platform>, env);
    rustler::resource!(WrapperEx<Device>, env);
    rustler::resource!(WrapperEx<Session>, env);
    rustler::resource!(Array, env);
    rustler::resource!(Tensor, env);
    true
}
