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

fn adjust_water_level(water_level: &mut f32, change: f32) {
    *water_level += change;
    *water_level = if *water_level > 255.0 {
        255.0
    } else if *water_level < 0.0 {
        0.0
    } else {
        *water_level
    };
}

pub fn key_event(
    input: KeyboardInput,
    camera: &mut CameraMatrix,
    water_level: &mut f32,
    scale: usize,
) {
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
            VirtualKeyCode::Right => *camera.position.x_mut() += 0.5 * scale as f32,
            VirtualKeyCode::Left => *camera.position.x_mut() -= 0.5 * scale as f32,
            VirtualKeyCode::Up => *camera.position.y_mut() += 0.5 * scale as f32,
            VirtualKeyCode::Down => *camera.position.y_mut() -= 0.5 * scale as f32,
            VirtualKeyCode::P => *camera.position.z_mut() += 0.5 * scale as f32,
            VirtualKeyCode::O => *camera.position.z_mut() -= 0.5 * scale as f32,
            VirtualKeyCode::D => *camera.direction.x_mut() += 0.5 * scale as f32,
            VirtualKeyCode::A => *camera.direction.x_mut() -= 0.5 * scale as f32,
            VirtualKeyCode::W => *camera.direction.y_mut() += 0.5 * scale as f32,
            VirtualKeyCode::S => *camera.direction.y_mut() -= 0.5 * scale as f32,
            VirtualKeyCode::Q => *camera.direction.z_mut() -= 0.5 * scale as f32,
            VirtualKeyCode::E => *camera.direction.z_mut() += 0.5 * scale as f32,
            VirtualKeyCode::J => adjust_water_level(water_level, -0.5),
            VirtualKeyCode::K => adjust_water_level(water_level, 0.5),
            _ => (),
        },
        _ => (),
    }
}
