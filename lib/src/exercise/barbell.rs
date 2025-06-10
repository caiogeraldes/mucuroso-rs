use crate::utils::Weight;

use super::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Barbell {
    pub(crate) bar_weight: f64,
    pub(crate) added_weight: f64,
    pub(crate) note: Option<String>,
}
impl Weight for Barbell {
    fn weight(&self) -> f64 {
        self.bar_weight + self.added_weight
    }
}
