use serde::{Deserialize, Serialize};

pub trait Weight {
    fn weight(&self) -> f64;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub(crate) height: f64,
    pub(crate) weight: f64,
}
pub(crate) mod parsers;
pub(crate) mod units;
