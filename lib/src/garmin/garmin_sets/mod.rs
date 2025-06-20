use crate::garmin::exercise_title::ExerciseTitle;
use crate::utils::parsers;
use crate::utils::units::{mass::Kilograms, time::Seconds};
use chrono::{DateTime, FixedOffset};
use fitparser::ValueWithUnits;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Set {
    pub duration: Seconds,
    pub set_type: String,
    pub start_time: DateTime<FixedOffset>,
    pub category_1: u16,
    pub category_2: u16,
    pub category_3: u16,
    pub category_subtype_1: u16,
    pub category_subtype_2: u16,
    pub category_subtype_3: u16,
    pub weight_display_unit: String,
    pub weight: Option<Kilograms>,
    pub repetitions: u16,
    pub wkt_step_index: String,
}

impl Set {
    pub fn weight(&self) -> Option<Kilograms> {
        self.weight.clone()
    }
    pub fn total_weight(&self) -> Option<Kilograms> {
        self.weight().map(|w| w * self.repetitions)
    }
    pub fn get_exercise_type(&self, options: &Vec<ExerciseTitle>) -> Option<ExerciseTitle> {
        for option in options {
            if option.id == self.category_1 {
                return Some(option.clone());
            }
        }
        None
    }
}

impl TryFrom<BTreeMap<String, ValueWithUnits>> for Set {
    type Error = anyhow::Error;
    fn try_from(value: BTreeMap<String, ValueWithUnits>) -> Result<Self, anyhow::Error> {
        let mut set = Set::default();
        for field in value {
            match field.0.as_str() {
                "duration" => set.duration = field.1.to_string().try_into()?,
                "set_type" => set.set_type = field.1.to_string(),
                "start_time" => {
                    set.start_time = DateTime::parse_from_str(
                        field.1.to_string().as_str(),
                        "%Y-%m-%d %H:%M:%S %z",
                    )?
                }
                "category" => {
                    (set.category_1, set.category_2, set.category_3) =
                        parsers::triple_u16_array(field.1.to_string())
                }
                "category_subtype" => {
                    (
                        set.category_subtype_1,
                        set.category_subtype_2,
                        set.category_subtype_3,
                    ) = parsers::triple_u16_array(field.1.to_string())
                }
                "weight_display_unit" => set.weight_display_unit = field.1.to_string(),
                "wkt_step_index" => set.wkt_step_index = field.1.to_string(),
                "weight" => set.weight = Some(field.1.to_string().try_into()?),
                "repetitions" => {
                    set.repetitions = parsers::unitless_u16_parser(field.1.to_string())
                }
                _ => (),
            }
        }
        Ok(set)
    }
}
