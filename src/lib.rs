pub mod delauney;
pub mod map;
pub mod render;
pub mod teapot;

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
    pub vertex_buffer: VertexBuffer<Coord>,
    pub indices: IndexBuffer<u16>,
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
        let vertex_buffer = match VertexBuffer::new(&display, &map.vertices) {
            Ok(vertex_buffer) => vertex_buffer,
            Err(_) => return Err("Unable to create vertex buffer from mod1 file"),
        };
        let indices = match IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &map.indices,
        ) {
            Ok(indices) => indices,
            Err(_) => return Err("Unable to create indices buffer from mod1 file"),
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
            vertex_buffer,
            indices,
        })
    }
}

fn create_vertex_shader() -> String {
    String::from(
        r#"
        #version 150

        in vec3 position;
        out float elevation; // pass position on to fragment shader

        uniform mat4 perspective;
        uniform mat4 model;
        uniform mat4 view;

        void main() {
            elevation = position.z;

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
            out vec4 color;

            void main() {
                color = vec4(0.0, (elevation / 255.0), 0.0, 1.0);
            }
        "#,
    )
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let Config {
        event_loop,
        display,
        scale,
        mut camera_matrix,
        program,
        mut vertex_buffer,
        mut indices,
    } = config;

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

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
                &indices,
                &program,
                &glium::uniform! { model: model, perspective: perspective, view: camera_matrix.mat4() },
                &params,
            )
            .unwrap();

        target.finish().unwrap();

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
                    render::key_event(input, &mut camera_matrix, scale);
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
}
