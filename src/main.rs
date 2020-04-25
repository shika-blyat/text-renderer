mod window;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use window::Window;

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(500.0, 500.0, &event_loop);

    /*let mut text_box = TextBox::new(Size::new(0.5, 0.5));
    let font = Font::new("ttf/JetBrainsMono-Regular");
    text_box.set_font(&font);
    loop{
        let mut content = vec!["Caca", "Foo", "Bar"];
        text_box.update_content(&content)
        content.pop()
        content.push("Baz");
    }
    */
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                // Application update code.
                // Queue a RedrawRequested event.
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferrable to render in this event rather than in MainEventsCleared, since
                // rendering in here allows the program to gracefully handle redraws requested
                // by the OS.
            }
            _ => (),
        }
    })
}
