pub trait NativeWrapper<T> {
    fn native(&self) -> &T;
}
pub trait BitflagEx<F>
where
    Self: Sized,
{
    fn list_from_bitflag(flag: F) -> Vec<Self>;
}
