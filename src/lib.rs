pub mod delauney;
pub mod map;
pub mod render;
pub mod teapot;

use crate::map::Rectangle;
use crate::render::{CameraBuilder, CameraMatrix, Coord};
use glium::{self, glutin, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub event_loop: glutin::event_loop::EventLoop<()>,
    pub display: glium::Display,
    pub scale: usize,
    pub camera_matrix: CameraMatrix,
    pub program: Program,
    pub vertices: Vec<Coord>,
    pub indices: Vec<u16>,
    pub base: Rectangle,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Self, &'static str> {
        args.next(); // Skip executable name

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("mod1 must be run with at least one argument, a mod1 file"),
        };

        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let map = match map::Map::new_from_file(&filename) {
            Ok(map) => map,
            Err(_) => return Err("Invalid mod1 file"),
        };

        let camera_matrix = CameraBuilder::new()
            .zoom(1.0)
            .position(Coord::new(
                *map.center().x(),
                *map.center().y() - 5.0 * map.scale as f32,
                map.elevation_max * 5.0,
            ))
            .direction(Coord::new(0.0, 1.0, -1.0))
            .up(Coord::new(0.0, 1.0, 0.0))
            .build();

        let vertices = map.vertices;
        let indices = map.indices;

        let vertex_shader_src = create_vertex_shader();
        let fragment_shader_src = create_fragment_shader();
        let program =
            Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
        Ok(Self {
            event_loop,
            display,
            scale: map.scale,
            camera_matrix,
            program,
            vertices,
            indices,
            base: map.base,
        })
    }
}

fn create_vertex_shader() -> String {
    String::from(
        r#"
        #version 150

        in vec3 position;
        in float is_water; // Can't use bool in GLSL
        out float elevation; // pass position on to fragment shader
        out float v_water;

        uniform mat4 perspective;
        uniform mat4 model;
        uniform mat4 view;

        void main() {
            elevation = position.z;
            v_water = is_water;

            mat4 modelview = view * model;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
        "#,
    )
}

fn create_fragment_shader() -> String {
    String::from(
        r#"
            #version 150

            in float elevation;
            in float v_water;
            out vec4 color;

            void main() {
                if (v_water > 0.5) {
                    color = vec4(0.0, 0.0, 200.0, 1.0);
                } else {
                    color = vec4(0.0, (elevation / 255.0), 0.0, 1.0);
                }
            }
        "#,
    )
}

fn add_water(
    vertices: &mut Vec<Coord>,
    indices: &mut Vec<u16>,
    base: &Rectangle,
    water_level: &f32,
) {
    assert!(vertices.len() >= 4);
    // Base water
    vertices.push(Coord::new(*base.origin.x(), *base.origin.y(), 0.0).set_as_water());
    vertices.push(Coord::new(*base.origin.x() + base.x_size, *base.origin.y(), 0.0).set_as_water());
    vertices.push(Coord::new(*base.origin.x(), *base.origin.y() + base.y_size, 0.0).set_as_water());
    vertices.push(
        Coord::new(
            *base.origin.x() + base.x_size,
            *base.origin.y() + base.y_size,
            0.0,
        )
        .set_as_water(),
    );
    // Current water level
    vertices.push(Coord::new(*base.origin.x(), *base.origin.y(), *water_level).set_as_water());
    vertices.push(
        Coord::new(
            *base.origin.x() + base.x_size,
            *base.origin.y(),
            *water_level,
        )
        .set_as_water(),
    );
    vertices.push(
        Coord::new(
            *base.origin.x(),
            *base.origin.y() + base.y_size,
            *water_level,
        )
        .set_as_water(),
    );
    vertices.push(
        Coord::new(
            *base.origin.x() + base.x_size,
            *base.origin.y() + base.y_size,
            *water_level,
        )
        .set_as_water(),
    );

    // Hard coding is not a great idea but I'm a little strained for time here

    // Bottom water level
    let base_first_water_index = (vertices.len() - 8) as u16;
    indices.push(base_first_water_index);
    indices.push(base_first_water_index + 1);
    indices.push(base_first_water_index + 2);

    indices.push(base_first_water_index + 1);
    indices.push(base_first_water_index + 2);
    indices.push(base_first_water_index + 3);

    // Top water level
    let current_first_water_index = (vertices.len() - 4) as u16;
    indices.push(current_first_water_index);
    indices.push(current_first_water_index + 1);
    indices.push(current_first_water_index + 2);

    indices.push(current_first_water_index + 1);
    indices.push(current_first_water_index + 2);
    indices.push(current_first_water_index + 3);

    // Front side
    indices.push(base_first_water_index);
    indices.push(base_first_water_index + 1);
    indices.push(current_first_water_index);

    indices.push(current_first_water_index);
    indices.push(base_first_water_index + 1);
    indices.push(current_first_water_index + 1);

    // Reverse side
    indices.push(base_first_water_index + 2);
    indices.push(base_first_water_index + 3);
    indices.push(current_first_water_index + 2);

    indices.push(current_first_water_index + 2);
    indices.push(base_first_water_index + 3);
    indices.push(current_first_water_index + 3);

    // Left side
    indices.push(base_first_water_index);
    indices.push(base_first_water_index + 2);
    indices.push(current_first_water_index);

    indices.push(current_first_water_index);
    indices.push(base_first_water_index + 2);
    indices.push(current_first_water_index + 2);

    // Right side
    indices.push(base_first_water_index + 1);
    indices.push(base_first_water_index + 3);
    indices.push(current_first_water_index + 1);

    indices.push(current_first_water_index + 1);
    indices.push(base_first_water_index + 3);
    indices.push(current_first_water_index + 3);
}

fn remove_water(vertices: &mut Vec<Coord>, indices: &mut Vec<u16>) {
    assert!(vertices.len() >= 8);
    assert!(indices.len() >= 36);
    for _ in 0..8 {
        vertices.pop();
    }
    for _ in 0..36 {
        indices.pop();
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let Config {
        event_loop,
        display,
        scale,
        mut camera_matrix,
        program,
        mut vertices,
        mut indices,
        base,
    } = config;

    let mut water_level = 0.0;

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut error = None;

    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        add_water(&mut vertices, &mut indices, &base, &water_level);

        let vertex_buffer = match VertexBuffer::new(&display, &vertices) {
            Ok(vertex_buffer) => vertex_buffer,
            Err(_) => {
                error = Some(String::from("Unable to create vertex buffer"));
                return;
            }
        };
        let indices_buffer = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        ).unwrap();

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        };

        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, camera_matrix.zoom, 1.0f32],
        ];

        target
            .draw(
                &vertex_buffer,
                &indices_buffer,
                &program,
                &glium::uniform! { model: model, perspective: perspective, view: camera_matrix.mat4() },
                &params,
            )
            .unwrap();

        target.finish().unwrap();

        remove_water(&mut vertices, &mut indices);

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::KeyboardInput {
                    device_id: _,
                    input,
                    is_synthetic: _,
                } => {
                    render::key_event(input, &mut camera_matrix, &mut water_level, scale);
                }
                glutin::event::WindowEvent::MouseWheel {
                    device_id: _,
                    delta,
                    phase: _,
                    ..
                } => {
                    render::mouse_scroll(&mut camera_matrix.zoom, delta, scale);
                }
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
    if let Some(err) = error {
        return Err(&err);
    } else {
        return Ok(());
    }
}
