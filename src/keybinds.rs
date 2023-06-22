use winsafe::{co::VK, GetAsyncKeyState};

pub fn color_pick_pressed() -> bool {
    GetAsyncKeyState(VK::CHAR_P) && GetAsyncKeyState(VK::MENU) && GetAsyncKeyState(VK::SHIFT)
}

pub fn mouse_pressed() -> bool {
    GetAsyncKeyState(VK::LBUTTON)
}
