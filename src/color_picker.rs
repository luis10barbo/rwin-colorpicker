use std::{thread::sleep, time};

use winsafe::{GetCursorPos, POINT};

use crate::keybinds;

pub fn color_picker_loop() {
    let mut pick_mode = false;
    let mut cursor_position: Option<POINT> = None;
    loop {
        println!(
            "{} {}",
            pick_mode,
            if let Some(pos) = cursor_position {
                pos
            } else {
                POINT {
                    ..Default::default()
                }
            }
        );

        if keybinds::color_pick_pressed() && !pick_mode {
            pick_mode = true;
        } else if keybinds::mouse_pressed() && pick_mode {
            cursor_position = Some(GetCursorPos().unwrap_or_default());
            pick_mode = false;
        };

        sleep(time::Duration::from_millis(200));
    }
}
