use notify_rust::Notification;

pub fn create_picker_notification(body: &str) {
    Notification::new()
        .summary("Color Picker")
        .body(body)
        .id(0)
        .sound_name("Default")
        .show()
        .unwrap_or_default();
}
