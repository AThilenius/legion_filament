#include <filament/Engine.h>
#include <filament/FilamentAPI.h>
#include <filament/LightManager.h>
#include <filament/RenderableManager.h>
#include <filament/Scene.h>
#include <filament/TransformManager.h>
#include <filament/View.h>
#include <math/norm.h>
#include <thread>
#include <utils/EntityManager.h>

#include "rendering_system.hpp"

using namespace filament;
using namespace utils;
using namespace math;
using namespace std;

struct Vertex {
  float2 position;
  uint32_t color;
};

utils::Entity renderable;
// float2 vertices[] = {{1, 0}, {-0.5, 0.866}, {-0.5, -0.866}};
// uint32_t colors[] = {0xffff0000, 0xff00ff00, 0xff0000ff};
Vertex verticies[] = {
    {{1, 0}, 0xFFFF0000},
    {{-0.5, 0.866}, 0xff00ff00},
    {{-0.5, -0.866}, 0xff0000ff},
};
const static uint16_t indices[] = {0, 1, 2};

RenderingSystem::RenderingSystem(void* window, uint32_t width, uint32_t height,
                                 void* mat, uint64_t mat_len) {

  double aspect = ((double)width) / ((double)height);

  std::cout << "Creating window of size: " << width << "x" << height
            << std::endl;

  engine_ = Engine::create();
  swap_chain_ = engine_->createSwapChain(window);
  renderer_ = engine_->createRenderer();

  scene_ = engine_->createScene();

  camera_ = engine_->createCamera();
  camera_->setProjection(Camera::Projection::ORTHO, -aspect, aspect, -1, 1, 0,
                         1);

  view_ = engine_->createView();
  view_->setScene(scene_);
  view_->setCamera(camera_);
  view_->setViewport({0, 0, width, height});
  view_->setClearColor({0.0, 0.0, 1.0, 1.0});
  view_->setClearTargets(true, true, false);

  // TEST

  VertexBuffer* vertexBuffer =
      VertexBuffer::Builder()
          .vertexCount(3)
          .bufferCount(1)
          .attribute(VertexAttribute::POSITION, 0,
                     VertexBuffer::AttributeType::FLOAT2, 0, 12)
          .attribute(VertexAttribute::COLOR, 0,
                     VertexBuffer::AttributeType::UBYTE4, 8, 12)
          .normalized(VertexAttribute::COLOR)
          .build(*engine_);
  vertexBuffer->setBufferAt(
      *engine_, 0,
      VertexBuffer::BufferDescriptor(verticies, sizeof(verticies)));

  IndexBuffer* indexBuffer = IndexBuffer::Builder()
                                 .indexCount(3)
                                 .bufferType(IndexBuffer::IndexType::USHORT)
                                 .build(*engine_);
  indexBuffer->setBuffer(
      *engine_, IndexBuffer::BufferDescriptor(indices, sizeof(indices)));

  Material* material =
      Material::Builder().package(mat, mat_len).build(*engine_);
  MaterialInstance* materialInstance = material->getDefaultInstance();

  // build a quad
  renderable = EntityManager::get().create();
  scene_->addEntity(renderable);

  RenderableManager::Builder(1)
      .boundingBox({{-1, -1, -1}, {1, 1, 1}})
      .culling(false)
      .material(0, materialInstance)
      .geometry(0, RenderableManager::PrimitiveType::TRIANGLES, vertexBuffer,
                indexBuffer)
      .build(*engine_, renderable);
}

void RenderingSystem::Destroy() {
  std::cout << "Shutting down engine" << std::endl;
  if (engine_ != nullptr) {
    Engine::destroy(engine_);
    engine_ = nullptr;
  }
}

VIBuffers RenderingSystem::LoadVertexIndexData(
    VertexAttributeDefinition* va_data, uint32_t va_count,
    uint32_t vertex_count, uint8_t vertex_size_bytes, void* vertex_data,
    void* index_data, uint32_t index_count) {
  VertexBuffer::Builder& vertexBufferBuilder =
      VertexBuffer::Builder().vertexCount(vertex_count).bufferCount(1);

  // Set attributes
  for (int i = 0; i < va_count; i++) {
    VertexAttributeDefinition vad = va_data[i];
    vertexBufferBuilder.attribute(
        (VertexAttribute)vad.vertex_attribute, 0,
        (VertexBuffer::AttributeType)vad.vertex_attribute_type, vad.byte_offset,
        vertex_size_bytes);
  }

  // Set normalized
  for (int i = 0; i < va_count; i++) {
    VertexAttributeDefinition vad = va_data[i];
    if (!vad.normalized) {
      continue;
    }
    vertexBufferBuilder.normalized((VertexAttribute)vad.vertex_attribute, true);
  }

  // Builder VertexBuffer and set data
  VertexBuffer* vertexBuffer = vertexBufferBuilder.build(*engine_);
  vertexBuffer->setBufferAt(*engine_, 0,
                            VertexBuffer::BufferDescriptor(
                                vertex_data, vertex_size_bytes * vertex_count));

  // Build the index buffer and set data
  IndexBuffer* indexBuffer = IndexBuffer::Builder()
                                 .indexCount(index_count)
                                 .bufferType(IndexBuffer::IndexType::USHORT)
                                 .build(*engine_);
  indexBuffer->setBuffer(
      *engine_, IndexBuffer::BufferDescriptor(index_data, 2 * index_count));

  return (VIBuffers){.vertex_buffer = vertexBuffer,
                     .index_buffer = indexBuffer};
}

Material* RenderingSystem::LoadMaterial(void* data, uint32_t data_size_bytes) {
  return Material::Builder().package(data, data_size_bytes).build(*engine_);
}

MaterialInstance* RenderingSystem::CreateMaterialInstance(Material* material) {
  return material->createInstance();
}

uint32_t RenderingSystem::CreateEntity(float* bb, bool culled,
                                       MaterialInstance* material_instance,
                                       VIBuffers vi_buffers) {
  utils::Entity entity = EntityManager::get().create();
  Box* filament_bb = (Box*)bb;
  RenderableManager::Builder(1)
      .boundingBox(*filament_bb)
      .culling(culled)
      .material(0, material_instance)
      .geometry(0, RenderableManager::PrimitiveType::TRIANGLES,
                vi_buffers.vertex_buffer, vi_buffers.index_buffer)
      .build(*engine_, entity);
  return entity.getId();
}

float rot = 0.0;
void RenderingSystem::Render() const {
  TransformManager& tcm = engine_->getTransformManager();
  tcm.setTransform(tcm.getInstance(renderable),
                   mat4f::rotation(rot, float3{0, 0, 1}));
  rot += 0.05;

  if (renderer_->beginFrame(swap_chain_)) {
    renderer_->render(view_);
    renderer_->endFrame();
  }
}
