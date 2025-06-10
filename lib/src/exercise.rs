use crate::garmin::exercise_title::ExerciseTitle as GarminExerciseTitle;
use crate::garmin::garmin_sets::Set as GarminSet;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

pub mod barbell;
pub mod dumbbell;
pub mod machine;
pub mod target;

use crate::{
    constants::GARMIN_EXERCISES,
    exercise::{
        barbell::Barbell,
        dumbbell::Dumbbell,
        machine::Machine,
        target::{Target, gen_target},
    },
    utils::Weight,
};
pub(crate) type Kettelbel = dumbbell::Dumbbell;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Set {
    name: String,
    loading_method: LoadingMethod,
    target_muscles: Vec<Target>,
    repetitions: u16,
    notes: Vec<String>,
    duration: f64,
    set_type: String,
    start_time: DateTime<FixedOffset>,
    wkt_step_index: String,
}

impl Set {
    pub fn try_from_garmin(
        set: GarminSet,
        exercise_titles: &Vec<GarminExerciseTitle>,
    ) -> Result<Self, anyhow::Error> {
        let mut title = String::new();
        let mut repetitions = set.repetitions;
        let duration = set.duration.0;
        for exercise_title in exercise_titles {
            if exercise_title.id == set.category_subtype_1 && exercise_title.category != "warm_up" {
                title = exercise_title.title.clone();
            }
        }

        let mut loading_method = LoadingMethod::Unknown;
        let mut target_muscles = vec![];
        let mut notes = vec![];

        if title.is_empty() {
            title = "Untitled Exercise".into();
            notes.push("Undefined Exercise".into());
        }

        for ge in GARMIN_EXERCISES.iter() {
            if ge.name.to_lowercase() == title.replace("-", " ").to_lowercase() {
                target_muscles = gen_target(ge);
                if ge.dumbbell {
                    let weight = match set.weight() {
                        Some(w) => w.0,
                        None => 0.0,
                    };
                    loading_method = LoadingMethod::Dumbbell(Dumbbell {
                        weight,
                        note: Default::default(),
                    })
                } else if ge.kettlebell {
                    let weight = match set.weight() {
                        Some(w) => w.0,
                        None => 0.0,
                    };
                    loading_method = LoadingMethod::Kettlebell(Kettelbel {
                        weight,
                        note: Default::default(),
                    })
                } else if ge.barbell {
                    let weight = match set.weight() {
                        Some(w) => w.0,
                        None => 0.0,
                    };
                    let assumed_bar_weight = 6.0;
                    notes.push("Bar weight is assumed!".into());
                    loading_method = LoadingMethod::Barbell(Barbell {
                        bar_weight: assumed_bar_weight,
                        added_weight: weight - assumed_bar_weight,
                        note: None,
                    })
                } else if ge.cable_machine {
                    let weight = match set.weight() {
                        Some(w) => w.0,
                        None => 0.0,
                    };
                    loading_method = LoadingMethod::Machine(Machine {
                        name: Default::default(),
                        weight,
                        note: Default::default(),
                    })
                }
            }
        }

        if loading_method == LoadingMethod::Unknown {
            loading_method = match LoadingMethod::try_from(&title, &set) {
                Ok(l) => l,
                Err(_) => LoadingMethod::Unknown,
            };
        }

        if set.set_type == "rest" {
            repetitions = 0;
            loading_method = LoadingMethod::None;
            target_muscles = vec![];
            title = "Rest".into();
        }

        Ok(Self {
            name: title.clone(),
            repetitions,
            duration,
            loading_method,
            target_muscles,
            notes,
            set_type: set.set_type.clone(),
            start_time: set.start_time,
            wkt_step_index: set.wkt_step_index.clone(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn loading_method(&self) -> &LoadingMethod {
        &self.loading_method
    }

    pub fn target_muscles(&self) -> &[Target] {
        &self.target_muscles
    }

    pub fn repetitions(&self) -> u16 {
        self.repetitions
    }

    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    pub fn duration(&self) -> f64 {
        self.duration
    }

    pub fn set_type(&self) -> &str {
        &self.set_type
    }

    pub fn start_time(&self) -> DateTime<FixedOffset> {
        self.start_time
    }

    pub fn wkt_step_index(&self) -> &str {
        &self.wkt_step_index
    }
}
impl Weight for Set {
    fn weight(&self) -> f64 {
        match &self.loading_method {
            LoadingMethod::BodyWeight => 0.0,
            LoadingMethod::LoadedBodyWeight(e) => *e,
            LoadingMethod::Machine(machine) => machine.weight(),
            LoadingMethod::Dumbbell(dumbbell) => dumbbell.weight(),
            LoadingMethod::Kettlebell(dumbbell) => dumbbell.weight(),
            LoadingMethod::Barbell(barbell) => barbell.weight(),
            LoadingMethod::None => 0.0,
            LoadingMethod::Unknown => 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub enum LoadingMethod {
    BodyWeight,
    LoadedBodyWeight(f64),
    Machine(Machine),
    Dumbbell(Dumbbell),
    Kettlebell(Kettelbel),
    Barbell(Barbell),
    None,
    #[default]
    Unknown,
}

impl LoadingMethod {
    fn try_from(title: &str, set: &GarminSet) -> Result<Self, anyhow::Error> {
        let mut loading_method = LoadingMethod::default();
        if title.to_lowercase().contains("dumbbell") {
            loading_method = Self::Dumbbell(Dumbbell {
                weight: set
                    .weight()
                    .ok_or(anyhow::anyhow!("No weight available"))?
                    .0,
                note: Default::default(),
            })
        } else if title.to_lowercase().contains("kettlebell") {
            loading_method = Self::Kettlebell(Kettelbel {
                weight: set
                    .weight()
                    .ok_or(anyhow::anyhow!("No weight available"))?
                    .0,
                note: Default::default(),
            })
        } else if title.to_lowercase().contains("barbell") {
            let weight = set
                .weight()
                .ok_or(anyhow::anyhow!("No weight available"))?
                .0;
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
    use crate::exercise::{LoadingMethod, Set, target::Target};
    use crate::garmin::garmin_session::GarminSessionData;
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
        let session_data: GarminSessionData = serde_json::from_str(SAMPLE_SESSION).unwrap();
        let sets = session_data.sets();
        let set = sets.get(0).unwrap().clone();
        let exercise = Set::try_from_garmin(set, &session_data.exercise_titles()).unwrap();
        assert_eq!(
            exercise,
            Set {
                name: "Dumbbell Shoulder Press".into(),
                loading_method: LoadingMethod::Dumbbell(crate::exercise::dumbbell::Dumbbell {
                    weight: 10.0,
                    note: None
                }),
                target_muscles: vec![
                    Target::Triceps,
                    Target::Deltoids,
                    Target::Back,
                    Target::UpperBack
                ],
                repetitions: 1,
                notes: vec![],
                duration: 20.0,
                set_type: "active".into(),
                start_time: Default::default(),
                wkt_step_index: "0".into()
            }
        );
    }
}
