use legion::world::World;
use shrev::EventChannel;
use winit::{Event, Window};

pub struct Resources {
  pub window: Option<Window>,
  pub window_event_channel: EventChannel<Event>,
}

impl Resources {
  pub fn new() -> Self {
    Self {
      window: None,
      window_event_channel: EventChannel::new(),
    }
  }
}

pub trait System {
  fn new(resources: &mut Resources) -> Self;
  fn run(&mut self, world: &mut World, resources: &mut Resources);
}
