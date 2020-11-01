#![windows_subsystem = "windows"] // hide console window

extern crate image;
#[macro_use]
extern crate sciter;

fn main() {
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8
        | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8
    )).unwrap();

    let mut frame = sciter::Window::new();
    let handler = EventHandler {};
	frame.event_handler(handler);
	let dir = std::env::current_dir().unwrap().as_path().display().to_string();
	let filename = format!("{}\\{}", dir, "main.htm");
	frame.load_file(&filename);
	frame.run_app();
}

fn check_if_contains_red(filename: &str, red: i32, green: i32, blue: i32) -> i32 {
    let screenshot = image::open(filename).unwrap().to_rgb();
    let mut contains_red_pixel = 0;
    let mut index = 0;
    for pixel in screenshot.pixels() {
        index += 1;
        if pixel[0] >= red as u8 && pixel[1] <= green as u8 && pixel[2] <= blue as u8 {
            let (width, height) = screenshot.dimensions();
            let y = index / width;
            let x = index % width;
            contains_red_pixel = index as i32;
            println!("pixel {} is red at coords {}, {}", index - 1, x, y);
            break;
        }
    }
    return contains_red_pixel;
}

struct EventHandler;

impl EventHandler {
	fn find_red_pixel(&self, filename: String, red: i32, green: i32, blue: i32) -> sciter::Value {
		sciter::Value::from(check_if_contains_red(filename.as_str(), red, green, blue))
	}
}

impl sciter::EventHandler for EventHandler {
	dispatch_script_call! (
		fn find_red_pixel(String, i32, i32, i32);
	);
}