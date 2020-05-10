use devices::{HatSwitch, JoystickInfo, JoystickState};
use event::RawEvent;
use std::mem::{transmute, MaybeUninit};
use winapi::shared::hidpi::{
    HidP_GetUsageValue, HidP_GetUsages, HidP_Input, HIDP_STATUS_INCOMPATIBLE_REPORT_ID,
    HIDP_STATUS_SUCCESS, HIDP_STATUS_INVALID_REPORT_LENGTH, HIDP_STATUS_INVALID_REPORT_TYPE, PHIDP_PREPARSED_DATA,
    HIDP_STATUS_BUFFER_TOO_SMALL, HIDP_STATUS_INVALID_PREPARSED_DATA, HIDP_STATUS_USAGE_NOT_FOUND
};
use winapi::shared::hidusage::USAGE;
use winapi::shared::ntdef::{LONG, PCHAR, ULONG};
use winapi::um::winuser::RAWHID;

pub unsafe fn garbage_vec<T>(size: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(size);
    v.set_len(size);
    v
}

pub fn process_joystick_data(
    raw_data: &RAWHID,
    id: usize,
    hid_info: &mut JoystickInfo,
) -> Vec<RawEvent> {
    let mut output: Vec<RawEvent> = Vec::new();
    unsafe {
        let mut button_states: Vec<bool> = vec![];
        if let Some(button_caps) = hid_info.button_caps.iter().nth(0) {
            let number_of_buttons: ULONG =
                (button_caps.u.Range().UsageMax - button_caps.u.Range().UsageMin + 1) as ULONG;
            let mut usage: Vec<USAGE> = garbage_vec(number_of_buttons as usize);
            let mut number_of_presses: ULONG = number_of_buttons;

            let status = 
                HidP_GetUsages(
                    HidP_Input,
                    button_caps.UsagePage,
                    0,
                    usage.as_mut_ptr(),
                    &mut number_of_presses,
                    hid_info.preparsed_data.as_mut_ptr() as PHIDP_PREPARSED_DATA,
                    transmute::<_, PCHAR>(raw_data.bRawData.as_ptr()),
                    raw_data.dwSizeHid
                );

            assert!(status != HIDP_STATUS_INVALID_REPORT_LENGTH, "Invalid Report Length!");
            assert!(status != HIDP_STATUS_INVALID_REPORT_TYPE, "Invalid Report Type!");
            assert!(status != HIDP_STATUS_BUFFER_TOO_SMALL, "Status Buffer Too Small!");
            assert!(status != HIDP_STATUS_INCOMPATIBLE_REPORT_ID, "Incompatible Report ID!");
            assert!(status != HIDP_STATUS_INVALID_PREPARSED_DATA, "Invalid Preparsed Data!");
            assert!(status != HIDP_STATUS_USAGE_NOT_FOUND, "Usage Not Found!");

            button_states = vec![false; number_of_buttons as usize];
            for i in 0..number_of_presses as usize {
                button_states[(usage[i] - button_caps.u.Range().UsageMin) as usize] = true;
            }
        }

        let vec_value_caps = hid_info.value_caps.clone();

        let mut axis_states = hid_info.state.axis_states.clone();
        let mut raw_axis_states = hid_info.state.raw_axis_states.clone();
        let mut hatswitch: Option<HatSwitch> = None;

        let mut value: ULONG = MaybeUninit::uninit().assume_init();
        let mut derived_value: f64;
        for value_caps in vec_value_caps {
            let usage_index = value_caps.u.Range().UsageMin;

            let mut logical_max = value_caps.LogicalMax;
            let mut logical_min = value_caps.LogicalMin;

            // Xbox Axes
            if logical_max == -1 && logical_min == 0 && hid_info.is_360_controller {
                logical_max = 65535;
                logical_min = 0;
            }

            let usage_value_result = HidP_GetUsageValue(
                HidP_Input,
                value_caps.UsagePage,
                0,
                usage_index,
                &mut value,
                hid_info.preparsed_data.as_mut_ptr() as PHIDP_PREPARSED_DATA,
                transmute::<_, PCHAR>(raw_data.bRawData.as_ptr()),
                raw_data.dwSizeHid,
            );
            // If the usage does not match the usage page reported by the device we ignore the result
            // (see https://github.com/Jonesey13/multiinput-rust/issues/3)
            assert!(
                (usage_value_result == HIDP_STATUS_SUCCESS)
                    || (usage_value_result == HIDP_STATUS_INCOMPATIBLE_REPORT_ID)
            );
            if value as i32 > logical_max {
                derived_value = (value as i32 - (logical_max - logical_min + 1)) as f64;
            } else {
                derived_value = value as f64;
            }
            derived_value = 2f64 * (derived_value - logical_min as f64)
                / (logical_max - logical_min) as f64
                - 1f64;
            if usage_index == 0x30 {
                axis_states.x = Some(derived_value);
                raw_axis_states.x = value;
            }
            if usage_index == 0x31 {
                axis_states.y = Some(-derived_value);
                raw_axis_states.y = value;
            }
            if usage_index == 0x32 {
                axis_states.z = Some(-derived_value);
                raw_axis_states.z = value;
            }
            if usage_index == 0x33 {
                axis_states.rx = Some(derived_value);
                raw_axis_states.rx = value;
            }
            if usage_index == 0x34 {
                axis_states.ry = Some(derived_value);
                raw_axis_states.ry = value;
            }
            if usage_index == 0x35 {
                axis_states.rz = Some(derived_value);
                raw_axis_states.rz = value;
            }
            if usage_index == 0x36 {
                axis_states.slider = Some(derived_value);
                raw_axis_states.slider = value;
            }
            if usage_index == 0x39 {
                hatswitch = match value as LONG - value_caps.LogicalMin {
                    0 => Some(HatSwitch::Up),
                    1 => Some(HatSwitch::UpRight),
                    2 => Some(HatSwitch::Right),
                    3 => Some(HatSwitch::DownRight),
                    4 => Some(HatSwitch::Down),
                    5 => Some(HatSwitch::DownLeft),
                    6 => Some(HatSwitch::Left),
                    7 => Some(HatSwitch::UpLeft),
                    _ => Some(HatSwitch::Center),
                };
            }
        }

        let newstate = JoystickState {
            button_states: button_states,
            axis_states: axis_states,
            hatswitch: hatswitch,
            raw_axis_states: raw_axis_states,
        };
        let new_events = hid_info.state.compare_states(newstate.clone(), id);
        output.extend(new_events);
        hid_info.state = newstate;
    }
    output
}
