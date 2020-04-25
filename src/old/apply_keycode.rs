use winit::event::VirtualKeyCode;

use crate::widgets::text_box::TextBox;

pub fn apply_keycode(text_box: &mut TextBox, keycode: VirtualKeyCode) {
    use VirtualKeyCode::*;
    match keycode {
        Back => text_box.pop(),
        _ => (),
    }
}
