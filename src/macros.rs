#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

#[macro_export]
macro_rules! match_lang {
    (
        Executor : let mut $executor:ident,
        Lang : $struct:ident.$field:ident,
        $($code:tt)*
    ) => {
        match $struct.$field {
            Language::Python3 => {
                let mut $executor = CodeExecutor::<python_3::Python3>::new();
                $($code)*
            },
            Language::Cpp => {
                let mut $executor = CodeExecutor::<cpp::Cpp>::new();
                $($code)*
            },
            _ => todo!(),
        }
    };
}
