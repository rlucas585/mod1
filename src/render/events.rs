use glium::glutin;

use glutin::event::{ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode};

use super::types::*;

pub fn mouse_scroll(zoom: &mut f32, delta: MouseScrollDelta) {
    match delta {
        MouseScrollDelta::LineDelta(_, vertical) => *zoom += vertical,
        MouseScrollDelta::PixelDelta(_) => (),
    }
    *zoom = if *zoom < 1.0 {
        1.0
    } else if *zoom > 4.0 {
        4.0
    } else {
        *zoom
    };
}

pub fn key_event(input: KeyboardInput, camera: &mut CameraMatrix) {
    match input {
        KeyboardInput {
            scancode: _,
            state: ElementState::Released,
            ..
        } => (), // Ignore key releases, only act on key press
        KeyboardInput {
            scancode: _,
            state: _,
            virtual_keycode: Some(keycode),
            ..
        } => match keycode {
            VirtualKeyCode::Right => *camera.position.x_mut() += 0.05,
            VirtualKeyCode::Left => *camera.position.x_mut() -= 0.05,
            VirtualKeyCode::Up => *camera.position.y_mut() += 0.05,
            VirtualKeyCode::Down => *camera.position.y_mut() -= 0.05,
            VirtualKeyCode::P => *camera.position.z_mut() += 0.05,
            VirtualKeyCode::O => *camera.position.z_mut() -= 0.05,
            VirtualKeyCode::D => *camera.direction.x_mut() += 0.05,
            VirtualKeyCode::A => *camera.direction.x_mut() -= 0.05,
            VirtualKeyCode::W => *camera.direction.y_mut() += 0.05,
            VirtualKeyCode::S => *camera.direction.y_mut() -= 0.05,
            VirtualKeyCode::Q => *camera.direction.z_mut() -= 0.05,
            VirtualKeyCode::E => *camera.direction.z_mut() += 0.05,
            VirtualKeyCode::Key6 => *camera.up.x_mut() += 0.05,
            VirtualKeyCode::Key7 => *camera.up.y_mut() += 0.05,
            VirtualKeyCode::Key8 => *camera.up.z_mut() += 0.05,
            VirtualKeyCode::Y => *camera.up.x_mut() -= 0.05,
            VirtualKeyCode::U => *camera.up.y_mut() -= 0.05,
            VirtualKeyCode::I => *camera.up.z_mut() -= 0.05,
            _ => (),
        },
        _ => (),
    }
}
