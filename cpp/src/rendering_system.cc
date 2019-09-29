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

#include <sdl2/SDL.h>

#include "rendering_system.hpp"

using namespace filament;
using namespace utils;
using namespace math;
using namespace std;

void* getNativeWindow(void* handle);

// SDL_Window* createSDLwindow() {
//   const uint32_t windowFlags =
//       SDL_WINDOW_SHOWN | SDL_WINDOW_RESIZABLE | SDL_WINDOW_ALLOW_HIGHDPI;
//   SDL_Window* win =
//       SDL_CreateWindow("Hello World!", 100, 100, 1920, 1080, windowFlags);
//   if (win == nullptr) {
//     std::cout << "SDL_CreateWindow Error: " << SDL_GetError() << std::endl;
//     SDL_Quit();
//     return NULL;
//   }

//   return win;
// }

void run_test(void* window, uint32_t w, uint32_t h, void* mat,
              uint64_t mat_len) {
  // SDL_Window* window = createSDLwindow();
  // uint32_t w, h;
  // SDL_GL_GetDrawableSize((SDL_Window*)window, (int*)&w, (int*)&h);

  double aspect = ((double)w) / ((double)h);

  std::cout << "Creating window of size: " << w << "x" << h << std::endl;

  Engine* engine_ = Engine::create();
  SwapChain* swap_chain_ = engine_->createSwapChain(getNativeWindow(window));
  Renderer* renderer_ = engine_->createRenderer();

  Scene* scene_ = engine_->createScene();

  Camera* camera_ = engine_->createCamera();
  camera_->setProjection(Camera::Projection::ORTHO, -aspect, aspect, -1, 1, 0,
                         1);

  View* view_ = engine_->createView();
  view_->setScene(scene_);
  view_->setCamera(camera_);
  view_->setViewport({0, 0, w, h});
  view_->setClearColor({0.0, 0.0, 1.0, 1.0});
  view_->setClearTargets(true, true, false);

  float2 vertices[] = {{1, 0}, {-0.5, 0.866}, {-0.5, -0.866}};
  uint32_t colors[] = {0xffff0000, 0xff00ff00, 0xff0000ff};

  VertexBuffer* vertexBuffer =
      VertexBuffer::Builder()
          .vertexCount(3)
          .bufferCount(2)
          .attribute(VertexAttribute::POSITION, 0,
                     VertexBuffer::AttributeType::FLOAT2, 0, 8)
          .attribute(VertexAttribute::COLOR, 1,
                     VertexBuffer::AttributeType::UBYTE4, 0, 4)
          .normalized(VertexAttribute::COLOR)
          .build(*engine_);
  vertexBuffer->setBufferAt(
      *engine_, 0, VertexBuffer::BufferDescriptor(vertices, sizeof(vertices)));
  vertexBuffer->setBufferAt(
      *engine_, 1, VertexBuffer::BufferDescriptor(colors, sizeof(colors)));

  const static uint16_t indices[] = {0, 1, 2};
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
  utils::Entity renderable = EntityManager::get().create();
  scene_->addEntity(renderable);

  RenderableManager::Builder(1)
      .boundingBox({{-1, -1, -1}, {1, 1, 1}})
      .culling(false)
      .material(0, materialInstance)
      .geometry(0, RenderableManager::PrimitiveType::TRIANGLES, vertexBuffer,
                indexBuffer)
      .build(*engine_, renderable);

  float rot = 0.0;
  while (true) {
    TransformManager& tcm = engine_->getTransformManager();
    tcm.setTransform(tcm.getInstance(renderable),
                     mat4f::rotation(rot, float3{0, 0, 1}));
    rot += 0.05;

    if (renderer_->beginFrame(swap_chain_)) {
      renderer_->render(view_);
      renderer_->endFrame();
    }

    SDL_Event event;
    while (SDL_PollEvent(&event)) {
      switch (event.type) {
      case SDL_QUIT:
        Engine::destroy(engine_);
        return;
      }
    }

    SDL_Delay(16);
  }
}

// RenderingSystem::RenderingSystem(void* window_handle, uint32_t win_width,
//                                  uint32_t win_height, void* mat,
//                                  uint64_t mat_len) {
//   std::cout << "Creating window of size: " << win_width << "x" << win_height
//             << std::endl;
//   this->engine_ = Engine::create();
//   this->swap_chain_ = this->engine_->createSwapChain(window_handle);
//   this->renderer_ = this->engine_->createRenderer();

//   this->scene_ = this->engine_->createScene();

