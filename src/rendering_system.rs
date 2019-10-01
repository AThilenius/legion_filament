use crate::{Resources, System};
use filament::prelude::*;
use legion::world::World;
use winit::Window;

const MATERIAL_BYTES: &'static [u8] = include_bytes!("../materials/bin/color_unlit.filamat");

pub struct RenderingSystem {
  engine: Engine,
  swap_chain: SwapChain,
  renderer: Renderer,
  view: View,
  scene: Scene,
  camera: Camera,
}

impl System for RenderingSystem {
  fn new(resources: &mut Resources) -> Self {
    let window = resources.window.as_ref().unwrap();
    let hidpi = window.get_hidpi_factor();
    let (width, height) = window.get_inner_size().unwrap().to_physical(hidpi).into();
    let aspect = width as f64 / height as f64;

    let mut engine = Engine::new(Backend::Default);
    let swap_chain = engine.create_swap_chain(get_active_surface(&window));
    let renderer = engine.create_renderer();
    let mut view = engine.create_view();
    let mut scene = engine.create_scene();

    // Make the camera
    let mut camera = engine.create_camera();
    camera.set_projection_fov(60.0, aspect, 0.1, 10000.0, Fov::Vertical);

    // Setup the view
    view.set_scene(&scene);
    view.set_camera(&camera);
    view.set_viewport(0, 0, width, height);
    view.set_clear_targets(true, true, false);

    RenderingSystem {
      engine,
      swap_chain,
      renderer,
      view,
      scene,
      camera,
    }
  }

  fn run(&mut self, _world: &mut World, _resources: &mut Resources) {
    // Then try to begin another frame (returns false if we need to skip a frame).
    if self.renderer.begin_frame(&self.swap_chain) {
      self.renderer.render(&self.view);
      self.renderer.end_frame();
    }
  }
}

#[cfg(target_os = "macos")]
fn get_active_surface(window: &Window) -> *mut std::ffi::c_void {
  use winit::os::macos::WindowExt;
  window.get_nsview()
}

#[cfg(target_os = "windows")]
fn get_active_surface(window: &Window) -> *mut std::ffi::c_void {
  use winit::os::windows::WindowExt;
  window.get_hwnd()
}
