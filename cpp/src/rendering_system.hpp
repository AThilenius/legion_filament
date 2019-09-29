#ifndef RENDERING_SYSTEM
#define RENDERING_SYSTEM

#include <stdint.h>

#include "opaque_types.hpp"

using namespace filament;

void run_test(void* sdl_window_handle, uint32_t w, uint32_t h, void* mat,
              uint64_t mat_len);
// void draw();

// /// <div rustbindgen opaque></div>
// class RenderingSystem {
// public:
//   explicit RenderingSystem(void* window_handle, uint32_t win_width,
//                            uint32_t win_height, void* mat, uint64_t mat_len);
//   void Destroy();

//   void RenderingSystem::Render() const;

// private:
//   Engine* engine_;
//   SwapChain* swap_chain_;
//   Renderer* renderer_;
//   View* view_;
//   Scene* scene_;
//   Camera* camera_;
// };

#endif
