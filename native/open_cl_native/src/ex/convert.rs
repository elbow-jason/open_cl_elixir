
use rustler::{Decoder, ListIterator, Error};
use crate::{NumberEx, RuntimeNumberList};

pub fn list_iterator_to_vec<'a, T: NumberEx + Decoder<'a>>(iter: ListIterator<'a>) -> Result<Vec<T>, Error> {
    iter.map(|x| x.decode::<T>()).collect()
}

pub fn list_iterator_to_rt_list<'a, T: NumberEx + Decoder<'a>>(iter: ListIterator<'a>) -> Result<RuntimeNumberList, Error> {
    let data: Vec<T> = list_iterator_to_vec(iter)?;
    Ok(RuntimeNumberList::from_vec(data))
}