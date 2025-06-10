use serde::{Deserialize, Serialize};

use crate::utils::Weight;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Dumbbell {
    pub(crate) weight: f64,
    pub(crate) note: Option<String>,
}

impl Weight for Dumbbell {
    fn weight(&self) -> f64 {
        self.weight
    }
}

impl From<f64> for Dumbbell {
    fn from(value: f64) -> Self {
        Self {
            weight: value,
            ..Default::default()
        }
    }
}
