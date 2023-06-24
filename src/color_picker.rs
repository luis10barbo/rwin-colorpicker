use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time,
};

use screenshots::{Image, Screen};
use winsafe::{GetCursorPos, POINT};

use crate::{clipboard::copy_to_clipboard, keybinds, notification::create_picker_notification};

#[derive(Clone)]
pub struct ColorPickerLoop {
    // pick_mode: bool,
    // cursor_position: Option<POINT>,
    // loop_delay: u8,
    pub pick_mode: Arc<Mutex<bool>>,
}
impl ColorPickerLoop {
    pub fn new(pick_mode: Option<Arc<Mutex<bool>>>) -> Self {
        let pick_mode = if let Some(loop_state) = pick_mode {
            loop_state
        } else {
            Arc::new(Mutex::new(false))
        };

        Self { pick_mode }
    }
    pub fn trigger_color_pick(&self, cursor_position: Option<POINT>) -> Option<()> {
        if cursor_position == None {
            return None;
        };
        let mut color: Option<Vec<u8>> = None;
        if let Some(position) = cursor_position {
            let mut current_width: u32 = position.x.try_into().unwrap();
            for screen in Screen::all().unwrap_or_default() {
                if current_width > screen.display_info.width {
                    current_width -= screen.display_info.width;
                    continue;
                }
                let capture_result = screen.capture_area(current_width as i32, position.y, 1, 1);

                if capture_result.is_err() {
                    create_picker_notification("Error capturing area");
                    break;
                };

                let capture: Image = capture_result.unwrap();

                color = Some(capture.rgba().to_owned());
                break;
            }
        };

        if let Some(color_values) = color {
            let color_rgb = &format!(
                "{}, {}, {}",
                color_values[0], color_values[1], color_values[2]
            );
            create_picker_notification(&format!("Copied color, {}!", color_rgb));

            let result = copy_to_clipboard(
                &format!(
                    "{}, {}, {}",
                    color_values[0], color_values[1], color_values[2]
                ),
                None,
            );
            if result.is_err() {
                println!("{:?}", result.unwrap_err());
            }
        };
        None
    }

    pub fn color_picker_loop(&mut self) {
        let mut cursor_position: Option<POINT> = None;
        let mut loop_delay = 200;
        loop {
            {
                let mut pick_mode = self.pick_mode.lock().unwrap();
                // log_info(pick_mode, cursor_position);

                if keybinds::color_pick_pressed() && !*pick_mode {
                    // create_picker_notification("Enabled color picker mode");

                    loop_delay = 10;
                    *pick_mode = true;
                } else if keybinds::mouse_pressed() && *pick_mode {
                    cursor_position = Some(GetCursorPos().unwrap_or_default());
                    *pick_mode = false;
                    loop_delay = 200;

                    self.trigger_color_pick(cursor_position);
                };
            }

            sleep(time::Duration::from_millis(loop_delay));
        }
    }
}

// fn log_info(pick_mode: bool, cursor_position: Option<POINT>) {
//     println!(
//         "{} {}",
//         pick_mode,
//         if let Some(pos) = cursor_position {
//             pos
//         } else {
//             POINT {
//                 ..Default::default()
//             }
//         }
//     );
// }
