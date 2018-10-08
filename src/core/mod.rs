use glium::glutin;
use glium::Surface;

pub mod scene_manager;
pub mod vertex;

pub struct Core {
    display: glium::Display,
    event_loop: glutin::EventsLoop,

    running: bool,
}

impl Core {
    pub fn new() -> Core {
        let window = glutin::WindowBuilder::new()
            .with_dimensions(glutin::dpi::LogicalSize { width: 1024.0, height: 768.0 })
            .with_title("Hello world");

        let context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);


        let event_loop = glutin::EventsLoop::new();

        let display = match glium::Display::new(window, context, &event_loop) {
            Ok(display) => display,
            Err(err) => panic!(err)
        };

        Core {
            display,
            event_loop,
            running: false
        }
    }

    pub fn start(&mut self) {
        self.running = true;

        while self.running {
            let running = &mut self.running;

            // Events handling
            self.event_loop.poll_events(move |event| match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => *running = false,
                    _ => (),
                },
                _ => (),
            });

            // Drawing
            let mut target = self.display.draw();
            target.clear_color_srgb(0.1, 0.1, 0.1, 1.0);

            target.finish().unwrap();
        }
    }
}