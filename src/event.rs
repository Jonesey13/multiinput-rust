use devices::{JoystickState, HatSwitch};

/// State of a Key or Button
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum State {
    Pressed,
    Released,
}

/// Key Identifier
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum KeyId {
    Escape,
    Return,
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Space,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Shift,
    LeftCtrl,
    RightCtrl,
    LeftAlt,
    RightAlt,
    CapsLock,
    Pause,
    PageUp,
    PageDown,
    PrintScreen,
    Insert,
    End,
    Home,
    Delete,
    Add,
    Subtract,
    Multiply,
    Separator,
    Decimal,
    Divide,
    BackTick
}

/// Mouse Buttons
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
    RX,
    RY,
    RZ,
}


/// Event types
///
/// The usize entry acts as a device ID unique to each DeviceType (Mouse, Keyboard, Hid).
/// Keyboard press events repeat when a key is held down.
#[derive(Clone, Debug)]
pub enum RawEvent {
    MouseButtonEvent(usize,MouseButton,State),
    MouseMoveEvent(usize,i32,i32),
    MouseWheelEvent(usize,f32),
    KeyboardEvent(usize,KeyId,State),
    JoystickButtonEvent(usize,usize,State),
    JoystickAxisEvent(usize,Axis,f64),
    JoystickHatSwitchEvent(usize,HatSwitch),
}


impl JoystickState {
    pub fn compare_states(&self, other_state: JoystickState, id: usize) -> Vec<RawEvent> {
        let mut output: Vec<RawEvent> = Vec::new();
        for (index, (&press_state, _)) in self.button_states.iter()
            .zip(other_state.button_states.iter()).enumerate()
            .filter(|&(_, (&a, &b))| a != b) {
            output.push(RawEvent::JoystickButtonEvent(
                id,
                index,
                if press_state { State::Released } else { State::Pressed }));
        }
        // for index in 0..self.button_states.len() {
        //     if self.button_states[index] == true
        //         && other_state.button_states[index] == false {
        //         output.push(RawEvent::JoystickButtonEvent(id,index,State::Released));
        //     }
        //     if self.button_states[index] == false
        //         && other_state.button_states[index] == true {
        //         output.push(RawEvent::JoystickButtonEvent(id,index,State::Pressed));
        //     }
        // }
        if self.raw_axis_states.x != other_state.raw_axis_states.x {
            if let Some(value) = other_state.axis_states.x {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::X, value));
            }
        }
        if self.raw_axis_states.y != other_state.raw_axis_states.y {
            if let Some(value) = other_state.axis_states.y {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::Y, value));
            }
        }
        if self.raw_axis_states.z != other_state.raw_axis_states.z {
            if let Some(value) = other_state.axis_states.z {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::Z, value));
            }
        }
        if self.raw_axis_states.rx != other_state.raw_axis_states.rx {
            if let Some(value) = other_state.axis_states.rx {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::RX, value));
            }
        }
        if self.raw_axis_states.ry != other_state.raw_axis_states.ry {
            if let Some(value) = other_state.axis_states.ry {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::RY, value));
            }
        }
        if self.raw_axis_states.rz != other_state.raw_axis_states.rz {
            if let Some(value) = other_state.axis_states.rz {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::RZ, value));
            }
        }
        if let Some(value_other) = other_state.hatswitch {
            if let Some(value_self) = self.hatswitch.clone() {
                if value_self != value_other {
                    output.push(RawEvent::JoystickHatSwitchEvent(id,value_other));
                }
            }
        }
        output
    }
}
