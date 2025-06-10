pub mod constants;
pub mod exercise;
pub mod garmin;
pub mod session;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::session::SessionData;

    use crate::garmin::garmin_session::GarminSessionData;

    #[test]
    fn test_name() {
        let mut fp = File::open("assets/19258265404_ACTIVITY.fit").unwrap();
        let g_session_data: GarminSessionData =
            GarminSessionData::try_from_reader(&mut fp).unwrap();
        let session_data = SessionData::try_from(g_session_data).unwrap();
        dbg!(session_data);
    }
}
