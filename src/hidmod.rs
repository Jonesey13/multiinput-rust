use winapi::*;
use event::*;
use devices::*;
use std::mem::*;
use std::mem;
use hid::*;

pub unsafe fn garbage_vec<T>(size: usize) -> Vec<T>{
    let mut v = Vec::with_capacity(size);
    v.set_len(size);
    v
}


pub fn process_joystick_data(raw_data: &RAWHID, id: usize, hid_info: &mut JoystickInfo) -> Vec<RawEvent> {
    let mut output: Vec<RawEvent> = Vec::new();
    unsafe{
        // let mut preparsed_data_size: UINT = 1024;
        // assert!(GetRawInputDeviceInfoW(header.hDevice,
        //                                       RIDI_PREPARSEDDATA,
        //                                       ptr::null_mut(),
        //                                       &mut preparsed_data_size) ==0);
        // let mut preparsed_data: Vec<u8> = garbage_vec(preparsed_data_size as usize) ;
        // assert!( GetRawInputDeviceInfoW(header.hDevice,
        //                                 RIDI_PREPARSEDDATA,
        //                                 preparsed_data.as_mut_ptr() as PHIDP_PREPARSED_DATA,
        //                                 &mut preparsed_data_size) as i32 >= 0  );
        // let mut caps: HIDP_CAPS = mem::uninitialized();
        // assert!(HidP_GetCaps(preparsed_data.as_mut_ptr() as PHIDP_PREPARSED_DATA, &mut caps) == HIDP_STATUS_SUCCESS );

        // let mut caps_length = caps.NumberInputButtonCaps;
        // let mut p_button_caps: Vec<HIDP_BUTTON_CAPS> = garbage_vec(caps_length as usize);

        // assert!(HidP_GetButtonCaps(HIDP_REPORT_TYPE::HidP_Input,
        //                           p_button_caps.as_mut_ptr() as PHIDP_BUTTON_CAPS,
        //                           &mut caps_length,
        //                           preparsed_data.as_mut_ptr() as PHIDP_PREPARSED_DATA) == HIDP_STATUS_SUCCESS);

        // caps_length = caps.NumberInputValueCaps;
        // let mut p_value_caps: Vec<HIDP_VALUE_CAPS> = garbage_vec(caps_length as usize);

        // assert!(HidP_GetValueCaps(HIDP_REPORT_TYPE::HidP_Input,
        //                           p_value_caps.as_mut_ptr() as PHIDP_VALUE_CAPS,
        //                           &mut caps_length,
        //                           preparsed_data.as_mut_ptr() as PHIDP_PREPARSED_DATA) == HIDP_STATUS_SUCCESS);

        // let mut number_of_buttons: ULONG = (p_button_caps[0].Range.UsageMax - p_button_caps[0].Range.UsageMin + 1) as ULONG;

        let button_caps = hid_info.button_caps[0].clone();
        let number_of_buttons: ULONG = (button_caps.Range().UsageMax - button_caps.Range().UsageMin + 1) as ULONG;
        let mut usage: Vec<USAGE> = garbage_vec(number_of_buttons as usize);
        let mut number_of_presses: ULONG = number_of_buttons;

	assert!(HidP_GetUsages(HIDP_REPORT_TYPE::HidP_Input,
                               button_caps.UsagePage,
                               0,
                               usage.as_mut_ptr(),
                               &mut number_of_presses,
                               hid_info.preparsed_data.as_mut_ptr() as PHIDP_PREPARSED_DATA,
		               transmute::<_, PCHAR>(raw_data.bRawData.as_ptr()),
                               raw_data.dwSizeHid
		               ) == HIDP_STATUS_SUCCESS );

        let mut button_states: Vec<bool> = vec![false; number_of_buttons as usize];
	for i in 0..number_of_presses as usize{
            button_states[(usage[i] - button_caps.Range().UsageMin) as usize] = true;
        }


        // let mut data: Vec<HIDP_DATA> = garbage_vec(number_of_buttons as usize);


        // assert!(HidP_GetData(HIDP_REPORT_TYPE::HidP_Input,
        //                      data.as_mut_ptr(),
        //                      &mut number_of_buttons,
        //                      preparsed_data.as_mut_ptr() as PHIDP_PREPARSED_DATA,
	// 	             raw_data.bRawData.as_mut_ptr() as PCHAR,
        //                      raw_data.dwSizeHid) == HIDP_STATUS_SUCCESS);

        // for i in 0..number_of_buttons as usize{
        //     println!("Index {:?}  Value {:?}", data[i].DataIndex, data[i].RawValue)


        // }

        let vec_value_caps = hid_info.value_caps.clone();

        let mut axis_states = hid_info.state.axis_states.clone();
        let mut raw_axis_states = hid_info.state.raw_axis_states.clone();
        let mut hatswitch: Option<HatSwitch> = None;


        let mut value: ULONG = mem::uninitialized();
        let mut derived_value: f64;
        for value_caps in vec_value_caps {
            let usage_index = value_caps.Range().UsageMin;
            let logical_max = value_caps.LogicalMax as f64;
            let logical_min = value_caps.LogicalMin as f64;

	    assert!(HidP_GetUsageValue(HIDP_REPORT_TYPE::HidP_Input,
                                       value_caps.UsagePage,
                                       0,
                                       usage_index,
                                       &mut value,
                                       hid_info.preparsed_data.as_mut_ptr() as PHIDP_PREPARSED_DATA,
		                       transmute::<_, PCHAR>(raw_data.bRawData.as_ptr()),
                                       raw_data.dwSizeHid
		                       ) == HIDP_STATUS_SUCCESS );
            if value as f64 > logical_max {
                derived_value = (value as f64) - (logical_max - logical_min + 1f64);
            }
            else {
                derived_value = value as f64;
            }
            derived_value = 2f64 * (derived_value - logical_min) / (logical_max - logical_min) - 1f64;
            // derived_value = value as f64;


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

        let newstate = JoystickState{ button_states: button_states,
                                      axis_states: axis_states,
                                      hatswitch: hatswitch,
                                      raw_axis_states: raw_axis_states,};
        let new_events = hid_info.state.compare_states(newstate.clone(), id);
        output.extend(new_events);
        // for event in output.clone(){
        //     println!("{:?}", event);
        // }
        hid_info.state = newstate;
	    //println!("Axis: {:x} Value: {:?}", p_value_caps[i].Range.UsageMin, value as i8);
    }
    output
}
