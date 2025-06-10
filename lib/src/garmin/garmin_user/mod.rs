use std::collections::BTreeMap;

use crate::utils::units::length::Meters;
use crate::utils::units::mass::{Kilograms, Pounds, Weight};
use fitparser::ValueWithUnits;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct User {
    name: String,
    pub height: Meters,
    pub weight: Kilograms,
}

impl TryFrom<BTreeMap<String, ValueWithUnits>> for User {
    type Error = anyhow::Error;
    fn try_from(value: BTreeMap<String, ValueWithUnits>) -> Result<Self, anyhow::Error> {
        let mut user = User::default();
        let mut metric_height: bool = true;
        let mut metric_weight: bool = true;

        for field in &value {
            match field.0.as_str() {
                "weight_setting" => match field.1.to_string().as_str() {
                    "metric" => {
                        metric_weight = true;
                    }
                    _ => {
                        metric_weight = false;
                    }
                },
                "height_setting" => match field.1.to_string().as_str() {
                    "metric" => {
                        metric_height = true;
                    }
                    _ => {
                        metric_height = false;
                    }
                },
                _ => (),
            }
        }
        for field in value {
            match field.0.as_str() {
                "friendly_name" => user.name = field.1.to_string(),
                "weight" => match metric_weight {
                    true => user.weight = Kilograms::try_from(field.1.to_string())?,
                    false => user.weight = Pounds::try_from(field.1.to_string())?.to_kilograms(),
                },
                "height" => match metric_height {
                    true => user.height = Meters::try_from(field.1.to_string())?,
                    false => todo!(),
                },
                _ => (),
            }
        }
        Ok(user)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImperialUser {
    name: String,
    height: Meters,
    weight: Pounds,
}

impl From<User> for ImperialUser {
    fn from(value: User) -> Self {
        Self {
            name: value.name,
            height: value.height,
            weight: value.weight.to_pounds(),
        }
    }
}
