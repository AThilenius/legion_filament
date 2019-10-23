#![allow(unused)]
use filament::prelude::*;
pub use filament::prelude::{AttributeType, VertexAttribute};
use legion::prelude::*;
use nalgebra::{Vector3, Vector4};
use std::{any::Any, collections::HashMap};

// Offsets and strides (assume all values are 1-Byte)
// ----------------------------------------
// | X  Y  Z  R  G  B |  X  Y  Z  R  G  B |
// ----------------------------------------
// Position { offset: 0, stride: sizeof(Vertex) = 6 }
// Color    { offset: 3, stride: sizeof(Vertex) = 6 }
pub struct VertexAttributeDefinition {
    pub(self) attribute: VertexAttribute,
    pub(self) attribute_type: AttributeType,
    pub(self) attribute_size: u32,
    pub(self) normalized: bool,
}

impl VertexAttributeDefinition {
    pub fn new(
        attribute: VertexAttribute,
        attribute_type: AttributeType,
        normalized: bool,
    ) -> Self {
        VertexAttributeDefinition {
            attribute,
            attribute_type,
            normalized,
            attribute_size: match attribute_type {
                AttributeType::Byte => 1,
                AttributeType::Byte2 => 2,
                AttributeType::Byte3 => 3,
                AttributeType::Byte4 => 4,
                AttributeType::Ubyte => 1,
                AttributeType::Ubyte2 => 2,
                AttributeType::Ubyte3 => 3,
                AttributeType::Ubyte4 => 4,
                AttributeType::Short => 2,
                AttributeType::Short2 => 4,
                AttributeType::Short3 => 6,
                AttributeType::Short4 => 8,
                AttributeType::Ushort => 2,
                AttributeType::Ushort2 => 4,
                AttributeType::Ushort3 => 6,
                AttributeType::Ushort4 => 8,
                AttributeType::Int => 4,
                AttributeType::Uint => 4,
                AttributeType::Float => 4,
                AttributeType::Float2 => 8,
                AttributeType::Float3 => 12,
                AttributeType::Float4 => 16,
                AttributeType::Half => 2,
                AttributeType::Half2 => 4,
                AttributeType::Half3 => 6,
                AttributeType::Half4 => 8,
            },
        }
    }
}

pub trait VertexStruct {
    fn attribute_definitions() -> Vec<VertexAttributeDefinition>;
}

pub struct MeshStorage {
    asset_storage: Vec<Box<dyn Any>>,
    buffer_storage: Vec<(VertexBuffer, IndexBuffer)>,
    engine: Engine,
}

struct MeshData<T: Sized> {
    vertices: Vec<T>,
    indices: Vec<u16>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MeshHandle(pub usize);

impl MeshStorage {
    pub fn new(engine: Engine) -> Self {
        Self {
            asset_storage: Vec::new(),
            buffer_storage: Vec::new(),
            engine,
        }
    }

    pub fn add<T>(&mut self, vertices: Vec<T>, indices: Vec<u16>) -> MeshHandle
    where
        T: VertexStruct + Sized + Send + Sync + 'static,
    {
        let index = self.asset_storage.len();
        let (vertex_count, index_count) = (vertices.len(), indices.len());
        self.asset_storage
            .push(Box::new(MeshData { vertices, indices }) as Box<dyn Any>);

        let mut vertex_builder = self
            .engine
            .create_vertex_buffer_builder()
            .buffer_count(1)
            .vertex_count(vertex_count as u32);

        let size_of = std::mem::size_of::<T>();
        let mut offset = 0_u32;

        for attribute_definition in T::attribute_definitions() {
            vertex_builder = vertex_builder.attribute(
                attribute_definition.attribute,
                0,
                attribute_definition.attribute_type,
                offset,
                size_of as u8,
            );
            offset += attribute_definition.attribute_size;

            if attribute_definition.normalized {
                vertex_builder = vertex_builder.normalized(attribute_definition.attribute, true);
            }
        }

        let mut vertex_buffer = vertex_builder.build();

        vertex_buffer.set_buffer_at(
            0,
            self.asset_storage[index]
                .downcast_ref::<MeshData<T>>()
                .unwrap()
                .vertices
                .as_slice(),
        );

        let mut index_buffer = self
            .engine
            .create_index_buffer_builder()
            .index_count(index_count as u32)
            .buffer_type(IndexType::Ushort)
            .build();

        index_buffer.set_buffer(
            self.asset_storage[index]
                .downcast_ref::<MeshData<T>>()
                .unwrap()
                .indices
                .as_slice(),
        );

        self.buffer_storage.push((vertex_buffer, index_buffer));

        MeshHandle(index)
    }

    pub fn get_buffers(&self, handle: MeshHandle) -> (VertexBuffer, IndexBuffer) {
        self.buffer_storage[handle.0].clone()
    }
}
