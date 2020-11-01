extern crate image;
extern crate sciter;

fn main() {
	let mut frame = sciter::Window::new();
	let dir = std::env::current_dir().unwrap().as_path().display().to_string();
	let filename = format!("{}\\{}", dir, "main.htm");
	frame.load_file(&filename);
	frame.run_app();
}

fn check_if_contains_red(filename: &str, red: u8, green: u8, blue: u8) -> bool {
    let screenshot = image::open(filename).unwrap().to_rgb();
    let mut contains_red_pixel = false;
    for pixel in screenshot.pixels() {
        if pixel[0] >= red && pixel[1] <= green && pixel[2] <= blue {
            contains_red_pixel = true;
            break;
        }
    }
    return contains_red_pixel;
}
