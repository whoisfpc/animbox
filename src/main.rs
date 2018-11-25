extern crate winit;

fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let builder = winit::WindowBuilder::new()
        .with_dimensions((900, 700).into())
        .with_title("animbox");
    let _window = builder.build(&events_loop).unwrap();

    events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent {
              event: winit::WindowEvent::CloseRequested,
              ..
            } => winit::ControlFlow::Break,
            _ => winit::ControlFlow::Continue,
        }
    });
}