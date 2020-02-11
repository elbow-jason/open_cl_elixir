#[macro_export]
macro_rules! impl_native_method_and_nif {
    ($ex_wrapper:ident, $namespace:ident, $func_name:ident, $ret:ty) => {
        impl $ex_wrapper {
            pub fn $func_name(&self) -> OutputEx<$ret> {
                self.native().$func_name().map_err(|e| e.into())
            }
        }

        paste::item! {
            #[rustler::nif]
            pub fn [<$namespace _self_ $func_name>](item: $ex_wrapper) -> OutputEx<$ret> {
                item.$func_name()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_low_level_method_and_nif {
    ($ex_wrapper:ident, $namespace:ident, $func_name:ident, $ret:ty) => {
        impl $ex_wrapper {
            pub fn $func_name(&self) -> OutputEx<$ret> {
                self.low_level().$func_name().map_err(|e| e.into())
            }
        }

        paste::item! {
            #[rustler::nif]
            pub fn [<$namespace _self_ $func_name>](item: $ex_wrapper) -> OutputEx<$ret> {
                item.$func_name()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_native_method_into_bitflag_and_nif {
    ($ex_wrapper:ident, $namespace:ident, $func_name:ident, $ret:ident) => {
        impl $ex_wrapper {
            pub fn $func_name(&self) -> OutputEx<Vec<$ret>> {
                use crate::traits::BitflagEx;
                self.native()
                    .$func_name()
                    .map_err(|e| e.into())
                    .map(|cl_flag| $ret::list_from_bitflag(cl_flag))
            }
        }

        paste::item! {
            #[rustler::nif]
            pub fn [<$namespace _self_ $func_name>](item: $ex_wrapper) -> OutputEx<Vec<$ret>> {
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
    ($ex_type:ident, $flag_type:ident,  $mapping:expr) => {
        impl BitflagEx<$flag_type> for $ex_type {
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
macro_rules! apply_number_type {
    ($num_type:expr, $func:ident, [ $( $arg:expr ),* ]) => {
        match $num_type.number_type() {
            NumberType::U8 => $func::<u8>($( $arg ),*),
            NumberType::I8 => $func::<i8>($( $arg ),*),
            NumberType::U16 => $func::<u16>($( $arg ),*),
            NumberType::I16 => $func::<i16>($( $arg ),*),
            NumberType::U32 => $func::<u32>($( $arg ),*),
            NumberType::I32 => $func::<i32>($( $arg ),*),
            NumberType::F32 => $func::<f32>($( $arg ),*),
            NumberType::U64 => $func::<u64>($( $arg ),*),
            NumberType::I64 => $func::<i64>($( $arg ),*),
            NumberType::F64 => $func::<f64>($( $arg ),*),
            NumberType::Usize => $func::<usize>($( $arg ),*),
            NumberType::Isize => $func::<isize>($( $arg ),*),
        }
    }
}



#[macro_export]
macro_rules! type_check {
    ($t1:ty, $t2:ty) => {
        {
            $t1::number_type_of().type_check($t2::number_type_of())
        }
    };
    ($t1:ty, $t2:ident) => {
        {
            $t1::number_type_of().type_check($t2.number_type())
        }
    };
    ($t1:ident, $t2:ty) => {
        {
            $t1.number_type().type_check($t2::number_type_of())
        }
    };
    ($t1:ident, $t2:ident) => {
        {
            $t1::number_type().type_check($t2.number_type())
        }
    };
}

