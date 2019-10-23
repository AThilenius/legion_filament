use crate::ThreadLocalSystem;
use legion::prelude::*;
use shrev::EventChannel;
use winit::{Event, EventsLoop, WindowBuilder};

pub struct WindowSystem {
    pub window_event_channel: EventChannel<Event>,
    events_loop: EventsLoop,
}

impl ThreadLocalSystem for WindowSystem {
    fn new(_world: &mut World, resources: &mut Resources) -> Self {
        let events_loop = EventsLoop::new();
        resources.insert(
            WindowBuilder::new()
                .with_title("Legion Filament")
                .build(&events_loop)
                .unwrap(),
        );

        WindowSystem {
            events_loop,
            window_event_channel: EventChannel::new(),
        }
    }

    fn run(&mut self, _world: &mut World, _resources: &mut Resources) {
        // Collect Winit events
        let mut events = Vec::with_capacity(100);
        self.events_loop.poll_events(|event| {
            events.push(event);
        });

        // Drain them into the EventChannel
        self.window_event_channel.drain_vec_write(&mut events);
    }
}
