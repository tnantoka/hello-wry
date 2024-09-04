use tao::{
  event::{Event, StartCause, WindowEvent},
  event_loop::{ControlFlow, EventLoopBuilder},
  window::WindowBuilder,
};
use wry::{http::Request, WebViewBuilder};
use dirs::data_local_dir;
use serde::{Deserialize, Serialize};
use std::fs::{File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::fs;


const INDEX_HTML: &str = include_str!("./index.html");

enum UserEvent {
  Hello(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    color: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            color: "ffffff".to_string(),
        }
    }
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

  let mut settings = read_settings_json()?;
  println!("settings: {:?}", settings);

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => {
        println!("Wry has started!");
        set_color(&webview, settings.color.as_str());
      },
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      Event::UserEvent(e) => match e {
        UserEvent::Hello(color) => {
            set_color(&webview, &color);
            settings.color = color;
            write_settings_json(&settings).unwrap();
        }
      },
      _ => (),
    }
  });
}

fn read_settings_json() -> Result<Settings, std::io::Error> {
    let path = app_dir()?.join("settings.json");
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let settings = serde_json::from_reader(reader)?;
        Ok(settings)
    } else {
        let settings = Settings::default();
        write_settings_json(&settings)?;
        Ok(settings)
    }
}

fn write_settings_json(settings: &Settings) -> Result<(), std::io::Error> {
    let path = app_dir()?.join("settings.json");
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, settings)?;
    Ok(())
}

fn app_dir() -> Result<PathBuf, std::io::Error> {
    let mut path = data_local_dir().unwrap();
    path.push("hello-wry");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

fn set_color(webview: &wry::WebView, color: &str) {
            let _ = webview.evaluate_script(
                &format!(
                    "document.body.style.backgroundColor = '{}'",
                    color
                )
            );
            println!("color: {}", color);
}