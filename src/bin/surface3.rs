//! # surface3
//!
//! `surface3` attempts to incorporate elevation into the 3D model, with different color applied to
//! the surface at different heights.

use glium::Surface;

use mod1::map::Map;
use mod1::render::{self, CameraBuilder, Coord};

fn main() {
    use glium::glutin;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut zoom = 2.0;

    let map = Map::new_from_file("src/map/demo_c.mod1").unwrap();
    println!("map: {}", map);

    let vertex_buffer = glium::VertexBuffer::new(&display, &map.vertices).unwrap();
    println!("{:?}", map.indices);
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &map.indices,
    )
    .unwrap();

    let mut camera_matrix = CameraBuilder::new()
        .zoom(2.0)
        .position(Coord::new(
            0.5 * (*map.center().x() / 5.0),
            -0.6 * map.scale as f32,
            -1.5 + (map.scale as f32 / 3.0),
        ))
        .direction(Coord::new(0.0, 2.0, -1.0))
        .up(Coord::new(0.0, 1.0, 0.0))
        .build();

    let vertex_shader_src = r#"
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
        "#;

    let fragment_shader_src = r#"
            #version 150

            in float elevation;
            out vec4 color;

            void main() {
                color = vec4(0.0, (elevation / 255.0), 0.0, 1.0);
            }
            "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

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
            [0.1, 0.0, 0.0, 0.0],
            [0.0, 0.1, 0.0, 0.0],
            [0.0, 0.0, 0.1, 0.0],
            [0.0, 0.0, zoom, 1.0f32],
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
                    render::key_event(input, &mut camera_matrix, map.scale);
                }
                glutin::event::WindowEvent::MouseWheel {
                    device_id: _,
                    delta,
                    phase: _,
                    ..
                } => {
                    render::mouse_scroll(&mut zoom, delta, map.scale);
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
