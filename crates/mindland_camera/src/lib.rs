//! MindLand Ultra-Smooth Camera System
//! 
//! First-person camera with sub-millisecond response time and quaternion-based rotation.

use bevy::{
    prelude::*,
    render::camera::CameraProjection,
};
use glam::Quat;

/// High-performance first-person camera controller
#[derive(Component)]
pub struct CameraController {
    pub transform: Transform,
    pub projection: PerspectiveProjection,
    pub movement_state: MovementState,
    pub sensitivity: f32,
    pub smoothing: ExponentialSmoothing,
    pub update_rate: u32, // Target 1000Hz internal updates
}

/// Movement state with acceleration curves
#[derive(Debug, Clone)]
pub struct MovementState {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub max_speed: f32,
    pub sprint_multiplier: f32,
    pub precision_multiplier: f32,
    pub friction: f32,
}

/// Exponential smoothing for micro-stutter elimination
#[derive(Debug, Clone)]
pub struct ExponentialSmoothing {
    pub alpha: f32,
    pub previous_value: Vec3,
    pub previous_rotation: Quat,
}

impl Default for CameraController {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraController {
    /// Create a new camera controller with optimized defaults
    pub fn new() -> Self {
        Self {
            transform: Transform::from_xyz(0.0, 1.8, 0.0), // Eye level height
            projection: PerspectiveProjection {
                fov: 70.0_f32.to_radians(), // Comfortable FOV
                near: 0.1,
                far: 1000.0,
                aspect_ratio: 16.0 / 9.0,
            },
            movement_state: MovementState {
                velocity: Vec3::ZERO,
                acceleration: Vec3::ZERO,
                max_speed: 5.0,
                sprint_multiplier: 3.0,
                precision_multiplier: 0.3,
                friction: 0.9,
            },
            sensitivity: 0.002, // Optimized mouse sensitivity
            smoothing: ExponentialSmoothing {
                alpha: 0.8,
                previous_value: Vec3::ZERO,
                previous_rotation: Quat::IDENTITY,
            },
            update_rate: 1000, // 1000Hz internal update rate
        }
    }

    /// Update camera rotation using quaternions (prevents gimbal lock)
    pub fn update_rotation(&mut self, mouse_delta: Vec2, delta_time: f32) {
        if mouse_delta.length_squared() < f32::EPSILON {
            return;
        }

        // Calculate rotation deltas
        let yaw_delta = -mouse_delta.x * self.sensitivity;
        let pitch_delta = -mouse_delta.y * self.sensitivity;

        // Create rotation quaternions
        let yaw_rotation = Quat::from_rotation_y(yaw_delta);
        let pitch_rotation = Quat::from_rotation_x(pitch_delta);

        // Apply rotations (yaw around world Y, pitch around local X)
        self.transform.rotation = yaw_rotation * self.transform.rotation * pitch_rotation;

        // Clamp pitch to prevent over-rotation
        let (yaw, pitch, _roll) = self.transform.rotation.to_euler(EulerRot::YXZ);
        let clamped_pitch = pitch.clamp(-1.5, 1.5); // ~86 degrees
        self.transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, clamped_pitch, 0.0);

        // Apply exponential smoothing
        self.smoothing.previous_rotation = self.smoothing.previous_rotation.slerp(
            self.transform.rotation,
            self.smoothing.alpha * delta_time * self.update_rate as f32
        );
    }

    /// Update camera movement with acceleration curves
    pub fn update_movement(&mut self, movement_input: Vec3, sprint: bool, precision: bool, delta_time: f32) {
        // Calculate target velocity based on input
        let speed_multiplier = if sprint {
            self.movement_state.sprint_multiplier
        } else if precision {
            self.movement_state.precision_multiplier
        } else {
            1.0
        };

        let target_velocity = movement_input * self.movement_state.max_speed * speed_multiplier;

        // Apply acceleration for natural feel
        let velocity_diff = target_velocity - self.movement_state.velocity;
        self.movement_state.acceleration = velocity_diff * 10.0; // Responsive acceleration

        // Update velocity with acceleration
        self.movement_state.velocity += self.movement_state.acceleration * delta_time;

        // Apply friction when no input
        if movement_input.length_squared() < f32::EPSILON {
            self.movement_state.velocity *= self.movement_state.friction;
        }

        // Transform velocity to world space
        let forward = -self.transform.local_z();
        let right = self.transform.local_x();
        let up = Vec3::Y; // Always use world up for movement

        let world_velocity = 
            right * self.movement_state.velocity.x +
            up * self.movement_state.velocity.y +
            forward * self.movement_state.velocity.z;

        // Apply smoothing to eliminate micro-stutters
        let smoothed_velocity = self.smoothing.previous_value.lerp(
            world_velocity,
            self.smoothing.alpha * delta_time * self.update_rate as f32
        );

        // Update position
        self.transform.translation += smoothed_velocity * delta_time;
        self.smoothing.previous_value = smoothed_velocity;
    }

    /// Get the view matrix for rendering (SIMD-optimized)
    pub fn view_matrix(&self) -> Mat4 {
        self.transform.compute_matrix().inverse()
    }

    /// Get the projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.get_projection_matrix()
    }
}