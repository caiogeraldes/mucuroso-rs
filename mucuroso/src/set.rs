use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::exercise::Exercise;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Set {
    duration: u16,
    set_type: String,
    start_time: DateTime<FixedOffset>,
    exercise: Exercise,
    weight: Option<u16>,
    repetitions: u16,
    wkt_step_index: String,
}
