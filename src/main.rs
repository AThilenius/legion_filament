use legion::prelude::*;
use std::{
  thread::sleep,
  time::{Duration, Instant},
};
use winit::{Event, WindowEvent};

mod raw_bindings;
mod rendering_system;
mod system;
mod window_system;

use rendering_system::RenderingSystem;
use system::{Resources, System};
use window_system::WindowSystem;

fn main() {
  let mut resources = Resources::new();

  let universe = Universe::new();
  let mut world = universe.create_world();

  let mut window_system = WindowSystem::new(&mut resources);
  let mut rendering_system = RenderingSystem::new(&mut resources);

  let mut exit = false;
  let target_frame_time = Duration::from_secs(1) / 144;
  let mut window_event_reader = resources.window_event_channel.register_reader();

  while !exit {
    let frame_timer = Instant::now();

    window_system.run(&mut world, &mut resources);
    rendering_system.run(&mut world, &mut resources);

    for event in resources
      .window_event_channel
      .read(&mut window_event_reader)
    {
      match event {
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => {
          exit = true;
        }
        _ => {}
      }
    }

    let title = format!(
      "Quad Example - Last frame: {:.2}ms",
      (frame_timer.elapsed().as_micros() as f64) / 1000_f64
    );
    resources.window.as_ref().unwrap().set_title(&title);

    // Try to sleep long enough to hit the target frame time.
    if frame_timer.elapsed() < target_frame_time {
      sleep(target_frame_time - frame_timer.elapsed());
    }
  }
}
