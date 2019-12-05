use rustler::resource::{ResourceArc, ResourceTypeProvider};
// use rustler::{Decoder, Encoder, Env, Error, Term};


pub struct WrapperEx<T> {
    pub item: T,
}

impl<T> WrapperEx<T> {
    pub fn new(item: T) -> WrapperEx<T> {
        WrapperEx { item }
    }
}

pub trait WrapperExResource where WrapperEx<Self>: ResourceTypeProvider, Self: Sized {
    fn into_resource_arc(self) -> ResourceArc<WrapperEx<Self>> {
        ResourceArc::new(WrapperEx::new(self))
    }
}

// #[export_macro]
// macro_rules! impl_resource_wrapper {
//     ($t:ty, $env:expr) => {
//         #[derive(Clone, Debug)]
  

//         unsafe impl<T> Send for Wrapper<T> where T: Send {}
//         unsafe impl<T> Sync for Wrapper<T> where T: Sync {}

//         pub trait Resource: ResourceTypeProvider {
//             fn into_resource_arc(self) -> ResourceArc<Self>;
//         }

//         impl<T> Resource for Wrapper<T> where T: Sync + Send {
//             fn into_resource_arc(self) -> ResourceArc<Wrapper<T>> {
//                 ResourceArc::new(self)
//             }
//         }

//         // impl<'a, T: 'a> Decoder<'a> for Wrapper<T> {
//         //     fn decode(term: Term<'a>) -> Result<Wrapper<T>, Error> {
//         //         let wrapper: ResourceArc<Wrapper<T>> = term.decode()?;
//         //         Ok(wrapper)
//         //     }
//         // }


//         impl<T> Encoder for Wrapper<T> where Wrapper<T>: ResourceTypeProvider{
//             fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
//                 self.into_resource_arc().encode(env)
//             }
//         }
//     }
// }
