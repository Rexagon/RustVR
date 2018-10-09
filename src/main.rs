#[macro_use]
extern crate glium;

use std::fs;
use glium::Surface;

mod core;

struct Triangle {
    vertex_buffer: glium::VertexBuffer<core::Vertex>,
    indices: glium::index::NoIndices,
    program: glium::program::Program,
}

impl Triangle {
    fn new(display: &glutin::Facade) -> Triangle {
        let vertex_shader = fs::read_to_string("./data/color.vert").expect("Unable to load vertex shader");
        let fragment_shader = fs::read_to_string("./data/color.frag").expect("Unable to load fragment shader");

        let vertices = vec![
            core::Vertex { position: [-0.5, -0.5] },
            core::Vertex { position: [0.0, 0.5] },
            core::Vertex { position: [0.5, -0.25] }
        ];

        Triangle {
            vertex_buffer: glium::VertexBuffer::new(&display, &vertices).unwrap(),
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            program: glium::program::Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap()
        }
    }
}

struct MainScene<'a> {
    display: &'a glium::Display,
    triangle: Option<Triangle>,
}

impl<'a> MainScene<'a> {
    fn new(display: &glium::Display) -> MainScene {
        MainScene {
            display,
            triangle: None,
        }
    }
}

impl<'a> core::Scene for MainScene<'a> {
    fn on_init(&mut self) {
        self.vertices = Some(Triangle::new(self.display));
        println!("Init called");
    }

    fn on_close(&mut self) {
        println!("Close called");
    }

    fn on_update(&mut self, dt: f64) {
        let mut target = self.display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);
    }
}

fn main() {
    let mut core = core::Core::new();
    core.scene_manager().add_scene(Box::new(MainScene::new(core.display())));

    core.start();
}