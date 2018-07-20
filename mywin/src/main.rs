extern crate winit;
//use winit::{Window, Event, ControlFlow, WindowEvent, EventsLoop};

fn main() {
    println!("Hello, world!");
    let mut event_loop = winit::EventsLoop::new();
    let window = winit::Window::new(&event_loop).unwrap();

    event_loop.run_forever( |event| {
        match event {
            winit::Event::WindowEvent {
                event: winit::WindowEvent::CloseRequested, ..
            } => winit::ControlFlow::Break,
            _ => winit::ControlFlow::Continue,
        }
    });
}
