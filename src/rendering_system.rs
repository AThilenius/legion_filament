use crate::raw_bindings;
use crate::{Resources, System};
use legion::world::World;
use winit::Window;

const MATERIAL_BYTES: &'static [u8] = include_bytes!("../materials/bin/color_unlit.filamat");

pub struct RenderingSystem {
  filament_rendering_system: raw_bindings::RenderingSystem,
}

impl System for RenderingSystem {
  fn new(resources: &mut Resources) -> Self {
    let window = resources.window.as_ref().unwrap();
    let hidpi = window.get_hidpi_factor();
    let (width, height) = window.get_inner_size().unwrap().to_physical(hidpi).into();

    RenderingSystem {
      filament_rendering_system: unsafe {
        raw_bindings::RenderingSystem::new(
          get_active_surface(window),
          width,
          height,
          MATERIAL_BYTES.as_ptr() as *mut std::ffi::c_void,
          MATERIAL_BYTES.len() as u64,
        )
      },
    }
  }

  fn run(&mut self, _world: &mut World, _resources: &mut Resources) {
    unsafe {
      self.filament_rendering_system.Render();
    }
  }
}

impl Drop for RenderingSystem {
  fn drop(&mut self) {
    unsafe {
      self.filament_rendering_system.Destroy();
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
