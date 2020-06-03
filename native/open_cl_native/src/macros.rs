#[macro_export]
macro_rules! impl_native_method_and_nif {
    ($ex_wrapper:ident, $namespace:ident, $func_name:ident, $ret:ty) => {
        impl $ex_wrapper {
            pub fn $func_name(&self) -> $crate::nif::Result<$ret> {
                self.native().$func_name().map_err($crate::nif::ErrorT::error)
            }
        }

        paste::item! {
            #[rustler::nif]
            pub fn [<$namespace _self_ $func_name>](item: $ex_wrapper) -> $crate::nif::Result<$ret> {
                item.$func_name()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_low_level_method_and_nif {
    ($ex_wrapper:ident, $namespace:ident, $func_name:ident, $ret:ty) => {
        impl $ex_wrapper {
            pub fn $func_name(&self) -> $crate::nif::Result<$ret> {
                self.low_level()
                    .$func_name()
                    .map_err($crate::nif::ErrorT::error)
            }
        }

        paste::item! {
            #[rustler::nif]
            pub fn [<$namespace _self_ $func_name>](item: $ex_wrapper) -> $crate::nif::Result<$ret> {
                item.$func_name()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_native_method_into_bitflag_and_nif {
    ($ex_wrapper:ident, $namespace:ident, $func_name:ident, $ret:ident) => {
        impl $ex_wrapper {
            pub fn $func_name(&self) -> $crate::nif::Result<Vec<$ret>> {
                use crate::traits::BitflagEx;
                self.native()
                    .$func_name()
                    .map_err($crate::nif::ErrorT::error)
                    .map(|cl_flag| $ret::list_from_bitflag(cl_flag))
            }
        }

        paste::item! {
            #[rustler::nif]
            pub fn [<$namespace _self_ $func_name>](item: $ex_wrapper) -> $crate::nif::Result<Vec<$ret>> {
                item.$func_name()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_native_method_into_other_and_nif {
    ($ex_wrapper:ident, $namespace:ident, $func_name:ident, Vec<$other_ex_wrapper:ident>) => {
        impl $ex_wrapper {
            pub fn $func_name(&self) -> OutputEx<Vec<$other_ex_wrapper>> {
                self.native()
                    .$func_name()
                    .map_err(|e| e.into())
                    .map(|cl_wrappers| {
                        cl_wrappers
                            .into_iter()
                            .map(|other| {
                                $other_ex_wrapper::new(other)
                            })
                            .collect()
                    })

            }
        }
        paste::item! {
            #[rustler::nif]
            pub fn [<$namespace _self_ $func_name>](item: $ex_wrapper) -> OutputEx<Vec<$other_ex_wrapper>> {
                item.$func_name()
            }
        }
    };
    ($ex_wrapper:ident, $namespace:ident, $func_name:ident, $other_ex_wrapper:ident) => {
        impl $ex_wrapper {
            pub fn $func_name(&self) -> OutputEx<$other_ex_wrapper> {
                self.native()
                    .$func_name()
                    .map_err(|e| e.into())
                    .map(|other_cl_type| $other_ex_wrapper::new(other_cl_type))

            }
        }

        paste::item! {
            #[rustler::nif]
            pub fn [<$namespace _self_ $func_name>](item: $ex_wrapper) -> OutputEx<$other_ex_wrapper> {
                item.$func_name()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_bitflag_ex_for {
    ($ex_type:ident, $flag_type:ident, $mapping:expr) => {
        impl $crate::traits::BitflagEx<$flag_type> for $ex_type {
            fn list_from_bitflag(flag: $flag_type) -> Vec<$ex_type> {
                let mut output: Vec<$ex_type> = Vec::new();
                for (mapper_flag, ex_value) in $mapping.iter() {
                    if flag.contains(*mapper_flag) {
                        output.push(ex_value.clone())
                    }
                }
                output
            }
        }
    };
}

#[macro_export]
macro_rules! type_check {
    ($t1:ty, $t2:ty) => {
        $t1::number_type_of().type_check($t2::number_type_of())
    };

    ($t1:ty, $t2:ident) => {
        $t1::number_type_of().type_check($t2.number_type())
    };

    ($t1:ident, $t2:ty) => {
        $t1.number_type().type_check($t2::number_type_of())
    };

    ($t1:ident, $t2:ident) => {
        $t1::number_type().type_check($t2.number_type())
    };
}

#[macro_export]
macro_rules! apply_num_type_ex1 {
    ($num_type_ex:expr, $func:ident, [ $( $arg:expr ),*]) => {
        match $num_type_ex {
            NumTypeEx::Char => apply_generic!(i8, $func, $( $arg ),*),
            NumTypeEx::Uchar => apply_generic!(u8, $func, $( $arg ),*),
            NumTypeEx::Short => apply_generic!(i16, $func, $( $arg ),*),
            NumTypeEx::Ushort => apply_generic!(u16, $func, $( $arg ),*),
            NumTypeEx::Int => apply_generic!(i32, $func, $( $arg ),*),
            NumTypeEx::Uint => apply_generic!(u32, $func, $( $arg ),*),
            NumTypeEx::Long => apply_generic!(i64, $func, $( $arg ),*),
            NumTypeEx::Ulong => apply_generic!(u64, $func, $( $arg ),*),
            NumTypeEx::Float => apply_generic!(f32, $func, $( $arg ),*),
            NumTypeEx::Double => apply_generic!(f64, $func, $( $arg ),*),
            NumTypeEx::SizeT => apply_generic!(usize, $func, $( $arg ),*),
        }
    };
}

#[macro_export]
macro_rules! apply_2_generics {
    ($t1:ty, $t2:ty, $func:ident, [ $( $arg:expr ),* ]) => {
        $func::<$t1, $t2>($( $arg ),*)
    };
}

#[macro_export]
macro_rules! apply_generic {
    ($t1:ty, $func:ident, $( $arg:expr ),*) => {
        $func::<$t1>($( $arg ),*)
    };
}

#[macro_export]
macro_rules! apply_type_id {

    {
        type_id: $type_id:expr,
        right_t: $t:ty,
        func: $func:ident,
        args: [ $( $arg:expr ),*],
        default: $default:expr
    } => {
        match $type_id {
            type_id::U8 => $func::<u8, $t>($($arg),*),
            type_id::I8 => $func::<i8, $t>($($arg),*),
            type_id::U16 => $func::<u16, $t>($($arg),*),
            type_id::I16 => $func::<i16, $t>($($arg),*),
            type_id::U32 => $func::<u32, $t>($($arg),*),
            type_id::I32 => $func::<i32, $t>($($arg),*),
            type_id::F32 => $func::<f32, $t>($($arg),*),
            type_id::U64 => $func::<u64, $t>($($arg),*),
            type_id::I64 => $func::<i64, $t>($($arg),*),
            type_id::F64 => $func::<f64, $t>($($arg),*),
            type_id::USIZE => $func::<usize, $t>($($arg),*),
            _ => {
                $default
            },
        }
    };


    {
        type_id: $type_id:expr,
        func: $func:ident,
        args: [ $( $arg:expr ),*],
        default: $default:expr
    } => {
        match $type_id {
        $crate::type_id::U8 => $func::<u8>($($arg),*),
        $crate::type_id::I8 => $func::<i8>($($arg),*),
        $crate::type_id::U16 => $func::<u16>($($arg),*),
        $crate::type_id::I16 => $func::<i16>($($arg),*),
        $crate::type_id::U32 => $func::<u32>($($arg),*),
        $crate::type_id::I32 => $func::<i32>($($arg),*),
        $crate::type_id::F32 => $func::<f32>($($arg),*),
        $crate::type_id::U64 => $func::<u64>($($arg),*),
        $crate::type_id::I64 => $func::<i64>($($arg),*),
        $crate::type_id::F64 => $func::<f64>($($arg),*),
        $crate::type_id::USIZE => $func::<usize>($($arg),*),
            _ => {
                $default
            },
        }
    };
}
