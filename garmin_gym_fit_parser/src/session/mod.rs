use crate::fitdata::FitDataMap;
use crate::sets::Set;
use crate::{exercise::ExerciseTitle, user::User};
use chrono::{DateTime, FixedOffset};
use fitparser::FitDataRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SessionData {
    timestamp: DateTime<FixedOffset>,
    exercise_titles: Vec<ExerciseTitle>,
    sets: Vec<Set>,
    user: User,
}

impl SessionData {
    fn new() -> Self {
        SessionData::default()
    }
}

impl TryFrom<Vec<FitDataRecord>> for SessionData {
    type Error = anyhow::Error;
    fn try_from(value: Vec<FitDataRecord>) -> Result<Self, anyhow::Error> {
        let mut session_data = SessionData::new();
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

impl SessionData {
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
