use crate::app_state::*;
use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};
use smooth_bevy_cameras::{
    controllers::orbit::{ControlEvent, OrbitCameraController},
    LookTransform,
};

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, reset_camera)
            .add_exit_system(AppState::InGame, freeze_camera)
            .add_system(control_camera.run_in_state(AppState::InGame));
    }
}

fn reset_camera(mut query: Query<(&mut OrbitCameraController, &mut LookTransform)>) {
    for (mut orbit, mut look) in query.iter_mut() {
        orbit.enabled = true;
        look.target = Vec3::default();
        look.eye = Vec3::new(5., 10., 5.);
    }
}

fn freeze_camera(mut query: Query<(&mut OrbitCameraController, &mut LookTransform)>) {
    for (mut orbit, mut look) in query.iter_mut() {
        orbit.enabled = false;
        look.target = Vec3::default();
        look.eye = Vec3::new(5., 10., 5.);
    }
}

fn control_camera(
    mut events: EventWriter<ControlEvent>,
    keys: Res<Input<KeyCode>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    controllers: Query<&OrbitCameraController>,
) {
    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        cursor_delta += event.delta;
    }

    if keys.any_pressed([KeyCode::LControl, KeyCode::RControl]) {
        // Orbit Mode
        if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
            events.send(ControlEvent::Orbit(Vec2::new(1., 0.)));
        }
        if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
            events.send(ControlEvent::Orbit(Vec2::new(-1., 0.)));
        }
        if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
            events.send(ControlEvent::Orbit(Vec2::new(0., 1.)));
        }
        if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
            events.send(ControlEvent::Orbit(Vec2::new(0., -1.)));
        }
    } else {
        // Move Mode
        if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
            events.send(ControlEvent::TranslateTarget(Vec2::new(1., 0.)));
        }
        if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
            events.send(ControlEvent::TranslateTarget(Vec2::new(-1., 0.)));
        }
        if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
            events.send(ControlEvent::TranslateTarget(Vec2::new(0., 1.)));
        }
        if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
            events.send(ControlEvent::TranslateTarget(Vec2::new(0., -1.)));
        }
    }

    if keys.any_pressed([KeyCode::NumpadSubtract, KeyCode::Minus]) {
        events.send(ControlEvent::Zoom(1.01));
    }
    if keys.any_pressed([KeyCode::NumpadAdd, KeyCode::Equals]) {
        events.send(ControlEvent::Zoom(0.99));
    }

    if mouse_buttons.pressed(MouseButton::Middle) {
        events.send(ControlEvent::TranslateTarget(
            controller.mouse_translate_sensitivity * cursor_delta,
        ));
    }

    if mouse_buttons.pressed(MouseButton::Right) {
        events.send(ControlEvent::Orbit(
            controller.mouse_rotate_sensitivity * cursor_delta,
        ));
    }

    for ev in mouse_wheel.iter() {
        let mut scalar = ev.y * controller.mouse_wheel_zoom_sensitivity;
        if ev.unit == MouseScrollUnit::Line {
            scalar /= controller.pixels_per_line;
        }
        events.send(ControlEvent::Zoom(1. - scalar));
    }
}
