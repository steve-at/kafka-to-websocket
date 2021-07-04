use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
    pub timestamp: u64,
    pub props: HashMap<String, String>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trajectory {
    pub id: String,
    pub startTime: u64,
    pub endTime: u64,
    pub iteration: u32,
    pub updateTime: u64,
    pub locationCount: u32,
    pub locations: Vec<Location>,
}