use notify_rust::Notification;
const NOTIF_ENABLED: bool = false;
pub fn create_picker_notification(body: &str) {
    if !NOTIF_ENABLED {
        return;
    }

    Notification::new()
        .summary("Color Picker")
        .appname("color_picker_app")
        .body(body)
        .id(0)
        .sound_name("Default")
        .show()
        .unwrap_or_default();
}
