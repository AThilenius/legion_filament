mod camera;
mod input_handler;
mod input_system;
mod mesh_storage;
mod orbit_system;
mod rendering_system;
mod thread_local_system;
mod window_system;

#[macro_use]
extern crate log;

// #[macro_use]
// extern crate serde_derive;

use input_system::InputSystem;
use legion::prelude::*;
use legion_transform::prelude::*;
use mesh_storage::*;
use nalgebra::Vector2;
use orbit_system::{OrbitControl, OrbitControlsSystem};
use rendering_system::RenderingSystem;
use shrev::EventChannel;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use thread_local_system::ThreadLocalSystem;
use window_system::WindowSystem;
use winit::{Event, Window, WindowEvent};

#[repr(C)]
struct Vertex {
    pub position: Vector2<f32>,
    pub color: u32,
}

impl VertexStruct for Vertex {
    fn attribute_definitions() -> Vec<VertexAttributeDefinition> {
        vec![
            VertexAttributeDefinition::new(VertexAttribute::Position, AttributeType::Float2, false),
            VertexAttributeDefinition::new(VertexAttribute::Color, AttributeType::Ubyte4, true),
        ]
    }
}

fn main() {
    let universe = Universe::new();
    let mut world = universe.create_world();
    let mut schedulables = Vec::new();

    let mut window_system = WindowSystem::new(&mut world);
    let mut rendering_system = RenderingSystem::new(&mut world);
    schedulables.push(InputSystem::default().build(&mut world));
    schedulables.extend(TransformSystemBundle::default().build());
    schedulables.push(OrbitControlsSystem::default().build());

    let mesh_handle = rendering_system.mesh_storage.add(
        vec![
            Vertex {
                position: Vector2::new(1.0, 0.0),
                color: 0xffff0000,
            },
            Vertex {
                position: Vector2::new(-0.5, 0.866),
                color: 0xff00ff00,
            },
            Vertex {
                position: Vector2::new(-0.5, -0.866),
                color: 0xff0000ff,
            },
        ],
        vec![0, 1, 2],
    );

    let triangle_entity = *world
        .insert(
            (),
            vec![(
                LocalToWorld::identity(),
                Translation::new(0.0, 0.0, 0.0),
                mesh_handle,
            )],
        )
        .first()
        .unwrap();

    // Add the main camera, with an orbital control
    world.insert(
        (),
        vec![(
            LocalToWorld::identity(),
            camera::MainCamera::default(),
            OrbitControl::new(triangle_entity),
        )],
    );

    let mut exit = false;
    let target_frame_time = Duration::from_secs(1) / 144;
    let mut window_event_reader = world
        .resources
        .get_mut::<EventChannel<Event>>()
        .unwrap()
        .register_reader();

    // Create a 'prefab' for the cube with physics (prefab can just be a closure?)
    // MeshStorage: ContentHash -> { Vertices, Indices, VertexBuffer, IndexBuffer }
    // ECS Entity: FilamentEntity with: Renderable { MaterialInstance, VertexBuffer, IndexBuffer }

    while !exit {
        let frame_timer = Instant::now();

        window_system.run(&mut world);

        for system in schedulables.iter() {
            system.run(&world);
            system.command_buffer_mut().write(&mut world);
        }

        rendering_system.run(&mut world);

        for event in world
            .resources
            .get::<EventChannel<Event>>()
            .unwrap()
            .read(&mut window_event_reader)
        {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    exit = true;
                }
                _ => {}
            }
        }

        let title = format!(
            "Quad Example - Last frame: {:.2}ms",
            (frame_timer.elapsed().as_micros() as f64) / 1000_f64
        );
        world
            .resources
            .get_mut::<Window>()
            .unwrap()
            .set_title(&title);

        // Try to sleep long enough to hit the target frame time.
        if frame_timer.elapsed() < target_frame_time {
            sleep(target_frame_time - frame_timer.elapsed());
        }
    }
}
