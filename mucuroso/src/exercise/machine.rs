use serde::{Deserialize, Serialize};

use crate::utils::Weight;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct Machine {
    name: String,
    weight: f64,
    note: Option<String>,
}
impl Weight for Machine {
    fn weight(&self) -> f64 {
        self.weight
    }
}
