use crate::{Resources, System};
use legion::world::World;
use winit::{
  event_loop::{ControlFlow, EventLoop},
  platform::desktop::EventLoopExtDesktop,
  window::WindowBuilder,
};

pub struct WindowSystem {
  event_loop: EventLoop<()>,
}

impl System for WindowSystem {
  fn new(resources: &mut Resources) -> Self {
    let event_loop = EventLoop::new();

    resources.window = Some(
      WindowBuilder::new()
        .with_title("Legion Filament")
        .build(&event_loop)
        .unwrap(),
    );

    WindowSystem { event_loop }
  }

  fn run(&mut self, _world: &mut World, resources: &mut Resources) {
    // Collect Winit events
    let mut events = Vec::with_capacity(100);

    self.event_loop.run_return(|event, _, control_flow| {
      events.push(event);
      *control_flow = ControlFlow::Exit;
    });

    // Drain them into the EventChannel
    resources.window_event_channel.drain_vec_write(&mut events);
  }
}
