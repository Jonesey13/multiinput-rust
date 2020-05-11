use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt;
use winapi::shared::hidpi::{HIDP_BUTTON_CAPS, HIDP_CAPS, HIDP_VALUE_CAPS};
use winapi::um::winnt::HANDLE;
use winapi::um::winuser::RID_DEVICE_INFO;

#[derive(Clone)]
pub struct MouseInfo {
    pub name: String,
    pub handle: HANDLE,
    pub serial: Option<String>,
    pub info: RID_DEVICE_INFO,
}

impl fmt::Debug for MouseInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mouse Info")
            .field("name", &self.name)
            .field("handle", &self.handle)
            .field("serial", &self.serial)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct MouseDisplayInfo {
    pub name: String,
    pub serial: Option<String>,
}

impl From<MouseInfo> for MouseDisplayInfo {
    fn from(mouse: MouseInfo) -> Self {
        Self {
            name: mouse.name,
            serial: mouse.serial
        }
    }
}

#[derive(Clone)]
pub struct KeyboardInfo {
    pub name: String,
    pub handle: HANDLE,
    pub serial: Option<String>,
    pub info: RID_DEVICE_INFO,
}

impl fmt::Debug for KeyboardInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Keyboard Info")
            .field("name", &self.name)
            .field("handle", &self.handle)
            .field("serial", &self.serial)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct KeyboardDisplayInfo {
    pub name: String,
    pub serial: Option<String>,
}

impl From<KeyboardInfo> for KeyboardDisplayInfo {
    fn from(keyboard: KeyboardInfo) -> Self {
        Self {
            name: keyboard.name,
            serial: keyboard.serial
        }
    }
}

#[derive(Clone)]
pub struct JoystickInfo {
    pub name: String,
    pub handle: HANDLE,
    pub serial: Option<String>,
    pub info: RID_DEVICE_INFO,
    pub caps: HIDP_CAPS,
    pub button_caps: Vec<HIDP_BUTTON_CAPS>,
    pub value_caps: Vec<HIDP_VALUE_CAPS>,
    pub preparsed_data: Vec<u8>,
    pub state: JoystickState,
    pub is_360_controller: bool,
}

impl fmt::Debug for JoystickInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Joystick Info")
            .field("name", &self.name)
            .field("handle", &self.handle)
            .field("serial", &self.serial)
            .field("360 Controller?", &self.is_360_controller)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct JoystickDisplayInfo {
    pub name: String,
    pub serial: Option<String>,
}

impl From<JoystickInfo> for JoystickDisplayInfo {
    fn from(joystick: JoystickInfo) -> Self {
        Self {
            name: joystick.name,
            serial: joystick.serial
        }
    }
}

#[derive(Clone, Debug)]
pub enum DeviceInfo {
    Mouse(MouseInfo),
    Keyboard(KeyboardInfo),
    Joystick(JoystickInfo),
}

/// Stores Names to All Raw Input Devices
#[derive(Clone)]
pub struct Devices {
    pub mice: Vec<MouseInfo>,
    pub keyboards: Vec<KeyboardInfo>,
    pub joysticks: Vec<JoystickInfo>,
    pub device_map: HashMap<HANDLE, usize>,
    pub original_device_map: HashMap<HANDLE, usize>,
}

impl Devices {
    pub fn new() -> Devices {
        Devices {
            mice: Vec::new(),
            keyboards: Vec::new(),
            joysticks: Vec::new(),
            device_map: HashMap::new(),
            original_device_map: HashMap::new(),
        }
    }
}

impl Devices {
    pub fn filter_device_map(&mut self, device_filter: HashSet<String>) {
        self.device_map = HashMap::new();

        for (pos, mouse) in self.mice.iter().enumerate() {
            if device_filter.contains(&mouse.name) {
                self.device_map.insert(mouse.handle, pos);
            }
        }
        for (pos, keyboard) in self.keyboards.iter().enumerate() {
            if device_filter.contains(&keyboard.name) {
                self.device_map.insert(keyboard.handle, pos);
            }
        }
        for (pos, joystick) in self.joysticks.iter().enumerate() {
            if device_filter.contains(&joystick.name) {
                self.device_map.insert(joystick.handle, pos);
            }
        }
    }

