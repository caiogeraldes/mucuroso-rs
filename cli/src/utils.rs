use chrono::DateTime;
use chrono::FixedOffset;
use mucuroso::exercise::Set;
use mucuroso::utils::Weight;
use serde::{Deserialize, Serialize};

/// Helper struct to transform set data into a csv line
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct CsvSet {
    name: String,
    duration: f64,
    weight: f64,
    repetitions: u16,
    set_type: String,
    start_time: DateTime<FixedOffset>,
    wkt_step_index: String,
    chest: bool,
    triceps: bool,
    delts: bool,
    abs: bool,
    hips: bool,
    quads: bool,
    glutes: bool,
    back: bool,
    calves: bool,
    total_body: bool,
    legs: bool,
    obliques: bool,
    core: bool,
    upper_back: bool,
    lower_back: bool,
    hamstrings: bool,
    biceps: bool,
    spinal_erectors: bool,
    traps: bool,
    arms: bool,
    lats: bool,
}

impl From<&Set> for CsvSet {
    fn from(value: &Set) -> Self {
        let (
            mut chest,
            mut triceps,
            mut delts,
            mut abs,
            mut hips,
            mut quads,
            mut glutes,
            mut back,
            mut calves,
            mut total_body,
            mut legs,
            mut obliques,
            mut core_,
            mut upper_back,
            mut lower_back,
            mut hamstrings,
            mut biceps,
            mut spinal_erectors,
            mut traps,
            mut arms,
            mut lats,
        ) = (
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false,
        );

        for target in value.target_muscles() {
            match target {
                mucuroso::exercise::target::Target::Other => (),
                mucuroso::exercise::target::Target::Cardiovascular => (),
                mucuroso::exercise::target::Target::Abdomen => abs = true,
                mucuroso::exercise::target::Target::Biceps => biceps = true,
                mucuroso::exercise::target::Target::Calves => calves = true,
                mucuroso::exercise::target::Target::Chest => chest = true,
                mucuroso::exercise::target::Target::Forearms => (),
                mucuroso::exercise::target::Target::Glutes => glutes = true,
                mucuroso::exercise::target::Target::Hamstrings => hamstrings = true,
                mucuroso::exercise::target::Target::HipAbductors => hips = true,
                mucuroso::exercise::target::Target::HipAdductors => hips = true,
                mucuroso::exercise::target::Target::HipFlexors => hips = true,
                mucuroso::exercise::target::Target::LatissimusDorsi => lats = true,
                mucuroso::exercise::target::Target::LowerBack => lower_back = true,
                mucuroso::exercise::target::Target::Obliques => obliques = true,
                mucuroso::exercise::target::Target::Quadriceps => quads = true,
                mucuroso::exercise::target::Target::Deltoids => delts = true,
                mucuroso::exercise::target::Target::Trapezius => traps = true,
                mucuroso::exercise::target::Target::Triceps => triceps = true,
                mucuroso::exercise::target::Target::Neck => (),
                mucuroso::exercise::target::Target::Hips => hips = true,
                mucuroso::exercise::target::Target::Back => back = true,
                mucuroso::exercise::target::Target::TotalBody => total_body = true,
                mucuroso::exercise::target::Target::UpperBack => upper_back = true,
                mucuroso::exercise::target::Target::SpinalErectors => spinal_erectors = true,
                mucuroso::exercise::target::Target::Core => core_ = true,
                mucuroso::exercise::target::Target::Legs => legs = true,
                mucuroso::exercise::target::Target::Arms => arms = true,
            }
        }

        Self {
            name: value.name().into(),
            repetitions: value.repetitions(),
            duration: value.duration(),
            set_type: value.set_type().into(),
            start_time: value.start_time(),
            wkt_step_index: value.wkt_step_index().into(),
            weight: value.weight(),
            chest,
            triceps,
            delts,
            abs,
            hips,
            quads,
            glutes,
            back,
            calves,
            total_body,
            legs,
            obliques,
            core: core_,
            upper_back,
            lower_back,
            hamstrings,
            biceps,
            spinal_erectors,
            traps,
            arms,
            lats,
        }
    }
}
