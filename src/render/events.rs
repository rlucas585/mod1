use glium::glutin;

use glutin::event::{ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode};

use super::types::*;

pub fn mouse_scroll(zoom: &mut f32, delta: MouseScrollDelta, scale: usize) {
    match delta {
        MouseScrollDelta::LineDelta(_, vertical) => *zoom += vertical * scale as f32,
        MouseScrollDelta::PixelDelta(_) => (),
    }
    *zoom = if *zoom < 1.0 * scale as f32 {
        1.0
    } else if *zoom > 4.0 * scale as f32 {
        4.0
    } else {
        *zoom
    };
}

pub fn key_event(input: KeyboardInput, camera: &mut CameraMatrix, scale: usize) {
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
            VirtualKeyCode::Right => *camera.position.x_mut() += 0.05 * scale as f32,
            VirtualKeyCode::Left => *camera.position.x_mut() -= 0.05 * scale as f32,
            VirtualKeyCode::Up => *camera.position.y_mut() += 0.05 * scale as f32,
            VirtualKeyCode::Down => *camera.position.y_mut() -= 0.05 * scale as f32,
            VirtualKeyCode::P => *camera.position.z_mut() += 0.05 * scale as f32,
            VirtualKeyCode::O => *camera.position.z_mut() -= 0.05 * scale as f32,
            VirtualKeyCode::D => *camera.direction.x_mut() += 0.05 * scale as f32,
            VirtualKeyCode::A => *camera.direction.x_mut() -= 0.05 * scale as f32,
            VirtualKeyCode::W => *camera.direction.y_mut() += 0.05 * scale as f32,
            VirtualKeyCode::S => *camera.direction.y_mut() -= 0.05 * scale as f32,
            VirtualKeyCode::Q => *camera.direction.z_mut() -= 0.05 * scale as f32,
            VirtualKeyCode::E => *camera.direction.z_mut() += 0.05 * scale as f32,
            VirtualKeyCode::Key6 => *camera.up.x_mut() += 0.05 * scale as f32,
            VirtualKeyCode::Key7 => *camera.up.y_mut() += 0.05 * scale as f32,
            VirtualKeyCode::Key8 => *camera.up.z_mut() += 0.05 * scale as f32,
            VirtualKeyCode::Y => *camera.up.x_mut() -= 0.05 * scale as f32,
            VirtualKeyCode::U => *camera.up.y_mut() -= 0.05 * scale as f32,
            VirtualKeyCode::I => *camera.up.z_mut() -= 0.05 * scale as f32,
            _ => (),
        },
        _ => (),
    }
}
