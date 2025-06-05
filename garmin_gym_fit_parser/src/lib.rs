pub mod exercise;
pub mod fitdata;
pub mod session;
pub mod sets;
pub mod user;
pub mod utils;

#[cfg(test)]
mod test {
    use crate::session::SessionData;
    use std::fs::File;
    #[test]
    fn general() {
        let mut fp = File::open("assets/19258265404_ACTIVITY.fit").unwrap();
        let session_data = SessionData::try_from_reader(&mut fp).unwrap();

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
