use event::{MouseButton, RawEvent, State};
use std::mem::transmute_copy;
use winapi::um::winuser::{
    RAWMOUSE, RI_MOUSE_BUTTON_4_DOWN, RI_MOUSE_BUTTON_4_UP, RI_MOUSE_BUTTON_5_DOWN,
    RI_MOUSE_BUTTON_5_UP, RI_MOUSE_LEFT_BUTTON_DOWN, RI_MOUSE_LEFT_BUTTON_UP,
    RI_MOUSE_MIDDLE_BUTTON_DOWN, RI_MOUSE_MIDDLE_BUTTON_UP, RI_MOUSE_RIGHT_BUTTON_DOWN,
    RI_MOUSE_RIGHT_BUTTON_UP, RI_MOUSE_WHEEL,
};

pub fn process_mouse_data(raw_data: &RAWMOUSE, id: usize) -> Vec<RawEvent> {
    let cursor = (raw_data.lLastX, raw_data.lLastY);
    let buttons = raw_data.usButtonFlags;
    let mut output: Vec<RawEvent> = Vec::new();
    if buttons & RI_MOUSE_LEFT_BUTTON_DOWN != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Left,
            State::Pressed,
        ));
    }
    if buttons & RI_MOUSE_LEFT_BUTTON_UP != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Left,
            State::Released,
        ));
    }
    if buttons & RI_MOUSE_RIGHT_BUTTON_DOWN != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Right,
            State::Pressed,
        ));
    }
    if buttons & RI_MOUSE_RIGHT_BUTTON_UP != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Right,
            State::Released,
        ));
    }
    if buttons & RI_MOUSE_MIDDLE_BUTTON_DOWN != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Middle,
            State::Pressed,
        ));
    }
    if buttons & RI_MOUSE_MIDDLE_BUTTON_UP != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Middle,
            State::Released,
        ));
    }
    if buttons & RI_MOUSE_BUTTON_4_DOWN != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Button4,
            State::Pressed,
        ));
    }
    if buttons & RI_MOUSE_BUTTON_4_UP != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Button4,
            State::Released,
        ));
    }
    if buttons & RI_MOUSE_BUTTON_5_DOWN != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Button5,
            State::Pressed,
        ));
    }
    if buttons & RI_MOUSE_BUTTON_5_UP != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Button5,
            State::Released,
        ));
    }
    if buttons & RI_MOUSE_WHEEL != 0 {
        let wheel_data = raw_data.usButtonData;
        let wheel_value = unsafe { (transmute_copy::<u16, i16>(&wheel_data) as f32) / 120f32 };
        output.push(RawEvent::MouseWheelEvent(id, wheel_value));
    }
    if (cursor.0 != 0) || (cursor.1 != 0) {
        output.push(RawEvent::MouseMoveEvent(id, cursor.0, cursor.1));
    }
    output
}
