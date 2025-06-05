use serde::{Deserialize, Serialize};
use std::{fmt::Display, num::ParseFloatError};

pub trait Weight {
    fn to_kilograms(self) -> Kilograms;
    fn to_pounds(self) -> Pounds;
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub(crate) struct Kilograms(f64);
impl Weight for Kilograms {
    fn to_kilograms(self) -> Kilograms {
        self
    }

    fn to_pounds(self) -> Pounds {
        Pounds(self.0 * (1.0 / 0.45359237))
    }
}

impl TryFrom<&str> for Kilograms {
    type Error = ParseFloatError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.trim();
        if let Some(v) = value.strip_suffix("kg") {
            value = v;
        } else if let Some(v) = value.strip_suffix("kilogram") {
            value = v;
        }
        Ok(Self(value.trim().parse()?))
    }
}

impl TryFrom<String> for Kilograms {
    type Error = ParseFloatError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for Kilograms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}kg", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Pounds(f64);
impl Weight for Pounds {
    fn to_kilograms(self) -> Kilograms {
        Kilograms(self.0 * 0.45359237)
    }

    fn to_pounds(self) -> Pounds {
        self
    }
}

impl TryFrom<&str> for Pounds {
    type Error = ParseFloatError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.trim();
        if let Some(v) = value.strip_suffix("lb") {
            value = v;
        } else if let Some(v) = value.strip_suffix("lbs") {
            value = v;
        }
        Ok(Self(value.trim().parse()?))
    }
}

impl TryFrom<String> for Pounds {
    type Error = ParseFloatError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for Pounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}kg", self.0)
    }
}
