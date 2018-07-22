extern crate winit;
use winit::{Window, Event, ControlFlow, WindowEvent, EventsLoop};

fn main() {
    println!("Hello, world!");
    let mut event_loop = EventsLoop::new();
    let _win = Window::new(&event_loop).unwrap();

    event_loop.run_forever( |event| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested, ..
            } => ControlFlow::Break,
            _ => ControlFlow::Continue,
        }
    });
    println!("main thread ends. {:?}", _win.id());
}
