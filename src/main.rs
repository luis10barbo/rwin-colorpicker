// #![windows_subsystem = "windows"]
mod color_picker;
mod keybinds;
mod main_window;

use main_window::ColorPicker;
use winsafe::co;

fn main() {
    let color_picker = ColorPicker::new();
    if let Err(e) = color_picker.window.run_main(Some(co::SW::SHOW)) {
        eprintln!("{}", e);
    }
}
