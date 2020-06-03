use rustler::resource::{ResourceArc, ResourceTypeProvider};

pub struct WrapperEx<T> {
    pub item: T,
}

impl<T> WrapperEx<T> {
    pub fn new(item: T) -> WrapperEx<T> {
        WrapperEx { item }
    }
}

pub trait WrapperExResource
where
    WrapperEx<Self>: ResourceTypeProvider,
    Self: Sized,
{
    fn into_resource_arc(self) -> ResourceArc<WrapperEx<Self>> {
        ResourceArc::new(WrapperEx::new(self))
    }
}
