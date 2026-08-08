/// This macro accepts a data-less enum definition and generates `From` impls
/// for converting between the enum and its repr type.
///
/// Example usage:
/// ```ignore
/// convertible_enum! {
///     repr = u8,
///     unknown = Unknown,
///
///     #[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
///     pub enum MouseButton {
///         Left = 0,
///         Middle = 1,
///         Right = 2,
///         Unknown = 255,
///     }
/// }
/// ```
macro_rules! convertible_enum {
    (
        repr = $repr:ident,
        unknown = $unknown_variant:ident,
        $(#[$outer:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$inner:meta])*
                $variant:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        $(#[$outer])*
        #[repr($repr)]
        $vis enum $name {
            $(
                $(#[$inner])*
                $variant = $value,
            )*
        }

        impl From<$repr> for $name {
            fn from(value: $repr) -> Self {
                match value {
                    $(
                        $value => Self::$variant,
                    )*
                    _ => Self::$unknown_variant,
                }
            }
        }

        impl From<$name> for $repr {
            fn from(value: $name) -> Self {
                value as $repr
            }
        }
    };
}

pub(crate) use convertible_enum;
