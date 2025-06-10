use serde::{Deserialize, Serialize};
use std::{fmt::Display, num::ParseFloatError};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Centimiters(pub f64);

impl TryFrom<&str> for Centimiters {
    type Error = ParseFloatError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.trim();
        if let Some(v) = value.strip_suffix("cm") {
            value = v;
        } else if let Some(v) = value.strip_suffix("centimiters") {
            value = v;
        }
        Ok(Self(value.trim().parse()?))
    }
}

impl TryFrom<String> for Centimiters {
    type Error = ParseFloatError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for Centimiters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}cm", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Meters(pub f64);

impl TryFrom<&str> for Meters {
    type Error = ParseFloatError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.trim();
        if let Some(v) = value.strip_suffix("m") {
            value = v;
        } else if let Some(v) = value.strip_suffix("meters") {
            value = v;
        }
        Ok(Self(value.trim().parse()?))
    }
}

impl TryFrom<String> for Meters {
    type Error = ParseFloatError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}

impl From<Centimiters> for Meters {
    fn from(value: Centimiters) -> Self {
        Self(value.0 * 0.01)
    }
}

impl From<Meters> for Centimiters {
    fn from(value: Meters) -> Self {
        Self(value.0 * 100.0)
    }
}