//   this->camera_ = this->engine_->createCamera();
//   // this->camera_->setProjection(75.0, (double)win_width /
//   (double)win_height,
//   //                              0.1, 50.0, Camera::Fov::VERTICAL);
//   double aspect = double(win_width) / double(win_height);
//   this->camera_->setProjection(Camera::Projection::ORTHO, -aspect, aspect,
//   -1,
//                                1, 0, 1);

//   this->view_ = this->engine_->createView();
//   this->view_->setScene(this->scene_);
//   this->view_->setCamera(this->camera_);
//   this->view_->setViewport({0, 0, win_width, win_height});
//   this->view_->setVisibleLayers(0x4, 0x4);
//   // this->view_->setClearColor({0.0, 0.0, 1.0, 1.0});
//   this->view_->setClearTargets(false, true, false);

//   // TEST

//   float2 vertices[] = {{1, 0}, {-0.5, 0.866}, {-0.5, -0.866}};
//   uint32_t colors[] = {0xffff0000, 0xff00ff00, 0xff0000ff};

//   VertexBuffer* vertexBuffer =
//       VertexBuffer::Builder()
//           .vertexCount(3)
//           .bufferCount(2)
//           .attribute(VertexAttribute::POSITION, 0,
//                      VertexBuffer::AttributeType::FLOAT2, 0, 8)
//           .attribute(VertexAttribute::COLOR, 1,
//                      VertexBuffer::AttributeType::UBYTE4, 0, 4)
//           .normalized(VertexAttribute::COLOR)
//           .build(*this->engine_);
//   vertexBuffer->setBufferAt(
//       *this->engine_, 0,
//       VertexBuffer::BufferDescriptor(vertices, sizeof(vertices)));
//   vertexBuffer->setBufferAt(
//       *this->engine_, 1,
//       VertexBuffer::BufferDescriptor(colors, sizeof(colors)));

//   const static uint16_t indices[] = {0, 1, 2};
//   IndexBuffer* indexBuffer = IndexBuffer::Builder()
//                                  .indexCount(3)
//                                  .bufferType(IndexBuffer::IndexType::USHORT)
//                                  .build(*this->engine_);
//   indexBuffer->setBuffer(
//       *this->engine_, IndexBuffer::BufferDescriptor(indices,
//       sizeof(indices)));

//   Material* material =
//       Material::Builder().package(mat, mat_len).build(*this->engine_);
//   MaterialInstance* materialInstance = material->getDefaultInstance();

//   // build a quad
//   utils::Entity renderable = EntityManager::get().create();
//   this->scene_->addEntity(renderable);

//   RenderableManager::Builder(1)
//       .boundingBox({{-1, -1, -1}, {1, 1, 1}})
//       .culling(false)
//       .material(0, materialInstance)
//       .geometry(0, RenderableManager::PrimitiveType::TRIANGLES, vertexBuffer,
//                 indexBuffer)
//       .build(*this->engine_, renderable);

//   // TransformManager& tcm = this->engine_->getTransformManager();
//   // this->camera_->setModelMatrix(mat4f::translation(float3{0, 0, 0}));
//   // tcm.setTransform(tcm.getInstance(renderable),
//   //                  mat4f::translation(float3{0, 0, 0}));
// }

// void RenderingSystem::Destroy() {
//   std::cout << "Shutting down engine" << std::endl;
//   if (this->engine_ != nullptr) {
//     Engine::destroy(this->engine_);
//     this->engine_ = nullptr;
//   }
// }

// void RenderingSystem::Render() const {
//   if (this->renderer_->beginFrame(this->swap_chain_)) {
//     this->renderer_->render(this->view_);
//     this->renderer_->endFrame();
//   }
// }

// extern "C" void
// init(void* window_handle) {
//   Engine* engine = Engine::create();
//   SwapChain* swapChain = engine->createSwapChain(window_handle);
//   Renderer* renderer = engine->createRenderer();

//   Camera* camera = engine->createCamera();
//   View* view = engine->createView();
//   view->setClearTargets(true, true, false);
//   view->setRenderTarget(View::TargetBufferFlags::DEPTH_AND_STENCIL);
//   view->setShadowsEnabled(false);
//   view->setClearColor({0.0, 0.25, 0.5, 1.0});
//   Scene* scene = engine->createScene();

//   // view->setCamera(camera);
//   // view->setScene(scene);

//   while (!renderer->beginFrame(swapChain)) {
//   }

//   renderer->render(view);
//   renderer->endFrame();
// }
