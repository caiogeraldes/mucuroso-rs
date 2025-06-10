use serde::{Deserialize, Serialize};
use std::{fmt::Display, num::ParseFloatError};

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Seconds(pub f64);

impl TryFrom<&str> for Seconds {
    type Error = ParseFloatError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.trim();
        if let Some(v) = value.strip_suffix("s") {
            value = v;
        } else if let Some(v) = value.strip_suffix("seconds") {
            value = v;
        }
        Ok(Self(value.trim().parse()?))
    }
}

impl TryFrom<String> for Seconds {
    type Error = ParseFloatError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for Seconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.0)
    }
}
