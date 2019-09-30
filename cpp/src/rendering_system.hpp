#ifndef RENDERING_SYSTEM
#define RENDERING_SYSTEM

#include <stdint.h>

#include "opaque_types.hpp"

using namespace filament;

/// <div rustbindgen opaque></div>
class RenderingSystem {
public:
  explicit RenderingSystem(void* window, uint32_t width, uint32_t height,
                           void* mat, uint64_t mat_len);
  void Destroy();

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
