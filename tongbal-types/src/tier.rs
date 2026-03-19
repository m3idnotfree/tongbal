use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tier {
    One,
    Two,
}

impl Serialize for Tier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Self::One => 1,
            Self::Two => 2,
        };

        serializer.serialize_u8(value)
    }
}

impl<'de> Deserialize<'de> for Tier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Unexpected, Visitor};
        struct TierVisitor;

        impl<'de> Visitor<'de> for TierVisitor {
            type Value = Tier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("1 or 2")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    1 => Ok(Tier::One),
                    2 => Ok(Tier::Two),
                    _ => Err(E::invalid_value(Unexpected::Unsigned(v), &self)),
                }
            }
        }

        deserializer.deserialize_u64(TierVisitor)
    }
}
