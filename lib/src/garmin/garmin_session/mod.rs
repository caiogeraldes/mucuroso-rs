use std::io::Read;

use crate::garmin::exercise_title::ExerciseTitle;
use crate::garmin::fitdata::FitDataMap;
use crate::garmin::garmin_sets::Set;
use crate::garmin::garmin_user::User;
use crate::utils::units::mass::Kilograms;
use chrono::{DateTime, FixedOffset};
use fitparser::FitDataRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct GarminSessionData {
    pub timestamp: DateTime<FixedOffset>,
    pub exercise_titles: Vec<ExerciseTitle>,
    pub sets: Vec<Set>,
    pub user: User,
}

impl GarminSessionData {
    fn new() -> Self {
        GarminSessionData::default()
    }
}

impl TryFrom<Vec<FitDataRecord>> for GarminSessionData {
    type Error = anyhow::Error;
    fn try_from(value: Vec<FitDataRecord>) -> Result<Self, anyhow::Error> {
        let mut session_data = GarminSessionData::new();
        for data in value {
            let data_map = FitDataMap::new(data);
            match data_map.kind {
                fitparser::profile::MesgNum::UserProfile => {
                    session_data.user = User::try_from(data_map.fields)?
                }
                fitparser::profile::MesgNum::FileId => {
                    for field in data_map.fields {
                        if field.0.as_str() == "time_created" {
                            session_data.timestamp = DateTime::parse_from_str(
                                field.1.to_string().as_str().trim(),
                                "%Y-%m-%d %H:%M:%S %z",
                            )?
                        }
                    }
                }
                fitparser::profile::MesgNum::Set => {
                    session_data.sets.push(Set::try_from(data_map.fields)?);
                }
                fitparser::profile::MesgNum::ExerciseTitle => {
                    session_data
                        .exercise_titles
                        .push(ExerciseTitle::from(data_map.fields));
                }
                _ => (),
            }
        }
        Ok(session_data)
    }
}

impl GarminSessionData {
    pub fn timestamp(&self) -> DateTime<FixedOffset> {
        self.timestamp
    }

    pub fn sets(&self) -> Vec<Set> {
        self.sets.clone()
    }

    pub fn exercise_titles(&self) -> Vec<ExerciseTitle> {
        self.exercise_titles.clone()
    }
}

impl GarminSessionData {
    pub fn try_from_reader<T: Read>(mut source: T) -> Result<Self, anyhow::Error> {
        GarminSessionData::try_from(fitparser::from_reader(&mut source)?)
    }

    pub fn total_weight(&self) -> Kilograms {
        let mut sum = Kilograms(0.0);
        for set in &self.sets {
            match set.total_weight() {
                Some(w) => {
                    sum = sum + w;
                }
                None => {
                    if let Some(ExerciseTitle { category, .. }) =
                        set.get_exercise_type(&self.exercise_titles)
                    {
                        if category != "cardio" {
                            sum = self.user.weight.clone() * set.repetitions
                        }
                    }
                }
            }
        }
        sum
    }
}

impl PartialOrd for GarminSessionData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for GarminSessionData {}

impl Ord for GarminSessionData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.timestamp).partial_cmp(&other.timestamp).unwrap()
    }
}
