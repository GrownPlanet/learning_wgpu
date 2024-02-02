use state::State;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

mod camera;
mod state;
mod texture;
mod vertex;

async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("exploring wgpu");

    let mut state = State::new(window).await;

    event_loop
        .run(move |event, target| {
            state.update();
            state.render().unwrap();
            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                if state.input(&event) {
                    // return;
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
            }
        })
        .unwrap();
}

fn main() {
    pollster::block_on(run());
}
