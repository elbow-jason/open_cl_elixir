use crate::nif;
use open_cl_core::CommandQueueProperties;

#[derive(nif::NifUnitEnum)]
pub enum CommandQueuePropEx {
    OutOfOrderExecution,
    ProfilingEnabled,
    OnDevice,
    OnDeviceDefault,
}

impl CommandQueuePropEx {
    pub fn into_cl_type(p: CommandQueuePropEx) -> CommandQueueProperties {
        use CommandQueuePropEx as P;
        match p {
            P::OutOfOrderExecution => CommandQueueProperties::OUT_OF_ORDER_EXEC_MODE_ENABLE,
            P::ProfilingEnabled => CommandQueueProperties::PROFILING_ENABLE,
            P::OnDevice => CommandQueueProperties::ON_DEVICE,
            P::OnDeviceDefault => CommandQueueProperties::ON_DEVICE_DEFAULT,
        }
    }

    pub fn vec_into_cl_type(props: Vec<CommandQueuePropEx>) -> CommandQueueProperties {
        let mut cl_p = CommandQueueProperties::default();
        for p in props {
            cl_p |= CommandQueuePropEx::into_cl_type(p);
        }
        cl_p
    }
}
