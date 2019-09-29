use crate::raw_bindings::filament;
use crate::{Resources, System};
use legion::world::World;
use winit::{
  event_loop::{ControlFlow, EventLoop},
  platform::desktop::EventLoopExtDesktop,
  window::{Window, WindowBuilder},
};

const MATERIAL_BYTES: &'static [u8] = include_bytes!("../materials/aiDefaultMat.filamat");

pub struct RenderingSystem {
  filament_rendering_system: filament::RenderingSystem,
}

impl System for RenderingSystem {
  fn new(resources: &mut Resources) -> Self {
    let window = &resources
      .window
      .as_ref()
      .expect("A WindowSystem must be created before a RenderSystem");
    let window_handle = get_active_surface(window);
    let (width, height) = window.inner_size().into();
    RenderingSystem {
      filament_rendering_system: unsafe {
        filament::RenderingSystem::new(
          window_handle,
          width,
          height,
          MATERIAL_BYTES.as_ptr() as *mut std::ffi::c_void,
          MATERIAL_BYTES.len() as u64,
        )
      },
    }
  }

  fn run(&mut self, world: &mut World, resources: &mut Resources) {
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
  use winit::platform::macos::WindowExtMacOS;
  window.nsview()
}

#[cfg(target_os = "windows")]
fn get_active_surface(window: &Window) -> *mut std::ffi::c_void {
  use winit::platform::windows::WindowExtWindows;
  window.hwnd()
}
