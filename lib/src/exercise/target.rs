use crate::garmin::garmin_exercises::GarminExercise;

use super::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Clone)]
pub enum Target {
    #[default]
    Other,
    Cardiovascular,
    #[serde(alias = "Abs")]
    Abdomen,
    Biceps,
    Calves,
    Chest,
    Forearms,
    Glutes,
    Hamstrings,
    HipAbductors,
    HipAdductors,
    HipFlexors,
    #[serde(alias = "Lats")]
    LatissimusDorsi,
    LowerBack,
    Obliques,
    #[serde(alias = "Quads")]
    Quadriceps,
    #[serde(alias = "Delts", alias = "Shoulders")]
    Deltoids,
    #[serde(alias = "Traps")]
    Trapezius,
    Triceps,
    Neck,
    Hips,
    Back,
    TotalBody,
    UpperBack,
    SpinalErectors,
    Core,
    Legs,
    Arms,
}

pub fn gen_target(exercise: &GarminExercise) -> Vec<Target> {
    let mut output = vec![];
    if exercise.chest {
        output.push(Target::Chest)
    }
    if exercise.triceps {
        output.push(Target::Triceps)
    }
    if exercise.delts {
        output.push(Target::Deltoids)
    }
    if exercise.abs {
        output.push(Target::Abdomen)
    }
    if exercise.pecs {
        output.push(Target::Chest)
    }
    if exercise.shoulders {
        output.push(Target::Deltoids)
    }
    if exercise.hips {
        output.push(Target::Hips)
    }
    if exercise.quads {
        output.push(Target::Quadriceps)
    }
    if exercise.glutes {
        output.push(Target::Glutes)
    }
    if exercise.back {
        output.push(Target::Back)
    }
    if exercise.calves {
        output.push(Target::Calves)
    }
    if exercise.total_body {
        output.push(Target::TotalBody)
    }
    if exercise.legs {
        output.push(Target::Legs)
    }
    if exercise.obliques {
        output.push(Target::Obliques)
    }
    if exercise.core {
        output.push(Target::Core)
    }
    if exercise.upper_back {
        output.push(Target::UpperBack)
    }
    if exercise.lower_back {
        output.push(Target::LowerBack)
    }
    if exercise.hamstrings {
        output.push(Target::Hamstrings)
    }
    if exercise.biceps {
        output.push(Target::Biceps)
    }
    if exercise.spinal_erectors {
        output.push(Target::SpinalErectors)
    }
    if exercise.traps {
        output.push(Target::Trapezius)
    }
    if exercise.arms {
        output.push(Target::Arms)
    }
    if exercise.lats {
        output.push(Target::LatissimusDorsi)
    }
    output
}
