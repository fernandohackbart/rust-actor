#![feature(decl_macro)]
#[macro_use] extern crate rocket;

mod probe;
mod actors;
use riker::actors::*;

// start the system and create an actor
fn main() {
    let sys = ActorSystem::new().unwrap();

    let my_actor = sys.actor_of::<actors::MyActor>("my-actor").unwrap();
    my_actor.tell("Subscribe".to_string(), None);

    rocket::ignite().mount("/", routes![probe::probe_out]).launch();
}