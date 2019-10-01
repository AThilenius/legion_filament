#ifndef RENDERING_SYSTEM
#define RENDERING_SYSTEM

#include <stdint.h>

#include "opaque_types.hpp"

using namespace filament;

class VIBuffers {
public:
  VertexBuffer* vertex_buffer;
  IndexBuffer* index_buffer;
};

class VertexAttributeDefinition {
public:
  uint8_t vertex_attribute;
  uint8_t vertex_attribute_type;
  uint32_t byte_offset;
  bool normalized;
};

/// <div rustbindgen opaque></div>
class RenderingSystem {
public:
  explicit RenderingSystem(void* window, uint32_t width, uint32_t height,
                           void* mat, uint64_t mat_len);
  void Destroy();

  VIBuffers LoadVertexIndexData(VertexAttributeDefinition* va_data,
                                uint32_t va_count, uint32_t vertex_count,
                                uint8_t vertex_size_bytes, void* vertex_data,
                                void* index_data, uint32_t index_count);

  Material* LoadMaterial(void* data, uint32_t data_size_bytes);

  MaterialInstance* CreateMaterialInstance(Material* material);

  uint32_t CreateEntity(float* bb, bool culled,
                        MaterialInstance* material_instance,
                        VIBuffers vi_buffers);

  void Render() const;

private:
  Engine* engine_;
  SwapChain* swap_chain_;
  Renderer* renderer_;
  View* view_;
  Scene* scene_;
  Camera* camera_;
};

#endif
