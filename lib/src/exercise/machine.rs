use serde::{Deserialize, Serialize};

use crate::utils::Weight;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Machine {
    pub name: String,
    pub weight: f64,
    pub note: Option<String>,
}
impl Weight for Machine {
    fn weight(&self) -> f64 {
        self.weight
    }
}
