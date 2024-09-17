#[macro_export]
macro_rules! with_keys {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            fn keys() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
        }
    };
    (pub struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        pub struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            fn keys() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
        }
    };
}
