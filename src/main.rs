#![feature(decl_macro)]
#![feature(option_result_contains)]
extern crate log;
#[macro_use]
extern crate rocket;

use std::time::Duration;

use futures_util::future::RemoteHandle;
use riker::actors::*;
use riker_patterns::ask::*;
use uuid::Uuid;
use log::info;

use crate::actors::{GardenerState, GetGardenerState};

mod probe;
mod actors;

// start the system and create an actor
fn main() {
    let sys = ActorSystem::with_name("rust-actor").unwrap();

    let props = Props::new::<actors::Gardener>();
    let gardener = sys.actor_of_props("gardener", props).unwrap();

    let garden_id = Uuid::new_v4().to_string();
    gardener.tell(
        actors::GardenNotification {
            garden: garden_id.clone(),
            sensor_metric: actors::SensorMetric {
                sensor: "BPM180".to_string(),
                metric: "Air pressure HPa".to_string(),
                value: 1026.018,
            },
        }, None);

    gardener.tell(
        actors::GardenNotification {
            garden: Uuid::new_v4().to_string(),
            sensor_metric: actors::SensorMetric {
                sensor: "BPM180".to_string(),
                metric: "Air pressure HPa".to_string(),
                value: 1029.018,
            },
        }, None);

    gardener.tell(
        actors::GardenNotification {
            garden: garden_id.clone(),
            sensor_metric: actors::SensorMetric {
                sensor: "BPM180".to_string(),
                metric: "Air pressure HPa".to_string(),
                value: 1026.028,
            },
        }, None);


    let res: RemoteHandle<GardenerState> = ask(&sys, &gardener, GetGardenerState);

    info!("Gardener state: {:?}",res);

    //rocket::ignite().mount("/", routes![probe::probe_out]).launch();

    // force main to wait before exiting program
    std::thread::sleep(Duration::from_millis(5000));
}