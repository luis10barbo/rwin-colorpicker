use winsafe::{gui, prelude::*};

use crate::color_picker::color_picker_loop;

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

        window.spawn_new_thread(move || {
            color_picker_loop();
            Ok(())
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
