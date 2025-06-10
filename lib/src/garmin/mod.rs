pub mod exercise_title;
pub mod fitdata;
pub mod garmin_exercises;
pub mod garmin_session;
pub mod garmin_sets;
pub mod garmin_user;

#[cfg(test)]
mod test {
    use crate::garmin::garmin_session::GarminSessionData;
    use std::fs::File;
    #[test]
    fn general() {
        let mut fp = File::open("assets/19258265404_ACTIVITY.fit").unwrap();
        let session_data = GarminSessionData::try_from_reader(&mut fp).unwrap();

        dbg!(session_data.timestamp());

        let sets_file = File::create("output/sets.csv").unwrap();
        let mut wtr = csv::Writer::from_writer(sets_file);
        for set in session_data.sets() {
            wtr.serialize(set).unwrap();
        }
        wtr.flush().unwrap();

        let exercise_file = File::create("output/exercise_types.csv").unwrap();
        let mut wtr = csv::Writer::from_writer(exercise_file);
        for exercise in session_data.exercise_titles() {
            wtr.serialize(exercise).unwrap();
        }
        wtr.flush().unwrap();
    }
}
