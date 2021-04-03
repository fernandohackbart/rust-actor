#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use riker::actors::*;
use uuid::Uuid;

mod probe;
mod actors;

// start the system and create an actor
fn main() {
    let sys = ActorSystem::new().unwrap();

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
            garden: garden_id.clone(),
            sensor_metric: actors::SensorMetric {
                sensor: "BPM180".to_string(),
                metric: "Air pressure HPa".to_string(),
                value: 1026.018,
            },
        }, None);

    rocket::ignite().mount("/", routes![probe::probe_out]).launch();
}