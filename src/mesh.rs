#![allow(unused)]
use crate::{raw_bindings, MaterialInstance};
use nalgebra::Vector3;

#[derive(Copy)]
pub enum VertexAttribute {
  Position = 0,
  Tangents = 1,
  Color = 2,
  UV0 = 3,
  UV1 = 4,
  BoneIndices = 5,
  BoneWeights = 6,
  Custom0 = 8,
  Custom1 = 9,
  Custom2 = 10,
  Custom3 = 11,
  Custom4 = 12,
  Custom5 = 13,
  Custom6 = 14,
  Custom7 = 15,
}

// Tuple of (Filament AttributeType enum index, size in bytes).
pub struct VertexAttributeData(pub u32, pub u32);

// All supported Filament attribute type data.
pub struct VertexAttributeTypes;
impl VertexAttributeTypes {
  pub const BYTE: VertexAttributeData = VertexAttributeData(0, 1);
  pub const BYTE2: VertexAttributeData = VertexAttributeData(1, 2);
  pub const BYTE3: VertexAttributeData = VertexAttributeData(2, 3);
  pub const BYTE4: VertexAttributeData = VertexAttributeData(3, 4);
  pub const UBYTE: VertexAttributeData = VertexAttributeData(4, 1);
  pub const UBYTE2: VertexAttributeData = VertexAttributeData(5, 2);
  pub const UBYTE3: VertexAttributeData = VertexAttributeData(6, 3);
  pub const UBYTE4: VertexAttributeData = VertexAttributeData(7, 4);
  pub const SHORT: VertexAttributeData = VertexAttributeData(8, 2);
  pub const SHORT2: VertexAttributeData = VertexAttributeData(9, 4);
  pub const SHORT3: VertexAttributeData = VertexAttributeData(10, 6);
  pub const SHORT4: VertexAttributeData = VertexAttributeData(11, 8);
  pub const USHORT: VertexAttributeData = VertexAttributeData(12, 2);
  pub const USHORT2: VertexAttributeData = VertexAttributeData(13, 4);
  pub const USHORT3: VertexAttributeData = VertexAttributeData(14, 6);
  pub const USHORT4: VertexAttributeData = VertexAttributeData(15, 8);
  pub const INT: VertexAttributeData = VertexAttributeData(16, 4);
  pub const UINT: VertexAttributeData = VertexAttributeData(17, 4);
  pub const FLOAT: VertexAttributeData = VertexAttributeData(18, 4);
  pub const FLOAT2: VertexAttributeData = VertexAttributeData(19, 8);
  pub const FLOAT3: VertexAttributeData = VertexAttributeData(20, 12);
  pub const FLOAT4: VertexAttributeData = VertexAttributeData(21, 16);
  pub const HALF: VertexAttributeData = VertexAttributeData(22, 2);
  pub const HALF2: VertexAttributeData = VertexAttributeData(23, 4);
  pub const HALF3: VertexAttributeData = VertexAttributeData(24, 6);
  pub const HALF4: VertexAttributeData = VertexAttributeData(25, 8);
}

// Offsets and strides (assume all values are 1-Byte)
// ----------------------------------------
// | X  Y  Z  R  G  B |  X  Y  Z  R  G  B |
// ----------------------------------------
// Position { offset: 0, stride: sizeof(Vertex) = 6 }
// Color    { offset: 3, stride: sizeof(Vertex) = 6 }
pub struct VertexAttributeDefinition {
  attribute: VertexAttribute,
  data: VertexAttributeData,
  normalized: bool,
}

impl VertexAttributeDefinition {
  pub fn new(attribute: VertexAttribute, data: VertexAttributeData, normalized: bool) -> Self {
    VertexAttributeDefinition {
      attribute,
      data,
      normalized,
    }
  }
}

pub struct Mesh<T: Sized> {
  pub(self) vertex_attribute_definitions: Vec<VertexAttributeDefinition>,
  pub(self) vertex_data: Vec<T>,
  pub(self) index_data: Vec<u16>,
  pub(self) bounding_box: BoundingBox,

  // Handles to the Filament classes.
  vi_buffers: Option<raw_bindings::VIBuffers>,
}

#[repr(C)]
pub struct BoundingBox {
  pub center: Vector3<f32>,
  pub half_extent: Vector3<f32>,
}

impl<T: Sized> Mesh<T> {
  pub fn new(
    vertex_attribute_definitions: Vec<VertexAttributeDefinition>,
    vertex_data: Vec<T>,
    index_data: Vec<u16>,
    bounding_box: BoundingBox,
  ) -> Self {
    Mesh {
      vertex_attribute_definitions,
      vertex_data,
      index_data,
      bounding_box,
      vi_buffers: None,
    }
  }

  pub(crate) fn get_or_load_vi_buffers(&mut self, filament_rendering_system: &mut raw_bindings::RenderingSystem) -> raw_bindings::VIBuffers {
    if let Some(vi_buffers) = self.vi_buffers {
      return vi_buffers;
    }

    self.vi_buffers = unsafe {
      let mut offset = 0;
      let filament_vertex_defs: Vec<_> = self.vertex_attribute_definitions.iter().map(|vd| {
        let filament_vad = raw_bindings::VertexAttributeDefinition {
          vertex_attribute: vd.attribute as u8,
          vertex_attribute_type: vd.data.0 as u8,
          byte_offset: offset as u32,
          normalized: vd.normalized,
        };
        offset += vd.data.1;
        return filament_vad;
      }).collect();

      Some(
        filament_rendering_system.LoadVertexIndexData(
          filament_vertex_defs.as_ptr() as *mut raw_bindings::VertexAttributeDefinition,
          filament_vertex_defs.len() as u32,
          self.vertex_data.len() as u32,
          std::mem::size_of::<T>() as u8,
          self.vertex_data.as_ptr() as *mut std::ffi::c_void,
          self.index_data.as_ptr() as *mut std::ffi::c_void,
          self.index_data.len() as u32,
      )
      )
    };

    self.vi_buffers.unwrap()
  }
}
