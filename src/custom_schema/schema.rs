use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
struct TrackLocation {
    lat: f64,
    lng: f64,
    timestamp: i64,
    props: HashMap<String, String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    id: String,
    startTime: i64,
    endTime: i64,
    updateTime: i64,
    iteration: i32,
    locationCount: i32,
    locations: Vec<TrackLocation>,
}