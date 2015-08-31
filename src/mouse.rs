use winapi::*;
use event::*;
use std::mem::*;

// #[repr(C)] #[derive(Clone, Copy, Debug)]
// pub struct RAWMOUSEMOD {
//     pub usFlags: USHORT,
//     pub memory_padding: USHORT, // 16bit Padding for 32bit align in following union
//     pub usButtonFlags: USHORT,
//     pub usButtonData: USHORT,
//     pub ulRawButtons: ULONG,
//     pub lLastX: LONG,
//     pub lLastY: LONG,
//     pub ulExtraInformation: ULONG,
// }


pub fn process_mouse_data(raw_data: &RAWMOUSE, id: usize) -> Vec<RawEvent> {
    let cursor = (raw_data.lLastX, raw_data.lLastY);
    let buttons = raw_data.usButtonFlags;
    let mut output: Vec<RawEvent> = Vec::new();
    if buttons & RI_MOUSE_LEFT_BUTTON_DOWN != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Left, State::Pressed ));
    }
    if buttons & RI_MOUSE_LEFT_BUTTON_UP != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Left, State::Released ));
    }
    if buttons & RI_MOUSE_RIGHT_BUTTON_DOWN != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Right, State::Pressed ));
    }
    if buttons & RI_MOUSE_RIGHT_BUTTON_UP != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Right, State::Released ));
    }
    if buttons & RI_MOUSE_MIDDLE_BUTTON_DOWN != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Middle, State::Pressed ));
    }
    if buttons & RI_MOUSE_MIDDLE_BUTTON_UP != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Middle, State::Released ));
    }
    if buttons & 0x0040 != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Button4, State::Pressed ));
    }
    if buttons & 0x0080 != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Button4, State::Released ));
    }
    if buttons & 0x0100 != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Button5, State::Pressed ));
    }
    if buttons & 0x0200 != 0{
        output.push(RawEvent::MouseButtonEvent(id, MouseButton::Button5, State::Released ));
    }
    if buttons & RI_MOUSE_WHEEL != 0{
        let wheel_data = raw_data.usButtonData;
        let wheel_value = unsafe{(transmute_copy::<u16,i16>(&wheel_data) as f32)/120f32};
        output.push(RawEvent::MouseWheelEvent(id, wheel_value));
    }
    if (cursor.0 != 0) || (cursor.1 != 0) {
        output.push(RawEvent::MouseMoveEvent(id, cursor.0, cursor.1));
    }
    output
}
