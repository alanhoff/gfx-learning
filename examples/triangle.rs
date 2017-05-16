#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::Device;
use gfx::traits::FactoryExt;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

const TRIANGLE: [Vertex; 3] = [Vertex { pos: [-1.0, -1.0, 0.0] },
                               Vertex { pos: [1.0, -1.0, 0.0] },
                               Vertex { pos: [0.0, 1.0, 0.0] }];

pub fn main() {
    let events_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("My First Triangle".to_string())
        .with_dimensions(800, 600)
        .with_vsync();

    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let pso = factory
        .create_pipeline_simple(include_bytes!("shaders/triangle.glslv"),
                                include_bytes!("shaders/triangle.glslf"),
                                pipe::new())
        .unwrap();

    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };

    let mut running = true;
    while running {
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();

        events_loop.poll_events(|event: glutin::Event| {
            match event {
                glutin::Event::WindowEvent { event: glutin::WindowEvent::Closed, .. } => {
                    running = false;
                }
                _ => {}
            };
        });
    }
}
