use crate::{Resources, System};
use legion::world::World;
use winit::{EventsLoop, WindowBuilder};

pub struct WindowSystem {
  events_loop: EventsLoop,
}

impl System for WindowSystem {
  fn new(resources: &mut Resources) -> Self {
    let events_loop = EventsLoop::new();
    resources.window = Some(
      WindowBuilder::new()
        .with_title("Legion Filament")
        .build(&events_loop)
        .unwrap(),
    );

    WindowSystem { events_loop }
  }

  fn run(&mut self, _world: &mut World, resources: &mut Resources) {
    // Collect Winit events
    let mut events = Vec::with_capacity(100);
    self.events_loop.poll_events(|event| {
      events.push(event);
    });

    // Drain them into the EventChannel
    resources.window_event_channel.drain_vec_write(&mut events);
  }
}