    pub fn reset_device_map(&mut self) {
        self.device_map = self.original_device_map.clone();
    }
}

/// Striped down version of devices fit for sharing across threads
#[derive(Clone, Debug)]
pub struct DevicesDisplayInfo {
    pub mice: Vec<MouseDisplayInfo>,
    pub keyboards: Vec<KeyboardDisplayInfo>,
    pub joysticks: Vec<JoystickDisplayInfo>,
}

impl From<Devices> for DevicesDisplayInfo {
    fn from(devices: Devices) -> Self {
        Self {
            mice: devices.mice.iter().cloned().map(|m| m.into()).collect(),
            keyboards: devices.keyboards.iter().cloned().map(|m| m.into()).collect(),
            joysticks: devices.joysticks.iter().cloned().map(|m| m.into()).collect()
        }
    }
}

#[derive(Clone, Debug)]
pub struct JoystickState {
    pub button_states: Vec<bool>,
    pub axis_states: Axes,
    pub hatswitch: Option<HatSwitch>,
    pub raw_axis_states: RawAxes,
}

impl JoystickState {
    pub fn new(
        p_button_caps: Vec<HIDP_BUTTON_CAPS>,
        p_value_caps: Vec<HIDP_VALUE_CAPS>,
    ) -> JoystickState {
        unsafe {
            let mut button_states: Vec<bool> = Vec::new();
            if p_button_caps.len() > 0 {
                let ref button_caps = p_button_caps[0];
                let number_of_buttons =
                    button_caps.u.Range().UsageMax - button_caps.u.Range().UsageMin + 1;
                for _ in 0..number_of_buttons {
                    button_states.push(false);
                }
            }
            let mut axis_states = Axes::new();
            let mut hatswitch: Option<HatSwitch> = None;
            for value_caps in p_value_caps {
                if value_caps.u.Range().UsageMin == 0x30 {
                    axis_states.x = Some(0f64);
                }
                if value_caps.u.Range().UsageMin == 0x31 {
                    axis_states.y = Some(0f64);
                }
                if value_caps.u.Range().UsageMin == 0x32 {
                    axis_states.z = Some(0f64);
                }
                if value_caps.u.Range().UsageMin == 0x33 {
                    axis_states.rx = Some(0f64);
                }
                if value_caps.u.Range().UsageMin == 0x34 {
                    axis_states.ry = Some(0f64);
                }
                if value_caps.u.Range().UsageMin == 0x35 {
                    axis_states.rz = Some(0f64);
                }
                if value_caps.u.Range().UsageMin == 0x36 {
                    axis_states.slider = Some(0f64);
                }
                if value_caps.u.Range().UsageMin == 0x39 {
                    hatswitch = Some(HatSwitch::Center);
                }
            }
            JoystickState {
                button_states: button_states,
                axis_states: axis_states,
                hatswitch: hatswitch,
                raw_axis_states: RawAxes::new(),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Axes {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
    pub rx: Option<f64>,
    pub ry: Option<f64>,
    pub rz: Option<f64>,
    pub slider: Option<f64>,
}

impl Axes {
    pub fn new() -> Axes {
        Axes {
            x: None,
            y: None,
            z: None,
            rx: None,
            ry: None,
            rz: None,
            slider: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RawAxes {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub rx: u32,
    pub ry: u32,
    pub rz: u32,
    pub slider: u32,
}

impl RawAxes {
    pub fn new() -> RawAxes {
        RawAxes {
            x: 0u32,
            y: 0u32,
            z: 0u32,
            rx: 0u32,
            ry: 0u32,
            rz: 0u32,
            slider: 0u32,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HatSwitch {
    Center,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}
