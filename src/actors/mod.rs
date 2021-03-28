use riker::actors::*;

#[path = "../mqtt_client/mod.rs"]
mod mqtt_client;

#[derive(Default)]
pub(crate) struct MyActor;

impl Actor for MyActor {
    type Msg = String;

    fn recv(&mut self,
            _ctx: &Context<String>,
            msg: String,
            _sender: Sender) {
        println!("Received: {}", msg);
    }
}
