use std::{
    borrow::Borrow,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
};

use serde::{Deserialize, Serialize};

macro_rules! new_type {
    ($name:ident) => {
        new_type!(@impl $name []);
    };

    ($name:ident [$($into:ident),* $(,)?]) => {
        new_type!(@impl $name [$($into),*] );
    };

    (@impl $name:ident [$($into:ident),* $(,)?] $(,)?) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                self.0.fmt(f)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_string())
            }
        }

        impl Borrow<str> for $name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl PartialEq<&str> for $name {
            fn eq(&self, other: &&str) -> bool {
                self.0 == *other
            }
        }

        $(
            impl From<$into> for $name {
                fn from(id: $into) -> Self {
                    Self(id.0)
                }
            }
        )*

    };
}

new_type!(ChannelId);
