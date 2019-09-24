#[macro_use]
extern crate cpp;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    platform::windows::WindowExtWindows,
};

cpp! {{
    #include <filament/Camera.h>
    #include <filament/Color.h>
    #include <filament/Engine.h>
    #include <filament/FilamentAPI.h>
    #include <filament/Renderer.h>
    #include <filament/Scene.h>
    #include <filament/View.h>
    #include <math/vec4.h>

    using namespace filament;
    using namespace filament::math;
}}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let handle = unsafe { window.hwnd() };

    let x: i32 = cpp!(unsafe [handle as "void*"] -> i32 as "int32_t" {
        Engine *engine = Engine::create();
        SwapChain* swapChain = engine->createSwapChain(handle);
        Renderer* renderer = engine->createRenderer();

        Camera* camera = engine->createCamera();
        View* view = engine->createView();
        view->setClearTargets(true, true, false);
        view->setRenderTarget(View::TargetBufferFlags::DEPTH_AND_STENCIL);
        view->setShadowsEnabled(false);
        view->setClearColor({0.0, 0.25, 0.5, 1.0});
        Scene* scene = engine->createScene();

        // view->setCamera(camera);
        // view->setScene(scene);

        while (!renderer->beginFrame(swapChain)) {}
        renderer->render(view);
        renderer->endFrame();

        // engine->destroy(&engine);
        return 0;
    });

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == window.id() => *control_flow = ControlFlow::Exit,
        _ => *control_flow = ControlFlow::Wait,
    });
}
