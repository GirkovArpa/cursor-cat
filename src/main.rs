#![windows_subsystem = "windows"]
#[macro_use]
extern crate sciter;

use device_query::{DeviceQuery, DeviceState, MouseState};
use std::thread;

use sciter::Value;
struct EventHandler;
impl EventHandler {
    fn set_volume(&self) -> () {}
}

impl sciter::EventHandler for EventHandler {
    fn document_complete(&mut self, root: sciter::HELEMENT, source: sciter::HELEMENT) {
        let root = sciter::Element::from(root);
        std::thread::spawn(|| {
            monitor_mouse(root);
        });
    }
    fn get_subscription(&mut self) -> Option<sciter::dom::event::EVENT_GROUPS> {
        Some(
            sciter::dom::event::default_events()
                | sciter::dom::event::EVENT_GROUPS::HANDLE_METHOD_CALL,
        )
    }
    dispatch_script_call!(
        fn set_volume();
    );
}

fn main() {
    sciter::set_options(sciter::RuntimeOptions::DebugMode(false)).unwrap();
    let archived = include_bytes!("../target/assets.rc");
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_EVAL as u8,
    ))
    .unwrap();
    let mut frame = sciter::Window::new();
    frame.event_handler(EventHandler {});
    frame.archive_handler(archived).unwrap();
    frame.load_file("this://app/html/main.html");
    frame.run_app();
}

fn monitor_mouse(mut element: sciter::Element) {
    let device_state = DeviceState::new();

    loop {
        let mouse: MouseState = device_state.get_mouse();
        //println!("{:?}", mouse.coords);
        let (x, y) = mouse.coords;
        //element.set_text(&format!("{:?}", mouse.coords).to_owned());
        //&element.call_function("move_circle", &make_args!(x, y));

        let script = format!(
            "
            document.querySelector('#disc').style.left = {} + 'px';
            document.querySelector('#disc').style.top = {} + 'px';
        ",
            x, y
        );

        //println!("{}", &script);

        //element.eval_script(&script);

        &element.call_function("move_circle", &make_args!(x, y));

        use std::{thread, time::Duration};
        //thread::sleep(Duration::from_millis(1000));
    }
}
