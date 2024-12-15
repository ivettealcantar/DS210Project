use crate::data_processing::{CleanRecord, filter_by_state};

pub fn compare_states(
    records: &[CleanRecord],
    state1: &str,
    state2: &str,
) -> (Vec<CleanRecord>, Vec<CleanRecord>) {
    let state1_data = filter_by_state(records, state1);
    let state2_data = filter_by_state(records, state2);
    (state1_data, state2_data)
}