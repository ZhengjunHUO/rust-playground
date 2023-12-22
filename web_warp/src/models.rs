use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Deserialize, Serialize)]
pub struct Candidate {
    pub name: String,
    pub votes: u32,
}

pub type CandidateList = Arc<Mutex<HashMap<String, u32>>>;

pub fn init_demo_db() -> CandidateList {
    Arc::new(Mutex::new(HashMap::from([
        (String::from("huo"), 0),
        (String::from("wang"), 0),
        (String::from("fufu"), 0),
    ])))
}
