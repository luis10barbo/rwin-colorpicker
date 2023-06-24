use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time,
};

use winsafe::{
    gui::{self, WindowMain},
    prelude::*,
};

use crate::color_picker::ColorPickerLoop;

#[derive(Clone)]
pub struct ColorPicker {
    pub window: gui::WindowMain,
    pub btn_color_pick: gui::Button,
    pub pick_mode: Arc<Mutex<bool>>,
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
        let is_looping = Arc::new(Mutex::new(false));
        let pick_mode = Arc::new(Mutex::new(false));
        let color_picker = Self {
            window,
            btn_color_pick,
            pick_mode,
        };

        color_picker.attach_btn_events();
        color_picker.attach_key_events();
        color_picker
    }
    pub fn attach_btn_events(&self) {
        let pick_mode = Arc::clone(&self.pick_mode);
        self.btn_color_pick.on().bn_clicked(move || {
            //
            // change_title(window.clone(), String::from("Picking Color"));
            sleep(time::Duration::from_millis(200));
            println!("color pick button, getting pick_mode lock");
            let mut pick_mode = pick_mode.lock().unwrap();
            println!("got pick_mode");
            *pick_mode = true;
            Ok(())
        });
    }
    pub fn attach_key_events(&self) {
        let window = self.window.clone();
        let pick_mode = Arc::clone(&self.pick_mode);
        window.spawn_new_thread(move || {
            let mut color_picker_loop = ColorPickerLoop::new(Some(pick_mode));
            color_picker_loop.color_picker_loop();
            Ok(())
        });
        window.on().wm_key_down(move |key| {
            //
            println!("key down: {:?}", key.vkey_code);
            Ok(())
        })
    }
}
pub fn change_title(window: WindowMain, new_title: String) {
    let window = window.clone();
    window.hwnd().SetWindowText(&new_title);
}
