use crate::{raw_bindings, Mesh, MaterialInstance};

pub struct Renderable {
  vi_buffers: raw_bindings::VIBuffers,
  material_instance: raw_bindings::filament_MaterialInstance,
  filament_entity: Option<u32>,
}

impl Renderable {
  pub fn new(vi_buffers: raw_bindings::VIBuffers, material_instance: raw_bindings::filament_MaterialInstance) -> Self {
    Renderable {
      vi_buffers,
      material_instance,
      filament_entity: None,
    }
  }

  pub fn get_or_create_entity(&mut self, filament_rendering_system: &mut raw_bindings::RenderingSystem) -> u32 {
    if let Some(id) = self.filament_entity {
      return id;
    }

    self.filament_entity = Some(unsafe {
      filament_rendering_system.CreateEntity(
        self.mesh.bounding_box.as_ptr() as *mut f32,
        true,
        self.material_instance,
        self.mesh.get_or_load_vi_buffers(filament_rendering_system)
      )
    });

    self.filament_entity.unwrap()
  }
}
