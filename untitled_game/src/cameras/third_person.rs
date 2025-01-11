use std::f32::consts::*;
use std::ops::Range;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use crate::{Player};
use crate::menu::MenuState;

#[derive(Debug, Component)]
#[require(Transform)]
pub struct OrbitCamera {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub yaw_speed: f32,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_pause: KeyCode,
    pub mouse_interact: MouseButton,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub velocity: Vec3,
    pub key_run: KeyCode,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        // Limiting pitch stops some unexpected rotation past 90° up or down.
        let pitch_limit = FRAC_PI_2 - 0.1;
        Self {
            friction: 0.1,
            key_down: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_pause: KeyCode::Escape,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyW,
            key_run: KeyCode::ShiftLeft,
            mouse_interact: MouseButton::Left,
            orbit_distance: 20.0,
            pitch_range: -pitch_limit..pitch_limit,
            pitch_speed: 0.003,
            run_speed: 15.0,
            walk_speed: 5.0,
            yaw_speed: 0.004,
            velocity: Vec3::ZERO,
        }
    }
}

fn orbit(
    query: Single<(&mut Transform, &mut OrbitCamera), (With<Camera>, Without<Player>)>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window>,
    mut player: Single<&mut Transform, With<Player>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    let (mut camera, mut controller) = query.into_inner();

    if key_input.pressed(controller.key_pause) {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
        menu_state.set(MenuState::Main)
    }

    let dt = time.delta_secs();

    let mut axis_input = Vec3::ZERO;
    if key_input.pressed(controller.key_up) {
        axis_input.z += 1.0;
    }
    if key_input.pressed(controller.key_down) {
        axis_input.z -= 1.0;
    }
    if key_input.pressed(controller.key_right) {
        axis_input.x += 1.0;
    }
    if key_input.pressed(controller.key_left) {
        axis_input.x -= 1.0;
    }

    // Apply movement update
    if axis_input != Vec3::ZERO {
        let max_speed = if key_input.pressed(controller.key_run) {
            controller.run_speed
        } else {
            controller.walk_speed
        };
        controller.velocity = axis_input.normalize() * max_speed;
    } else {
        let friction = controller.friction.clamp(0.0, 1.0);
        controller.velocity *= 1.0 - friction;
        if controller.velocity.length_squared() < 1e-6 {
            controller.velocity = Vec3::ZERO;
        }
    }
    let forward = camera.forward();
    let forward = Dir3::new(Vec3::new(forward.x, 0., forward.z)).unwrap();
    let right = camera.right();
    player.translation += controller.velocity.x * dt * right
        + controller.velocity.y * dt * Vec3::Y
        + controller.velocity.z * dt * forward;

    let delta = accumulated_mouse_motion.delta;

    // Mouse motion is one of the few inputs that should not be multiplied by delta time,
    // as we are already receiving the full movement since the last frame was rendered. Multiplying
    // by delta time here would make the movement slower that it should be.
    let delta_pitch = delta.y * controller.pitch_speed;
    let delta_yaw = delta.x * controller.yaw_speed;

    // Obtain the existing pitch, yaw, and roll values from the transform.
    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);

    // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
    let pitch = (pitch + delta_pitch).clamp(
        controller.pitch_range.start,
        controller.pitch_range.end,
    );
    let yaw = yaw - delta_yaw;
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    // Adjust the translation to maintain the correct orientation toward the orbit target.
    // In our example it's a static target, but this could easily be customized.
    camera.translation = player.translation - camera.forward() * controller.orbit_distance;
}

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, orbit.run_if(in_state(MenuState::Disabled)));
    }
}
