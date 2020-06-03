use crate::nif;
use open_cl_core::BufferBuilder;
use open_cl_core::{HostAccess, KernelAccess, MemAllocation, MemConfig, MemConfigBuilder};

#[derive(nif::NifUnitEnum, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum KernelAccessEx {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl From<KernelAccessEx> for KernelAccess {
    fn from(kernel_access_ex: KernelAccessEx) -> KernelAccess {
        match kernel_access_ex {
            KernelAccessEx::ReadOnly => KernelAccess::ReadOnly,
            KernelAccessEx::WriteOnly => KernelAccess::WriteOnly,
            KernelAccessEx::ReadWrite => KernelAccess::ReadWrite,
        }
    }
}

impl From<KernelAccess> for KernelAccessEx {
    fn from(kernel_access: KernelAccess) -> KernelAccessEx {
        match kernel_access {
            KernelAccess::ReadOnly => KernelAccessEx::ReadOnly,
            KernelAccess::WriteOnly => KernelAccessEx::WriteOnly,
            KernelAccess::ReadWrite => KernelAccessEx::ReadWrite,
        }
    }
}

#[derive(nif::NifUnitEnum, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum HostAccessEx {
    ReadOnly,
    WriteOnly,
    NoAccess,
    ReadWrite,
}

impl From<HostAccessEx> for HostAccess {
    fn from(host_access_ex: HostAccessEx) -> HostAccess {
        match host_access_ex {
            HostAccessEx::ReadOnly => HostAccess::ReadOnly,
            HostAccessEx::WriteOnly => HostAccess::WriteOnly,
            HostAccessEx::NoAccess => HostAccess::NoAccess,
            HostAccessEx::ReadWrite => HostAccess::ReadWrite,
        }
    }
}

impl From<HostAccess> for HostAccessEx {
    fn from(host_access: HostAccess) -> HostAccessEx {
        match host_access {
            HostAccess::ReadOnly => HostAccessEx::ReadOnly,
            HostAccess::WriteOnly => HostAccessEx::WriteOnly,
            HostAccess::NoAccess => HostAccessEx::NoAccess,
            HostAccess::ReadWrite => HostAccessEx::ReadWrite,
        }
    }
}

#[derive(nif::NifUnitEnum, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MemAllocationEx {
    KeepInPlace,
    AllocOnDevice,
    CopyToDevice,
    ForceCopyToDevice,
}

impl From<MemAllocationEx> for MemAllocation {
    fn from(mem_allocation: MemAllocationEx) -> MemAllocation {
        match mem_allocation {
            MemAllocationEx::KeepInPlace => MemAllocation::KeepInPlace,
            MemAllocationEx::AllocOnDevice => MemAllocation::AllocOnDevice,
            MemAllocationEx::CopyToDevice => MemAllocation::CopyToDevice,
            MemAllocationEx::ForceCopyToDevice => MemAllocation::ForceCopyToDevice,
        }
    }
}

impl From<MemAllocation> for MemAllocationEx {
    fn from(mem_allocation: MemAllocation) -> MemAllocationEx {
        match mem_allocation {
            MemAllocation::KeepInPlace => MemAllocationEx::KeepInPlace,
            MemAllocation::AllocOnDevice => MemAllocationEx::AllocOnDevice,
            MemAllocation::CopyToDevice => MemAllocationEx::CopyToDevice,
            MemAllocation::ForceCopyToDevice => MemAllocationEx::ForceCopyToDevice,
        }
    }
}

#[derive(nif::NifStruct, Debug, PartialEq, Eq, Hash, Clone)]
#[must_use]
#[module = "OpenCL.MemConfig"]
pub struct MemConfigBuilderEx {
    kernel_access: Option<KernelAccessEx>,
    host_access: Option<HostAccessEx>,
    mem_allocation: Option<MemAllocationEx>,
}

impl MemConfigBuilderEx {
    pub fn with_mem_allocation<B: BufferBuilder>(mut self, b: &B) -> MemConfigBuilderEx {
        let default = b.mem_config();
        let mem_allocation = self
            .mem_allocation
            .unwrap_or_else(|| MemAllocationEx::from(default.mem_allocation()));
        self.mem_allocation = Some(mem_allocation);
        self
    }

    pub fn from_mem_config(mem_config: MemConfig) -> MemConfigBuilderEx {
        MemConfigBuilderEx {
            kernel_access: Some(KernelAccessEx::from(mem_config.kernel_access())),
            host_access: Some(HostAccessEx::from(mem_config.host_access())),
            mem_allocation: Some(MemAllocationEx::from(mem_config.mem_allocation())),
        }
    }
}

impl Default for MemConfigBuilderEx {
    fn default() -> MemConfigBuilderEx {
        MemConfigBuilderEx {
            kernel_access: None,
            host_access: None,
            mem_allocation: None,
        }
    }
}

impl MemConfigBuilderEx {
    pub fn build(self) -> MemConfig {
        let mut builder: MemConfigBuilder = MemConfig::default().as_builder();
        if let Some(ha) = self.host_access {
            builder.host_access = ha.into();
        };
        if let Some(ka) = self.kernel_access {
            builder.kernel_access = ka.into();
        }
        if let Some(loc) = self.mem_allocation {
            builder.mem_allocation = loc.into();
        };
        builder.build()
    }
}

#[derive(nif::NifUnitEnum, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum NilOnly {
    Nil,
}

#[derive(nif::NifUntaggedEnum, Debug, PartialEq, Eq, Hash, Clone)]
pub enum MemConfigEx {
    Nil(NilOnly),
    Builder(MemConfigBuilderEx),
}

impl MemConfigEx {
    pub fn into_builder(self) -> MemConfigBuilderEx {
        match self {
            MemConfigEx::Nil(..) => MemConfigBuilderEx::default(),
            MemConfigEx::Builder(b) => b,
        }
    }

    pub fn from_mem_config(mem_config: MemConfig) -> MemConfigEx {
        MemConfigEx::Builder(MemConfigBuilderEx::from_mem_config(mem_config))
    }
}
