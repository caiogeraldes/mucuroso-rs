use serde::{Deserialize, Serialize, de};

#[derive(Serialize, Deserialize, Debug)]
pub struct GarminExercise {
    #[serde(rename = "NAME_GARMIN")]
    pub name_garmin: String,
    #[serde(rename = "CATEGORY_GARMIN")]
    pub category_garmin: GarminCategory,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Detailed", deserialize_with = "deserialize_bool")]
    pub detailed: bool,
    #[serde(rename = "Body parts")]
    pub body_parts: String,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "Chest", deserialize_with = "deserialize_bool")]
    pub chest: bool,
    #[serde(rename = "Triceps", deserialize_with = "deserialize_bool")]
    pub triceps: bool,
    #[serde(rename = "Delts", deserialize_with = "deserialize_bool")]
    pub delts: bool,
    #[serde(rename = "Abs", deserialize_with = "deserialize_bool")]
    pub abs: bool,
    #[serde(rename = "Pecs", deserialize_with = "deserialize_bool")]
    pub pecs: bool,
    #[serde(rename = "Shoulders", deserialize_with = "deserialize_bool")]
    pub shoulders: bool,
    #[serde(rename = "Hips", deserialize_with = "deserialize_bool")]
    pub hips: bool,
    #[serde(rename = "Quads", deserialize_with = "deserialize_bool")]
    pub quads: bool,
    #[serde(rename = "Glutes", deserialize_with = "deserialize_bool")]
    pub glutes: bool,
    #[serde(rename = "Back", deserialize_with = "deserialize_bool")]
    pub back: bool,
    #[serde(rename = "Calves", deserialize_with = "deserialize_bool")]
    pub calves: bool,
    #[serde(rename = "Total Body", deserialize_with = "deserialize_bool")]
    pub total_body: bool,
    #[serde(rename = "Legs", deserialize_with = "deserialize_bool")]
    pub legs: bool,
    #[serde(rename = "Obliques", deserialize_with = "deserialize_bool")]
    pub obliques: bool,
    #[serde(rename = "Core", deserialize_with = "deserialize_bool")]
    pub core: bool,
    #[serde(rename = "Upper Back", deserialize_with = "deserialize_bool")]
    pub upper_back: bool,
    #[serde(rename = "Lower Back", deserialize_with = "deserialize_bool")]
    pub lower_back: bool,
    #[serde(rename = "Hamstrings", deserialize_with = "deserialize_bool")]
    pub hamstrings: bool,
    #[serde(rename = "Biceps", deserialize_with = "deserialize_bool")]
    pub biceps: bool,
    #[serde(rename = "Spinal Erectors", deserialize_with = "deserialize_bool")]
    pub spinal_erectors: bool,
    #[serde(rename = "Traps", deserialize_with = "deserialize_bool")]
    pub traps: bool,
    #[serde(rename = "Arms", deserialize_with = "deserialize_bool")]
    pub arms: bool,
    #[serde(rename = "Lats", deserialize_with = "deserialize_bool")]
    pub lats: bool,
    #[serde(rename = "Difficulty")]
    pub difficulty: Option<GarminDifficulty>,
    #[serde(rename = "Equipment")]
    pub equipment: String,
    #[serde(rename = "Nothing", deserialize_with = "deserialize_bool")]
    pub no_equipment: bool,
    #[serde(rename = "Bench", deserialize_with = "deserialize_bool")]
    pub bench: bool,
    #[serde(rename = "Barbell", deserialize_with = "deserialize_bool")]
    pub barbell: bool,
    #[serde(rename = "Dumbbells", deserialize_with = "deserialize_bool")]
    pub dumbbell: bool,
    #[serde(rename = "Kettlebells", deserialize_with = "deserialize_bool")]
    pub kettlebell: bool,
    #[serde(rename = "Sliding Discs", deserialize_with = "deserialize_bool")]
    pub sliding_discs: bool,
    #[serde(rename = "Box", deserialize_with = "deserialize_bool")]
    pub exercise_box: bool,
    #[serde(rename = "Dip Device", deserialize_with = "deserialize_bool")]
    pub dip_device: bool,
    #[serde(rename = "Squat Rack", deserialize_with = "deserialize_bool")]
    pub squat_rack: bool,
    #[serde(rename = "Jump Rope", deserialize_with = "deserialize_bool")]
    pub jump_rope: bool,
    #[serde(rename = "Mat", deserialize_with = "deserialize_bool")]
    pub mat: bool,
    #[serde(rename = "Ab Wheel", deserialize_with = "deserialize_bool")]
    pub ab_wheel: bool,
    #[serde(rename = "Weight Plates", deserialize_with = "deserialize_bool")]
    pub weight_plates: bool,
    #[serde(rename = "Swiss Ball", deserialize_with = "deserialize_bool")]
    pub swiss_ball: bool,
    #[serde(rename = "Cable Machine", deserialize_with = "deserialize_bool")]
    pub cable_machine: bool,
    #[serde(rename = "Cable Attachment", deserialize_with = "deserialize_bool")]
    pub cable_attatchment: bool,
    #[serde(rename = "Pull-up Bar", deserialize_with = "deserialize_bool")]
    pub pullup_bar: bool,
    #[serde(rename = "Focus")]
    pub focus: String,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GarminDifficulty {
    Beginner,
    #[serde(alias = "Intermmediate", alias = "5", alias = "6")]
    Intermediate,
    Advanced,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GarminCategory {
    BenchPress,
    Cardio,
    Core,
    Crunch,
    Curl,
    Deadlift,
    Flye,
    HipRaise,
    HipStability,
    HipSwing,
    Hyperextension,
    LateralRaise,
    LegCurl,
    LegRaise,
    Lunge,
    Plank,
    Plyo,
    PullUp,
    PushUp,
    Row,
    Run,
    ShoulderPress,
    Shrug,
    SitUp,
    Squat,
    TotalBody,
    TricepsExtension,
    WarmUp,
    CalfRaise,
    Carry,
    Chop,
    OlympicLift,
    ShoulderStability,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["1", "0"])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn read() {
        let fp = File::open("assets/GARMIN CONNECT EXERCICES - Exercices.csv").unwrap();
        let mut rdr = csv::Reader::from_reader(fp);
        for result in rdr.deserialize() {
            let _: GarminExercise = result.unwrap();
        }
    }
}
