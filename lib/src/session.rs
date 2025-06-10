use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::garmin::garmin_session::GarminSessionData;
use crate::{exercise::Set, utils::User};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SessionData {
    pub timestamp: DateTime<FixedOffset>,
    pub sets: Vec<Set>,
    pub user: User,
}

impl TryFrom<GarminSessionData> for SessionData {
    type Error = anyhow::Error;

    fn try_from(value: GarminSessionData) -> Result<Self, Self::Error> {
        let mut exercises = vec![];
        for set in value.sets {
            exercises.push(Set::try_from_garmin(set, &value.exercise_titles)?)
        }
        Ok(SessionData {
            timestamp: value.timestamp,
            sets: exercises,
            user: User {
                height: value.user.height.0,
                weight: value.user.weight.0,
            },
        })
    }
}

impl PartialOrd for SessionData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SessionData {}

impl Ord for SessionData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.timestamp).partial_cmp(&other.timestamp).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::garmin::garmin_session::GarminSessionData;
    use crate::session::SessionData;
    const SAMPLE_SESSION: &str = r#"
            {
              "timestamp": "1970-01-01T00:00:00+00:00",
              "exercise_titles": [
                {
                  "id": 24,
                  "title": "Dumbbell Shoulder Press",
                  "category": "shoulder_press"
                }
              ],
              "sets": [
                {
                  "duration": 20.0,
                  "set_type": "active",
                  "start_time": "1970-01-01T00:00:00+00:00",
                  "category_1": 2,
                  "category_2": 2,
                  "category_3": 2,
                  "category_subtype_1": 24,
                  "category_subtype_2": 24,
                  "category_subtype_3": 24,
                  "weight_display_unit": "kilogram",
                  "weight": 10.0,
                  "repetitions": 1,
                  "wkt_step_index": "0"
                },
                {
                  "duration": 20.0,
                  "set_type": "active",
                  "start_time": "1970-01-01T00:00:40+00:00",
                  "category_1": 2,
                  "category_2": 2,
                  "category_3": 2,
                  "category_subtype_1": 2,
                  "category_subtype_2": 2,
                  "category_subtype_3": 2,
                  "weight_display_unit": "kilogram",
                  "weight": 10.0,
                  "repetitions": 0,
                  "wkt_step_index": "1"
                }
              ],
              "user": {
                "name": "User",
                "height": 1.7,
                "weight": 70
              }
            }
            "#;

    #[test]
    fn from_garmin() {
        let g_session_data: GarminSessionData = serde_json::from_str(SAMPLE_SESSION).unwrap();
        let session_data = SessionData::try_from(g_session_data).unwrap();
        dbg!(session_data);
    }
}
