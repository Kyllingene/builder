pub use paste::paste;

#[macro_export]
macro_rules! builder {
    ( @field_type $typ:path, $default:expr ) => {
        $typ
    };

    ( @field_type $typ:path, ) => {
        ::core::option::Option<$typ>
    };

    ( @field_take $field:expr, $default:expr ) => {
        ::core::mem::replace($field, ::core::convert::Into::into($default))
    };

    ( @field_take $field:expr, ) => {
        ($field).take()?
    };

    ( @field_assign $field:expr, $val:expr, $default:expr ) => {
        $field = ::core::convert::Into::into($val);
    };

    ( @field_assign $field:expr, $val:expr, ) => {
        $field = Some(::core::convert::Into::into($val));
    };

    ( @field_default $default:expr ) => {
        ::core::convert::Into::into($default)
    };

    ( @field_default ) => {
        ::core::option::Option::None
    };

    ( @field_val $typ:path, into ) => {
        impl ::core::convert::Into<$typ>
    };

    ( @field_val $typ:path, ) => {
        $typ
    };

    ($(
        $(#[$outer:meta])*
        $v:vis struct $name:ident {
            $(
                $fv:vis
                $field:ident 
                $( into $(@@@$into:tt)? )? : $typ:path
                $( = $default:expr )?
            ),* $(,)?
        }
    )+) => {$($crate::paste!{
        $(#[$outer])*
        $v struct $name {
            $( $fv $field: $typ, )*
        }

        $(#[$outer])*
        $v struct [< $name Builder >] {
            $( $fv $field: $crate::builder!(@field_type $typ, $($default)?), )*
        }

        impl $name {
            pub fn new() -> [< $name Builder >] {
                [< $name Builder >] {$(
                    $field: $crate::builder!(@field_default $($default)?),
                )*}
            }
        }

        impl [< $name Builder >] {
            pub fn build(&mut self) -> ::core::option::Option<$name> {
                ::core::option::Option::Some($name {$(
                    $field: $crate::builder!(@field_take &mut self.$field, $($default)?),
                )*})
            }

            $(
                pub fn $field(
                    &mut self, $field: $crate::builder!(@field_val $typ, $(into $($into)?)?)
                ) -> &mut Self {
                    $crate::builder!(@field_assign self.$field, $field, $($default)?);
                    self
                }
            )*
        }
    })+};
}

builder! {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Foo {
        x: u32,
        pub y into: String,
        z into: String = "Hello, World!",
    }
}

fn main() {
    dbg!(Foo::new().x(2));
    dbg!(Foo::new().x(2).y("Hello, world!").z("foobar").build());
    dbg!(Foo::new().build());
}
