use serde::Serialize;
use std::collections::BTreeMap;
/// Alternate serialization format
#[derive(Clone, Debug, Serialize)]
pub struct FitDataMap {
    pub(crate) kind: fitparser::profile::MesgNum,
    pub(crate) fields: BTreeMap<String, fitparser::ValueWithUnits>,
}

impl FitDataMap {
    pub fn new(record: fitparser::FitDataRecord) -> Self {
        FitDataMap {
            kind: record.kind(),
            fields: record
                .into_vec()
                .into_iter()
                .map(|f| (f.name().to_owned(), fitparser::ValueWithUnits::from(f)))
                .collect(),
        }
    }
}
