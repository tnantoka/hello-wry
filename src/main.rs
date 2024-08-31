use tao::{
  event::{Event, StartCause, WindowEvent},
  event_loop::{ControlFlow, EventLoopBuilder},
  window::WindowBuilder,
};
use wry::{http::Request, WebViewBuilder};

const INDEX_HTML: &str = include_str!("./index.html");

enum UserEvent {
  Hello(String),
}

fn main() -> wry::Result<()> {
  let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
  let window = WindowBuilder::new()
    .with_title("Hello World")
    .build(&event_loop)
    .unwrap();

  let proxy = event_loop.create_proxy();
  let handler = move |req: Request<String>| {
    let body = req.body();
    match body.as_str() {
      _ if body.starts_with("hello:") => {
        let color = body.replace("hello:", "");
        let _ = proxy.send_event(UserEvent::Hello(color));
      }
      _ => {}
    }
  };

  let webview = WebViewBuilder::new(&window)
    .with_html(INDEX_HTML)
    .with_ipc_handler(handler)
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      Event::UserEvent(e) => match e {
        UserEvent::Hello(color) => {
            let _ = webview.evaluate_script(
                &format!(
                    "document.body.style.backgroundColor = '{}'",
                    color
                )
            );
            println!("color: {}", color);
        }
      },
      _ => (),
    }
  });
}
