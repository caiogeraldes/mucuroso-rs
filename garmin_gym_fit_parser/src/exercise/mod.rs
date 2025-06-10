use fitparser::ValueWithUnits;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display};

use crate::utils::parsers;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct ExerciseTitle {
    pub id: u16,
    pub title: String,
    pub category: String,
}

impl From<BTreeMap<String, ValueWithUnits>> for ExerciseTitle {
    fn from(value: BTreeMap<String, ValueWithUnits>) -> Self {
        let mut exercise_type = ExerciseTitle::default();
        for field in value {
            match field.0.as_str() {
                "exercise_name" => {
                    exercise_type.id = parsers::unitless_u16_parser(field.1.to_string())
                }
                "wkt_step_name" => exercise_type.title = field.1.to_string(),
                "exercise_category" => exercise_type.category = field.1.to_string(),
                _ => (),
            }
        }
        exercise_type
    }
}

impl Display for ExerciseTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.id)
    }
}
