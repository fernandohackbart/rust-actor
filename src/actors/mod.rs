use std::collections::HashMap;

use riker::actors::*;

//SensorMetric
#[derive(Clone, Debug)]
pub struct SensorMetric {
    pub sensor: String,
    pub metric: String,
    pub value: f64,
}

// GardenNotification
#[derive(Clone, Debug)]
pub struct GardenNotification {
    pub garden: String,
    pub sensor_metric: SensorMetric,
}

// GardenNotification
#[derive(Clone, Debug)]
pub struct SeedbedNotification {
    pub garden: String,
    pub seedbed: String,
    pub sensor_metric: SensorMetric,
}

#[actor(GardenNotification, SeedbedNotification)]
#[derive(Default, Debug, Clone)]
pub struct Garden {
    messages: u32,
    //sensors_metrics: HashMap<String, SensorMetric>,
}

/**
 Garden: digital twin for a garden
**/
impl Actor for Garden {
    type Msg = GardenMsg;

    fn recv(&mut self,
            ctx: &Context<Self::Msg>,
            msg: Self::Msg,
            sender: Sender) {
        // Use the respective Receive<T> implementation
        self.receive(ctx, msg, sender);
    }
}

impl Receive<GardenNotification> for Garden {
    type Msg = GardenMsg;

    fn receive(&mut self,
               _ctx: &Context<Self::Msg>,
               _msg: GardenNotification,
               _sender: Sender) {
        println!("Garden received {:?}", _msg);
        self.messages += 1;
    }
}

impl Receive<SeedbedNotification> for Garden {
    type Msg = GardenMsg;

    fn receive(&mut self,
               _ctx: &Context<Self::Msg>,
               _msg: SeedbedNotification,
               _sender: Sender) {
        println!("Garden received {:?}", _msg);
        self.messages += 1;
    }
}

/**
 Gardener: actor that keeps the gardens
 **/
#[actor(GardenNotification, SeedbedNotification)]
#[derive(Default)]
pub struct Gardener {
    messages: u32,
    gardens: HashMap<String, ActorRef<GardenMsg>>,
}

impl Actor for Gardener {
    type Msg = GardenerMsg;

    fn recv(&mut self,
            ctx: &Context<Self::Msg>,
            msg: Self::Msg,
            sender: Sender) {
        // Use the respective Receive<T> implementation
        self.receive(ctx, msg, sender);
    }
}

impl Receive<GardenNotification> for Gardener {
    type Msg = GardenerMsg;

    fn receive(&mut self,
               _ctx: &Context<Self::Msg>,
               _msg: GardenNotification,
               _sender: Sender) {
        println!("Gardener received {:?}", _msg);
        self.messages += 1;

        // let mut garden_ref: Option<&ActorRef<Garden>>;
        // let x = &_ctx.system.actor_of::<Garden>(&_msg.garden).unwrap();
        // x.tell(_msg, _sender);

        //TODO: Sender should be the gardener

        //TODO: utilizar localização de atores ao invés de HashMap

        match self.gardens.get(&_msg.garden) {
            None => {
                let actor_ref = &_ctx.system.actor_of::<Garden>(&_msg.garden).unwrap();
                self.gardens.insert(_msg.clone().garden, *actor_ref);
                actor_ref.tell(_msg, _sender);
            }
            Some(actor_ref) => {
                // check if the actorRef is valid
                // if not valid create a new actor and update the actorRef in the HashMap
                // select the actorRef (may not be in the HashMap but exists)
                actor_ref.tell(_msg, _sender)
            }
        }
    }
}

impl Receive<SeedbedNotification> for Gardener {
    type Msg = GardenerMsg;

    fn receive(&mut self,
               _ctx: &Context<Self::Msg>,
               _msg: SeedbedNotification,
               _sender: Sender) {
        println!("Gardener received {:?}", _msg);
        self.messages += 1;
    }
}