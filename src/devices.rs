use winapi::um::winuser::RID_DEVICE_INFO;
use winapi::shared::hidpi::{HIDP_VALUE_CAPS, HIDP_CAPS, HIDP_BUTTON_CAPS};
use winapi::um::winnt::HANDLE;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MouseInfo {
    pub names: Vec<String>,
    pub handles: Vec<HANDLE>,
    pub serial: Option<String>,
    pub info: RID_DEVICE_INFO,
}

#[derive(Clone)]
pub struct KeyboardInfo {
    pub names: Vec<String>,
    pub handles: Vec<HANDLE>,
    pub serial: Option<String>,
    pub info: RID_DEVICE_INFO,
}

#[derive(Clone)]
pub struct JoystickInfo {
    pub names: Vec<String>,
    pub handles: Vec<HANDLE>,
    pub serial: Option<String>,
    pub info: RID_DEVICE_INFO,
    pub caps: HIDP_CAPS,
    pub button_caps: Vec<HIDP_BUTTON_CAPS>,
    pub value_caps: Vec<HIDP_VALUE_CAPS>,
    pub preparsed_data: Vec<u8>,
    pub state: JoystickState,
    pub is_360_controller: bool
}

#[derive(Clone)]
pub enum DeviceInfo {
    Mouse(MouseInfo),
    Keyboard(KeyboardInfo),
    Joystick(JoystickInfo),
}

/// Stores Names to All Raw Input Devices
#[derive(Clone)]
pub struct Devices{
    pub mice: Vec<MouseInfo>,
    pub keyboards: Vec<KeyboardInfo>,
    pub joysticks: Vec<JoystickInfo>,
    pub device_map: HashMap<HANDLE, usize>,
}

impl Devices{
    pub fn new() -> Devices {
        Devices{ mice: Vec::new(),
                 keyboards: Vec::new(),
                 joysticks: Vec::new(),
                 device_map: HashMap::new(),
        }
    }

    pub fn unique(&mut self) {
        let old_devices = self.clone();
        let mut new_devices = Devices::new();
        for mouse in old_devices.mice {
            if !(mouse.names[0].find("RDP") == None) {
                continue;
            }
            if !(mouse.names[0].find("Virtual") == None) {
                continue;
            }
            let handles = mouse.handles.clone();
            let serial_opt = mouse.serial.clone();
            let mut pos_opt: Option<usize> = None;
            if let Some(_) = serial_opt.clone() {
                pos_opt = new_devices.mice.iter().position(|x| x.serial == serial_opt);
            }
            if let Some(pos) = pos_opt {
                new_devices.mice[pos].names.extend(mouse.names.clone());
                new_devices.mice[pos].handles.extend(handles.clone());
                for handle in handles {
                    new_devices.device_map.insert(handle,pos);
                }
            }
            else {
                for handle in handles {
                    new_devices.device_map.insert(handle, new_devices.mice.len());
                }
                new_devices.mice.push(mouse);
            }
        }
        for keyboard in old_devices.keyboards {
            if !(keyboard.names[0].find("RDP") == None) {
                continue;
            }
            if !(keyboard.names[0].find("Virtual") == None) {
                continue;
            }
            let handles = keyboard.handles.clone();
            let serial_opt = keyboard.serial.clone();
            let mut pos_opt: Option<usize> = None;
            if let Some(_) = serial_opt.clone() {
                pos_opt = new_devices.keyboards.iter().position(|x| x.serial == serial_opt);
            }
            if let Some(pos) = pos_opt {
                new_devices.keyboards[pos].names.extend(keyboard.names.clone());
                new_devices.keyboards[pos].handles.extend(handles.clone());
                for handle in handles {
                    new_devices.device_map.insert(handle,pos);
                }
            }
            else {
                for handle in handles {
                    new_devices.device_map.insert(handle, new_devices.keyboards.len());
                }
                new_devices.keyboards.push(keyboard);
            }
        }
        for joystick in old_devices.joysticks {
            let handles = joystick.handles.clone();
            for handle in handles {
                new_devices.device_map.insert(handle, new_devices.joysticks.len());
            }
            new_devices.joysticks.push(joystick);
        }
        *self = new_devices;
    }
}

#[derive(Clone,Debug)]
pub struct JoystickState {
    pub button_states: Vec<bool>,
    pub axis_states: Axes,
    pub hatswitch: Option<HatSwitch>,
    pub raw_axis_states: RawAxes,
}

impl JoystickState {
    pub fn new (p_button_caps: Vec<HIDP_BUTTON_CAPS>, p_value_caps: Vec<HIDP_VALUE_CAPS>) -> JoystickState {
        unsafe{
            let mut button_states: Vec<bool> = Vec::new();
            if p_button_caps.len() > 0 {
                let ref button_caps = p_button_caps[0];
                let number_of_buttons = button_caps.u.Range().UsageMax - button_caps.u.Range().UsageMin + 1;
                for _ in 0..number_of_buttons{
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
            JoystickState{
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
        Axes{
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
    pub slider: u32
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
            slider: 0u32
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
