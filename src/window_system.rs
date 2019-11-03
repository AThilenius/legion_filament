use crate::{input_handler::InputHandler, ThreadLocalSystem};
use legion::prelude::*;
use shrev::EventChannel;
use winit::{Event, EventsLoop, WindowBuilder};

pub struct WindowSystem {
    events_loop: EventsLoop,
}

impl ThreadLocalSystem for WindowSystem {
    fn new(world: &mut World) -> Self {
        let events_loop = EventsLoop::new();
        world.resources.insert(
            WindowBuilder::new()
                .with_title("Legion Filament")
                .build(&events_loop)
                .unwrap(),
        );

        world.resources.insert(EventChannel::<Event>::new());

        WindowSystem { events_loop }
    }

    fn run(&mut self, world: &mut World) {
        // Collect Winit events
        let mut events = Vec::with_capacity(100);
        self.events_loop.poll_events(|event| {
            events.push(event);
        });

        // Drain them into the EventChannel
        let mut event_channel = world.resources.get_mut::<EventChannel<Event>>().unwrap();
        event_channel.drain_vec_write(&mut events);
    }
}
