use std::{thread::sleep, time};

use winsafe::{gui, prelude::*, GetCursorPos, POINT};

use crate::keybinds;

#[derive(Clone)]
pub struct ColorPicker {
    pub window: gui::WindowMain,
    pub btn_color_pick: gui::Button,
}

impl ColorPicker {
    pub fn new() -> Self {
        let window = gui::WindowMain::new(gui::WindowMainOpts {
            title: String::from("Color Picker RUST"),
            ..Default::default()
        });
        let btn_color_pick = gui::Button::new(
            &window,
            gui::ButtonOpts {
                text: String::from("Pick a Color"),
                ..Default::default()
            },
        );
        let color_picker = Self {
            window,
            btn_color_pick,
        };

        color_picker.attach_btn_events();
        color_picker.attach_key_events();
        color_picker
    }
    pub fn attach_btn_events(&self) {
        self.change_title(String::from("Picking Color"));
    }
    pub fn attach_key_events(&self) {
        let window = self.window.clone();

        window.spawn_new_thread({
            // let window = window.clone();
            let mut pick_mode = false;
            let mut cursor_position: Option<POINT> = None;
            move || loop {
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
                    cursor_position = Some(GetCursorPos()?);
                    pick_mode = false;
                };

                sleep(time::Duration::from_millis(200));
            }
        });
    }
    pub fn change_title(&self, new_title: String) {
        let window = self.window.clone();
        self.btn_color_pick.on().bn_clicked(move || {
            window.hwnd().SetWindowText(&new_title)?;
            Ok(())
        });
    }
}
