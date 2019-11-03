use crate::{camera::MainCamera, mesh_storage::*, ThreadLocalSystem};
use filament::prelude::*;
use legion::prelude::*;
use legion_transform::prelude::*;
use nalgebra::Vector3;
use winit::Window;

const MATERIAL_BYTES: &'static [u8] = include_bytes!("../materials/bin/color_unlit.filamat");

struct FEntityLink(filament::prelude::Entity);

pub struct RenderingSystem {
    pub mesh_storage: MeshStorage,
    engine: Engine,
    swap_chain: SwapChain,
    renderer: Renderer,
    view: View,
    scene: Scene,
    camera: Camera,

    // DEBUG
    material: Material,
}

impl ThreadLocalSystem for RenderingSystem {
    fn new(world: &mut World) -> Self {
        let window = world.resources.get::<Window>().unwrap();
        let hidpi = window.get_hidpi_factor();
        let (width, height) = window.get_inner_size().unwrap().to_physical(hidpi).into();
        let aspect = width as f64 / height as f64;

        let mut engine = Engine::new(Backend::Default);
        let swap_chain = engine.create_swap_chain(get_active_surface(&window));
        let renderer = engine.create_renderer();
        let mut view = engine.create_view();
        let scene = engine.create_scene();

        // Make the camera
        let mut camera = engine.create_camera();
        camera.set_projection_fov(60.0, aspect, 0.1, 10000.0, Fov::Vertical);

        // Setup the view
        view.set_scene(&scene);
        view.set_camera(&camera);
        view.set_viewport(0, 0, width, height);
        view.set_clear_color(0.0, 0.0, 1.0, 1.0);
        view.set_clear_targets(true, true, false);

        let material = engine.create_material(MATERIAL_BYTES);

        RenderingSystem {
            mesh_storage: MeshStorage::new(engine.clone()),
            engine,
            swap_chain,
            renderer,
            view,
            scene,
            camera,
            material,
        }
    }

    fn run(&mut self, world: &mut World) {
        let mut missing_entity_handle = <Read<MeshHandle>>::query()
            .filter(!component::<FEntityLink>() & component::<LocalToWorld>());

        // Collect beforehand
        let missing_entity_handle: Vec<_> = missing_entity_handle
            .iter_entities(world)
            .map(|(e, m)| (e, *m))
            .collect();

        for (entity, mesh_handle) in missing_entity_handle {
            let f_entity = EntityManager::get().create();
            self.scene.add_entity(f_entity);

            let (vertex_buffer, index_buffer) = self.mesh_storage.get_buffers(mesh_handle);

            // Build and attach a renderable to the entity.
            RenderableManager::builder(1)
                .bounding_box(BoundingBox {
                    center: Vector3::new(-1., -1., -1.),
                    half_extent: Vector3::new(1., 1., 1.),
                })
                .culling(false)
                .material(0, &self.material.create_instance())
                .geometry(0, PrimitiveType::Triangles, &vertex_buffer, &index_buffer)
                .build(&self.engine, f_entity);

            // Add the FEntityLink to the Legion entity
            world.add_component(entity, FEntityLink(f_entity));
        }

        // Update the transform of all entities with links and LocalToWorld.
        let mut linked_entities = <(Read<FEntityLink>, Read<LocalToWorld>)>::query();

        for (f_entity_link, local_to_world) in linked_entities.iter(world) {
            self.engine
                .get_transform_manager()
                .set_transform(f_entity_link.0, local_to_world.0);
        }

        // Update the camera with the first MainCamera.
        let mut main_camera_query = <(Read<MainCamera>, Read<LocalToWorld>)>::query();
        let mut first_camera = true;
        for (_main_camera, local_to_world) in main_camera_query.iter(world) {
            if !first_camera {
                warn!("More than one MainCamera component found");
                break;
            }

            self.camera.set_model_matrix(local_to_world.0);

            first_camera = false;
        }

        // Then try to begin another frame (returns false if we need to skip a frame).
        if self.renderer.begin_frame(&self.swap_chain) {
            self.renderer.render(&self.view);
            self.renderer.end_frame();
        }
    }
}

#[cfg(target_os = "macos")]
fn get_active_surface(window: &Window) -> *mut std::ffi::c_void {
    use winit::os::macos::WindowExt;
    window.get_nsview()
}

#[cfg(target_os = "windows")]
fn get_active_surface(window: &Window) -> *mut std::ffi::c_void {
    use winit::os::windows::WindowExt;
    window.get_hwnd()
}
