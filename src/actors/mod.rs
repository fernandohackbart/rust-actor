use std::collections::HashMap;

use log::{info, warn};
use riker::actors::*;
//use riker_patterns::ask;

/**
 SensorMetric
**/
#[derive(Clone, Debug)]
pub struct SensorMetric {
    pub sensor: String,
    pub metric: String,
    pub value: f64,
}

/**
  GardenNotification
**/
#[derive(Clone, Debug)]
pub struct GardenNotification {
    pub garden: String,
    pub sensor_metric: SensorMetric,
}

/**
  SeedbedNotification
**/
#[derive(Clone, Debug)]
pub struct SeedbedNotification {
    pub garden: String,
    pub seedbed: String,
    pub sensor_metric: SensorMetric,
}

/**
  GetGardenState
**/
#[derive(Clone, Debug)]
pub struct GetGardenState;

/**
  GardenState
**/
#[derive(Clone, Debug)]
pub struct GardenState {
    pub garden: String,
    pub sensor_metrics: Vec<SensorMetric>,
}

/**
  GetGardenerState
**/
#[derive(Clone, Debug)]
pub struct GetGardenerState;

/**
  GardenerState
**/
#[derive(Clone, Debug)]
pub struct GardenerState {
    pub messages: u32,
    pub gardens: Vec<GardenState>,
}

/**
  Garden
**/
#[actor(GardenNotification, SeedbedNotification, GetGardenState)]
#[derive(Default, Debug, Clone)]
pub struct Garden {
    messages: u32,
    sensor_metrics: HashMap<String, SensorMetric>,
    //TODO: keep de history oe N metrics
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
               ctx: &Context<Self::Msg>,
               msg: GardenNotification,
               _sender: Sender) {
        info!("Garden {} received {:?}", &ctx.myself.name(), msg);
        self.messages += 1;
        self.sensor_metrics.entry(msg.sensor_metric.sensor.clone()).or_insert(msg.sensor_metric);
    }
}

impl Receive<GetGardenState> for Garden {
    type Msg = GardenMsg;

    fn receive(&mut self,
               ctx: &Context<Self::Msg>,
               msg: GetGardenState,
               _sender: Sender) {
        info!("Garden {} received {:?}", &ctx.myself.name(), msg);
        self.messages += 1;
    }
}

impl Receive<SeedbedNotification> for Garden {
    type Msg = GardenMsg;

    fn receive(&mut self,
               ctx: &Context<Self::Msg>,
               msg: SeedbedNotification,
               _sender: Sender) {
        info!("Garden {} received {:?}", &ctx.myself.name(), msg);
        self.messages += 1;
    }
}

/**
 Gardener: actor that keeps the gardens
 **/
#[actor(GardenNotification, SeedbedNotification, GetGardenerState)]
#[derive(Default)]
pub struct Gardener {
    messages: u32,
    gardens: HashMap<String, ActorRef<GardenMsg>>,
}

impl Actor for Gardener {
    type Msg = GardenerMsg;

    // fn post_start(&mut self, ctx: &Context<Self::Msg>){
    //   self.gardens.en
    // }

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
               ctx: &Context<Self::Msg>,
               msg: GardenNotification,
               sender: Sender) {
        info!("Gardener received {:?}", msg);
        ctx.system.print_tree();

        self.messages += 1;

        if !self.gardens.contains_key(&msg.garden) {
            let child_exists = ctx.myself.children().map(|child| child.name() == msg.garden).fold(false, |acc, exists| acc || exists);
            if child_exists {
                warn!("About to create a child that already exists {}", &msg.garden);
            }
            let garden_actor: ActorRef<GardenMsg> = ctx.actor_of::<Garden>(&msg.garden).unwrap();
            self.gardens.insert(msg.clone().garden, garden_actor);
        }
        let garden_ref = self.gardens.get(&msg.garden).unwrap();
        garden_ref.tell(msg, sender);
    }
}

impl Receive<SeedbedNotification> for Gardener {
    type Msg = GardenerMsg;

    fn receive(&mut self,
               _ctx: &Context<Self::Msg>,
               msg: SeedbedNotification,
               _sender: Sender) {
        info!("Gardener received {:?}", msg);
        self.messages += 1;
    }
}

impl Receive<GetGardenerState> for Gardener {
    type Msg = GardenerMsg;

    fn receive(&mut self,
               ctx: &Context<Self::Msg>,
               msg: GetGardenerState,
               sender: Sender) {
        info!("Gardener received {:?}", msg);
        sender.try_tell(
            GardenerState {
                messages: self.messages,
                gardens: Vec::new(),
            },
            &ctx.myself
        );
    }
}