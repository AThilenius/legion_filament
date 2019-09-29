use legion::prelude::*;
use std::{
  thread::sleep,
  time::{Duration, Instant},
};

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};

mod raw_bindings;
// mod rendering_system;
// mod system;
// mod window_system;

const MATERIAL_BYTES: &'static [u8] = include_bytes!("../materials/aiDefaultMat.filamat");

fn main() {
  let sdl = sdl2::init().unwrap();
  let video_subsystem = sdl.video().unwrap();
  let mut window = video_subsystem
    .window("Game", 960, 720)
    .allow_highdpi()
    .build()
    .unwrap();

  let (width, height) = window.drawable_size();
  println!("Rust side size: {}x{}", width, height);

  unsafe {
    raw_bindings::filament::run_test(
      window.raw() as *mut std::ffi::c_void,
      width,
      height,
      MATERIAL_BYTES.as_ptr() as *mut std::ffi::c_void,
      MATERIAL_BYTES.len() as u64,
    );
  }

  // let mut event_pump = sdl.event_pump().unwrap();
  // 'main: loop {
  //   let start_time = std::time::Instant::now();
  //   unsafe {
  //     raw_bindings::filament::draw();
  //   }

  //   // for event in event_pump.poll_iter() {
  //   //   match event {
  //   //     sdl2::event::Event::Quit { .. } => break 'main,
  //   //     _ => {}
  //   //   };
  //   // }

  //   // let title = format!("Render time: {}ms", start_time.elapsed().as_millis());
  //   // window.set_title(&title).unwrap();

  //   // std::thread::sleep(std::time::Duration::from_millis(16));
  // }

  // let mut resources = Resources::new();

  // let universe = Universe::new();
  // let mut world = universe.create_world();

  // let mut window_system = WindowSystem::new(&mut resources);
  // let mut rendering_system = RenderingSystem::new(&mut resources);

  // let mut exit = false;
  // let target_frame_time = Duration::from_secs(1) / 144;
  // let mut window_event_reader = resources.window_event_channel.register_reader();

  // while !exit {
  //   let frame_timer = Instant::now();

  //   window_system.run(&mut world, &mut resources);
  //   rendering_system.run(&mut world, &mut resources);

  //   for event in resources
  //     .window_event_channel
  //     .read(&mut window_event_reader)
  //   {
  //     match event {
  //       Event::WindowEvent {
  //         event: WindowEvent::CloseRequested,
  //         ..
  //       } => {
  //         exit = true;
  //       }
  //       _ => {}
  //     }
  //   }

  //   // Try to sleep long enough to hit the target frame time.
  //   if frame_timer.elapsed() < target_frame_time {
  //     sleep(target_frame_time - frame_timer.elapsed());
  //   }

  //   let title = format!(
  //     "Quad Example - Last frame: {:.2}ms",
  //     (frame_timer.elapsed().as_micros() as f64) / 1000_f64
  //   );
  //   resources.window.as_ref().unwrap().set_title(&title);
  // }
}
