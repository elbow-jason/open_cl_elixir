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
