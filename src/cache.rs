use std::collections::{HashSet, HashMap};
use serde_json::{Result, Value, Map};
use serde::de::Error;
use crate::Trajectory::{Trajectory, Location};
use std::time::{SystemTime, UNIX_EPOCH};


pub struct Cache {
    pub entries: HashMap<String, Trajectory>,
    pub updated: HashMap<String, u64>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            entries: HashMap::new(),
            updated: HashMap::new(),
        }
    }

    pub fn write_and_get_delta(&mut self, entry: &str) -> Result<Trajectory> {
        let track: Trajectory = serde_json::from_str(entry)?;
        let old_track = self.entries.insert(track.id.clone(), track.clone());

        return match old_track {
            None => {Result::Ok(track.clone())}
            Some(mut v) => {
                let mut delta: Vec<Location> = Vec::new();
                for i in v.locations.len()..track.locations.len() {
                    delta.push(track.locations[i].clone())
                }
                self.entries.insert(track.id.clone(), track.clone());
                self.updated.insert(track.id.clone(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64);
                Result::Ok(Trajectory {
                    id: track.id.clone(),
                    startTime: track.startTime,
                    endTime: track.endTime,
                    updateTime: track.updateTime,
                    iteration: track.iteration,
                    locationCount: track.locationCount,
                    locations: delta
                })
            }
        }
    }






}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_add_to_an_empty_cache() -> Result<()>{
        let mut cache = Cache::new();

        let st = r#"{"endTime":1625235109601,"id":"device_4_1625235100357","iteration":38,"locationCount":38,"locations":[{"lat":13.0464967,"lng":47.8099047,"props":{},"timestamp":1625235100351},{"lat":13.0464797,"lng":47.8099107,"props":{},"timestamp":1625235100601},{"lat":13.0464627,"lng":47.8099173,"props":{},"timestamp":1625235100851},{"lat":13.0464455,"lng":47.8099241,"props":{},"timestamp":1625235101101},{"lat":13.0464287,"lng":47.8099308,"props":{},"timestamp":1625235101351},{"lat":13.0464129,"lng":47.8099374,"props":{},"timestamp":1625235101601},{"lat":13.0463987,"lng":47.8099449,"props":{},"timestamp":1625235101851},{"lat":13.0463832,"lng":47.809951,"props":{},"timestamp":1625235102101},{"lat":13.0463675,"lng":47.8099571,"props":{},"timestamp":1625235102351},{"lat":13.0463518,"lng":47.8099634,"props":{},"timestamp":1625235102601},{"lat":13.0463366,"lng":47.8099697,"props":{},"timestamp":1625235102851},{"lat":13.0463218,"lng":47.8099756,"props":{},"timestamp":1625235103101},{"lat":13.0463078,"lng":47.8099814,"props":{},"timestamp":1625235103351},{"lat":13.0462935,"lng":47.809987,"props":{},"timestamp":1625235103601},{"lat":13.0462792,"lng":47.8099927,"props":{},"timestamp":1625235103851},{"lat":13.046265,"lng":47.8099984,"props":{},"timestamp":1625235104101},{"lat":13.0462509,"lng":47.8100042,"props":{},"timestamp":1625235104351},{"lat":13.0462363,"lng":47.8100098,"props":{},"timestamp":1625235104601},{"lat":13.0462216,"lng":47.8100153,"props":{},"timestamp":1625235104851},{"lat":13.0462068,"lng":47.810021,"props":{},"timestamp":1625235105101},{"lat":13.0461929,"lng":47.8100266,"props":{},"timestamp":1625235105351},{"lat":13.0461789,"lng":47.8100328,"props":{},"timestamp":1625235105601},{"lat":13.0461648,"lng":47.8100391,"props":{},"timestamp":1625235105851},{"lat":13.0461509,"lng":47.8100453,"props":{},"timestamp":1625235106101},{"lat":13.046137,"lng":47.8100519,"props":{},"timestamp":1625235106351},{"lat":13.0461233,"lng":47.8100584,"props":{},"timestamp":1625235106601},{"lat":13.0461095,"lng":47.8100647,"props":{},"timestamp":1625235106851},{"lat":13.0460955,"lng":47.8100711,"props":{},"timestamp":1625235107101},{"lat":13.0460815,"lng":47.8100774,"props":{},"timestamp":1625235107351},{"lat":13.0460679,"lng":47.8100834,"props":{},"timestamp":1625235107601},{"lat":13.0460541,"lng":47.8100893,"props":{},"timestamp":1625235107851},{"lat":13.0460406,"lng":47.810095,"props":{},"timestamp":1625235108101},{"lat":13.0460269,"lng":47.8101006,"props":{},"timestamp":1625235108351},{"lat":13.0460134,"lng":47.8101061,"props":{},"timestamp":1625235108601},{"lat":13.046,"lng":47.8101116,"props":{},"timestamp":1625235108851},{"lat":13.0459863,"lng":47.810117,"props":{},"timestamp":1625235109101},{"lat":13.0459726,"lng":47.8101224,"props":{},"timestamp":1625235109351},{"lat":13.0459591,"lng":47.8101281,"props":{},"timestamp":1625235109601}],"startTime":1625235100351,"updateTime":1625235109627}"#;
        let st_as_obj: Trajectory = serde_json::from_str(st)?;
        let track = cache.write_and_get_delta(st).unwrap();
        assert_eq!(st_as_obj.id, track.id);
        Ok(())
    }
    #[test]
    fn can_add_to_a_non_empty_cache() -> Result<()>{
        let mut cache = Cache::new();

        let st = r#"{"endTime":1625235109601,"id":"device_4_1625235100357","iteration":38,"locationCount":38,"locations":[{"lat":13.0464967,"lng":47.8099047,"props":{},"timestamp":1625235100351},{"lat":13.0464797,"lng":47.8099107,"props":{},"timestamp":1625235100601},{"lat":13.0464627,"lng":47.8099173,"props":{},"timestamp":1625235100851},{"lat":13.0464455,"lng":47.8099241,"props":{},"timestamp":1625235101101},{"lat":13.0464287,"lng":47.8099308,"props":{},"timestamp":1625235101351},{"lat":13.0464129,"lng":47.8099374,"props":{},"timestamp":1625235101601},{"lat":13.0463987,"lng":47.8099449,"props":{},"timestamp":1625235101851},{"lat":13.0463832,"lng":47.809951,"props":{},"timestamp":1625235102101},{"lat":13.0463675,"lng":47.8099571,"props":{},"timestamp":1625235102351},{"lat":13.0463518,"lng":47.8099634,"props":{},"timestamp":1625235102601},{"lat":13.0463366,"lng":47.8099697,"props":{},"timestamp":1625235102851},{"lat":13.0463218,"lng":47.8099756,"props":{},"timestamp":1625235103101},{"lat":13.0463078,"lng":47.8099814,"props":{},"timestamp":1625235103351},{"lat":13.0462935,"lng":47.809987,"props":{},"timestamp":1625235103601},{"lat":13.0462792,"lng":47.8099927,"props":{},"timestamp":1625235103851},{"lat":13.046265,"lng":47.8099984,"props":{},"timestamp":1625235104101},{"lat":13.0462509,"lng":47.8100042,"props":{},"timestamp":1625235104351},{"lat":13.0462363,"lng":47.8100098,"props":{},"timestamp":1625235104601},{"lat":13.0462216,"lng":47.8100153,"props":{},"timestamp":1625235104851},{"lat":13.0462068,"lng":47.810021,"props":{},"timestamp":1625235105101},{"lat":13.0461929,"lng":47.8100266,"props":{},"timestamp":1625235105351},{"lat":13.0461789,"lng":47.8100328,"props":{},"timestamp":1625235105601},{"lat":13.0461648,"lng":47.8100391,"props":{},"timestamp":1625235105851},{"lat":13.0461509,"lng":47.8100453,"props":{},"timestamp":1625235106101},{"lat":13.046137,"lng":47.8100519,"props":{},"timestamp":1625235106351},{"lat":13.0461233,"lng":47.8100584,"props":{},"timestamp":1625235106601},{"lat":13.0461095,"lng":47.8100647,"props":{},"timestamp":1625235106851},{"lat":13.0460955,"lng":47.8100711,"props":{},"timestamp":1625235107101},{"lat":13.0460815,"lng":47.8100774,"props":{},"timestamp":1625235107351},{"lat":13.0460679,"lng":47.8100834,"props":{},"timestamp":1625235107601},{"lat":13.0460541,"lng":47.8100893,"props":{},"timestamp":1625235107851},{"lat":13.0460406,"lng":47.810095,"props":{},"timestamp":1625235108101},{"lat":13.0460269,"lng":47.8101006,"props":{},"timestamp":1625235108351},{"lat":13.0460134,"lng":47.8101061,"props":{},"timestamp":1625235108601},{"lat":13.046,"lng":47.8101116,"props":{},"timestamp":1625235108851},{"lat":13.0459863,"lng":47.810117,"props":{},"timestamp":1625235109101},{"lat":13.0459726,"lng":47.8101224,"props":{},"timestamp":1625235109351},{"lat":13.0459591,"lng":47.8101281,"props":{},"timestamp":1625235109601}],"startTime":1625235100351,"updateTime":1625235109627}"#;
        let st_after = r#"{"endTime":1625235112351,"id":"device_4_1625235100357","iteration":49,"locationCount":49,"locations":[{"lat":13.0464967,"lng":47.8099047,"props":{},"timestamp":1625235100351},{"lat":13.0464797,"lng":47.8099107,"props":{},"timestamp":1625235100601},{"lat":13.0464627,"lng":47.8099173,"props":{},"timestamp":1625235100851},{"lat":13.0464455,"lng":47.8099241,"props":{},"timestamp":1625235101101},{"lat":13.0464287,"lng":47.8099308,"props":{},"timestamp":1625235101351},{"lat":13.0464129,"lng":47.8099374,"props":{},"timestamp":1625235101601},{"lat":13.0463987,"lng":47.8099449,"props":{},"timestamp":1625235101851},{"lat":13.0463832,"lng":47.809951,"props":{},"timestamp":1625235102101},{"lat":13.0463675,"lng":47.8099571,"props":{},"timestamp":1625235102351},{"lat":13.0463518,"lng":47.8099634,"props":{},"timestamp":1625235102601},{"lat":13.0463366,"lng":47.8099697,"props":{},"timestamp":1625235102851},{"lat":13.0463218,"lng":47.8099756,"props":{},"timestamp":1625235103101},{"lat":13.0463078,"lng":47.8099814,"props":{},"timestamp":1625235103351},{"lat":13.0462935,"lng":47.809987,"props":{},"timestamp":1625235103601},{"lat":13.0462792,"lng":47.8099927,"props":{},"timestamp":1625235103851},{"lat":13.046265,"lng":47.8099984,"props":{},"timestamp":1625235104101},{"lat":13.0462509,"lng":47.8100042,"props":{},"timestamp":1625235104351},{"lat":13.0462363,"lng":47.8100098,"props":{},"timestamp":1625235104601},{"lat":13.0462216,"lng":47.8100153,"props":{},"timestamp":1625235104851},{"lat":13.0462068,"lng":47.810021,"props":{},"timestamp":1625235105101},{"lat":13.0461929,"lng":47.8100266,"props":{},"timestamp":1625235105351},{"lat":13.0461789,"lng":47.8100328,"props":{},"timestamp":1625235105601},{"lat":13.0461648,"lng":47.8100391,"props":{},"timestamp":1625235105851},{"lat":13.0461509,"lng":47.8100453,"props":{},"timestamp":1625235106101},{"lat":13.046137,"lng":47.8100519,"props":{},"timestamp":1625235106351},{"lat":13.0461233,"lng":47.8100584,"props":{},"timestamp":1625235106601},{"lat":13.0461095,"lng":47.8100647,"props":{},"timestamp":1625235106851},{"lat":13.0460955,"lng":47.8100711,"props":{},"timestamp":1625235107101},{"lat":13.0460815,"lng":47.8100774,"props":{},"timestamp":1625235107351},{"lat":13.0460679,"lng":47.8100834,"props":{},"timestamp":1625235107601},{"lat":13.0460541,"lng":47.8100893,"props":{},"timestamp":1625235107851},{"lat":13.0460406,"lng":47.810095,"props":{},"timestamp":1625235108101},{"lat":13.0460269,"lng":47.8101006,"props":{},"timestamp":1625235108351},{"lat":13.0460134,"lng":47.8101061,"props":{},"timestamp":1625235108601},{"lat":13.046,"lng":47.8101116,"props":{},"timestamp":1625235108851},{"lat":13.0459863,"lng":47.810117,"props":{},"timestamp":1625235109101},{"lat":13.0459726,"lng":47.8101224,"props":{},"timestamp":1625235109351},{"lat":13.0459591,"lng":47.8101281,"props":{},"timestamp":1625235109601},{"lat":13.0459452,"lng":47.8101337,"props":{},"timestamp":1625235109851},{"lat":13.0459311,"lng":47.8101391,"props":{},"timestamp":1625235110101},{"lat":13.0459171,"lng":47.8101446,"props":{},"timestamp":1625235110351},{"lat":13.0459037,"lng":47.8101498,"props":{},"timestamp":1625235110601},{"lat":13.0458918,"lng":47.8101546,"props":{},"timestamp":1625235110851},{"lat":13.0458761,"lng":47.8101558,"props":{},"timestamp":1625235111101},{"lat":13.0458665,"lng":47.810159,"props":{},"timestamp":1625235111351},{"lat":13.0458583,"lng":47.8101611,"props":{},"timestamp":1625235111601},{"lat":13.0458508,"lng":47.810162,"props":{},"timestamp":1625235111851},{"lat":13.0458435,"lng":47.810163,"props":{},"timestamp":1625235112101},{"lat":13.0458363,"lng":47.8101646,"props":{},"timestamp":1625235112351}],"startTime":1625235100351,"updateTime":1625235112385}"#;
        cache.write_and_get_delta(st).unwrap();
        let st_as_obj: Trajectory = serde_json::from_str(st)?;
        let delta_track = cache.write_and_get_delta(st_after).unwrap();
        assert_eq!(delta_track.id, st_as_obj.id);
        println!("{} {}", delta_track.locations.len(), st_as_obj.locations.len());
        // assert!(delta_track.locations.len() < st_as_obj.locations.len());
        Ok(())
    }

}
