use state::State;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

mod state;

async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;

    event_loop
        .run(move |event, target| {
            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                if state.input(&event) {
                    return;
                }

                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => target.exit(),
                    WindowEvent::Resized(new_inner_size) => state.resize(new_inner_size),
                    WindowEvent::RedrawRequested => {
                        state.update();
                        state.render().unwrap();
                    }
                    _ => (),
                }
                state.render().unwrap();
            }
        })
        .unwrap();
}

fn main() {
    pollster::block_on(run());
}
