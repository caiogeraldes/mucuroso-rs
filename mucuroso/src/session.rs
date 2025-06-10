use chrono::{DateTime, FixedOffset};
use garmin_gym_fit_parser::session::SessionData as GarminSessionData;

pub struct SessionData {
    timestamp: DateTime<FixedOffset>,
    // sets: Vec<Set>,
    // user: User,
}

impl TryFrom<GarminSessionData> for SessionData {
    type Error = anyhow::Error;

    fn try_from(value: GarminSessionData) -> Result<Self, Self::Error> {
        todo!()
    }
}
