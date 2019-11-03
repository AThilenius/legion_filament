use crate::input_handler::InputHandler;
use legion::prelude::*;
use legion_transform::prelude::*;
use nalgebra::{Matrix4, Rotation3, Translation3, UnitQuaternion, Vector3};
use shrev::EventChannel;
use winit::{DeviceEvent, ElementState, Event, MouseButton};

/// Attach as a child of another entity (with a LocalToWorld) to follow that entity.
pub struct OrbitControl {
    // The entity to follow (translation only).
    pub follow_target: Entity,
    // Distance from target.
    pub radius: f32,
    // Rotation around the Y axis.
    pub phi: f32,
    // Angle 'upward' of the boom.
    pub theta: f32,
    // Max radius (of the boom).
    pub max_radius: f32,
    // Min radius (of the boom).
    pub min_radius: f32,
}

impl OrbitControl {
    pub fn new(follow_target: Entity) -> Self {
        Self {
            follow_target,
            radius: 5.0,
            phi: 0.0,
            theta: 0.0,
            max_radius: 50.0,
            min_radius: 1.0,
        }
    }
}

#[derive(Default)]
pub struct OrbitControlsSystem;

impl OrbitControlsSystem {
    pub fn build(&mut self) -> Box<dyn Schedulable> {
        SystemBuilder::<()>::new("OrbitControlsSystem")
            .with_query(<(Write<OrbitControl>, Write<LocalToWorld>)>::query())
            .read_component::<Parent>()
            .read_component::<LocalToWorld>()
            .read_resource::<InputHandler>()
            .build(move |_commands, _world, input_handler, query| {
                let mouse_delta = input_handler.mouse_movement_delta();

                for (mut orbit_control, mut local_to_world) in query.iter() {
                    if input_handler.mouse_button_is_down(MouseButton::Right) {
                        orbit_control.phi -= mouse_delta.x * 0.002;
                        orbit_control.theta -= mouse_delta.y * 0.002;
                    }

                    // Re-calculate the world transform.
                    let pivot = UnitQuaternion::new(Vector3::new(0.0, orbit_control.phi, 0.0))
                        * UnitQuaternion::new(Vector3::new(orbit_control.theta, 0.0, 0.0));
                    let boom = Vector3::new(0.0, 0.0, orbit_control.radius);
                    let anchor_location: Vector3<f32> = pivot * boom;

                    // TODO: Add pivot location
                    let anchor_location = anchor_location + Vector3::new(0.0, 0.0, 0.0);

                    *local_to_world = LocalToWorld(
                        (-pivot)
                            .to_homogeneous()
                            .append_translation(&anchor_location),
                    );
                }
            })
    }
}
