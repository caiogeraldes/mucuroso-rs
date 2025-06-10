use garmin_gym_fit_parser::exercise::ExerciseTitle;
use garmin_gym_fit_parser::sets::Set as GarminSet;
use serde::{Deserialize, Serialize};

use crate::exercise::{barbell::Barbell, dumbbell::Dumbbell, machine::Machine};
pub(crate) type Kettelbel = dumbbell::Dumbbell;

pub mod barbell;
pub mod dumbbell;
pub mod machine;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub(crate) struct Exercise {
    name: String,
    loading_method: LoadingMethod,
    // target_muscles: Vec<Target>,
    // secondary_target_muscles: Vec<Target>,
}

impl Exercise {
    pub fn try_from(
        set: &GarminSet,
        exercise_titles: &Vec<ExerciseTitle>,
    ) -> Result<Self, anyhow::Error> {
        let mut title = String::new();
        for exercise_title in exercise_titles {
            if exercise_title.id == set.category_subtype_1 {
                title = exercise_title.title.clone();
            }
        }
        if title.is_empty() {
            return Err(anyhow::anyhow!("Unknown Exercise"));
        }

        let loading_method = LoadingMethod::try_from(title.as_str(), &set)?;

        Ok(Self {
            name: title.clone(),
            loading_method,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub(crate) enum LoadingMethod {
    #[default]
    BodyWeight,
    LoadedBodyWeight(u16),
    Machine(Machine),
    Dumbbell(Dumbbell),
    Kettlebell(Kettelbel),
    Barbell(Barbell),
    None,
    Unknown,
}

impl LoadingMethod {
    fn try_from(title: &str, set: &GarminSet) -> Result<Self, anyhow::Error> {
        let mut loading_method = LoadingMethod::default();
        if title.to_lowercase().contains("dumbbell") {
            loading_method = Self::Dumbbell(Dumbbell {
                weight: set.weight().unwrap().0,
                note: Default::default(),
            })
        } else if title.to_lowercase().contains("kettlebell") {
            loading_method = Self::Kettlebell(Kettelbel {
                weight: set.weight().unwrap().0,
                note: Default::default(),
            })
        } else if title.to_lowercase().contains("barbell") {
            let weight = set.weight().unwrap().0;
            let assumed_bar_weight = 6.0;
            loading_method = Self::Barbell(Barbell {
                bar_weight: assumed_bar_weight,
                added_weight: weight - assumed_bar_weight,
                note: None,
            })
        }
        Ok(loading_method)
    }
}

#[cfg(test)]
mod test {
    use crate::exercise::{Exercise, LoadingMethod};
    use garmin_gym_fit_parser::session::SessionData;
    const SAMPLE_SESSION: &str = r#"
            {
              "timestamp": "2025-05-28T06:14:09-03:00",
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
                  "start_time": "2025-05-28T06:14:09-03:00",
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
                  "start_time": "2025-05-28T06:14:30-03:00",
                  "category_1": 2,
                  "category_2": 2,
                  "category_3": 2,
                  "category_subtype_1": 2,
                  "category_subtype_2": 2,
                  "category_subtype_3": 2,
                  "weight_display_unit": "kilogram",
                  "weight": 10.0,
                  "repetitions": 0,
                  "wkt_step_index": "0"
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
    fn named_and_unnamed_exercises() {
        let session_data: SessionData = serde_json::from_str(SAMPLE_SESSION).unwrap();
        let sets = session_data.sets();
        let set = sets.get(0).unwrap();
        let exercise = Exercise::try_from(set, &session_data.exercise_titles()).unwrap();
        assert_eq!(
            exercise,
            Exercise {
                name: "Dumbbell Shoulder Press".into(),
                loading_method: LoadingMethod::Dumbbell(crate::exercise::dumbbell::Dumbbell {
                    weight: 10.0,
                    note: None
                })
            }
        );
    }
}
