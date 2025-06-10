use super::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub(crate) struct Target {
    pub(crate) group: TargetGroup,
    pub(crate) focus: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub(crate) enum TargetGroup {
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
}
