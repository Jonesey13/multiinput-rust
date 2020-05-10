use devices::{DeviceInfo, Devices, JoystickInfo, JoystickState, KeyboardInfo, MouseInfo};
use event::RawEvent;
use joystick::{garbage_vec, process_joystick_data};
use keyboard::process_keyboard_data;
use mouse::process_mouse_data;
use std::collections::VecDeque;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::mem::MaybeUninit;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use std::{mem, ptr};
use winapi::shared::hidpi::{
    HidP_GetButtonCaps, HidP_GetCaps, HidP_GetValueCaps, HidP_Input, HIDP_BUTTON_CAPS, HIDP_CAPS,
    HIDP_STATUS_SUCCESS, HIDP_VALUE_CAPS, PHIDP_BUTTON_CAPS, PHIDP_PREPARSED_DATA,
    PHIDP_VALUE_CAPS,
};
use winapi::shared::hidsdi::HidD_GetSerialNumberString;
use winapi::shared::minwindef::{INT, LPVOID, UINT};
use winapi::um::fileapi::{CreateFileW, OPEN_EXISTING};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, HANDLE, PVOID};
use winapi::um::winuser::{
    GetRawInputBuffer, GetRawInputDeviceInfoW, GetRawInputDeviceList, PRAWINPUT,
    PRAWINPUTDEVICELIST, RAWINPUT, RAWINPUTDEVICELIST, RAWINPUTHEADER, RIDI_DEVICEINFO,
    RIDI_DEVICENAME, RIDI_PREPARSEDDATA, RID_DEVICE_INFO, RIM_TYPEHID, RIM_TYPEKEYBOARD,
    RIM_TYPEMOUSE,
};

/// Follows the NEXTRAWINPUTBLOCK macro
unsafe fn next_raw_input_block(array_ptr: &mut *mut u8) {
    use std::mem::{size_of, transmute};

    // Shift by size of RAWINPUT data
    let dw_size = (*(*array_ptr as *mut RAWINPUT)).header.dwSize;
    (*array_ptr) = (*array_ptr).offset(dw_size as isize);

    // Correct for bit allignment
    let mut array_int: usize = transmute(*array_ptr);
    array_int = (array_int + size_of::<usize>() - 1) & !(size_of::<usize>() - 1);
    (*array_ptr) = transmute(array_int);
}

fn read_input_buffer(event_queue: &mut VecDeque<RawEvent>, devices: &mut Devices) {
    unsafe {
        let mut array_alloc: [u8; 16000] = MaybeUninit::uninit().assume_init();
        let mut buffer_size: UINT = 0;

        let mut numberofelements: i32 = GetRawInputBuffer(
            ptr::null_mut(),
            &mut buffer_size,
            mem::size_of::<RAWINPUTHEADER>() as UINT,
        ) as INT;

        if numberofelements as INT == -1 {
            panic!("GetRawInputBuffer Gave Error on First Call!");
        }
        buffer_size = 1024;
        numberofelements = GetRawInputBuffer(
            array_alloc.as_mut_ptr() as PRAWINPUT,
            &mut buffer_size,
            mem::size_of::<RAWINPUTHEADER>() as UINT,
        ) as INT;

        if numberofelements as INT == -1 {
            panic!("GetRawInputBuffer Gave Error on Second Call!");
        }

        let mut array_ptr = array_alloc.as_mut_ptr();

        for _ in 0..numberofelements as u32 {
            let header = (*(array_ptr as *mut RAWINPUT)).header;
            let raw_input = *(array_ptr as *mut RAWINPUT);
            next_raw_input_block(&mut array_ptr);
            let pos = match devices.device_map.get(&header.hDevice) {
                Some(item) => (*item).clone(),
                None => continue,
            };
            match raw_input.header.dwType {
                RIM_TYPEMOUSE => {
                    event_queue.extend(process_mouse_data(&raw_input.data.mouse(), pos));
                }
                RIM_TYPEKEYBOARD => {
                    event_queue.extend(process_keyboard_data(&raw_input.data.keyboard(), pos));
                }
                RIM_TYPEHID => {
                    event_queue.extend(process_joystick_data(
                        &raw_input.data.hid(),
                        pos,
                        &mut devices.joysticks[pos],
                    ));
                }
                _ => (),
            }
        }
    }
}

pub fn get_event(event_queue: &mut VecDeque<RawEvent>, devices: &mut Devices) -> Option<RawEvent> {
    if event_queue.is_empty() {
        read_input_buffer(event_queue, devices);
    }
    let event = event_queue.pop_front();
    event
}

pub fn get_joystick_state(devices: &Devices, id: usize) -> Option<JoystickState> {
    match (&devices.joysticks).get(id) {
        None => None,
        Some(joy) => Some(joy.state.clone()),
    }
}

/// Produces a Device struct containing ID's to all available raw input Devices
pub fn produce_raw_device_list(incl_360_devices: bool) -> Devices {
    let mut device_list = Devices::new();
    unsafe {
        let mut buffer: [RAWINPUTDEVICELIST; 1000] = MaybeUninit::uninit().assume_init();
        let mut num_devices: UINT = 0;
        let device_list_size = mem::size_of::<RAWINPUTDEVICELIST>();
        let mut result =
            GetRawInputDeviceList(ptr::null_mut(), &mut num_devices, device_list_size as UINT);
        if result == -1i32 as UINT {
            panic!("Failed to Get Raw Device List!");
        }
        result = GetRawInputDeviceList(
            buffer.as_mut_ptr() as PRAWINPUTDEVICELIST,
            &mut num_devices,
            device_list_size as UINT,
        );
        if result == -1i32 as UINT {
            panic!("Failed to Get Raw Device List Again!");
        }

        for pos in 0..result as usize {
            let device_ptr = (&mut buffer[pos..(pos + 1)]).as_mut_ptr() as PRAWINPUTDEVICELIST;
            let device = *device_ptr;
            let device_handle = device.hDevice;
            let device_type = device.dwType;
            let name = raw_handle_to_name(device_handle);
            let hid_handle = match raw_name_to_hid(name.clone()) {
                Ok(handle) => handle,
                Err(_) => continue,
            };
            let serial = get_serial_number(hid_handle);
            let device_info_option = get_device_info(device_handle, name, serial);
            match device_info_option {
                None => continue,
                _ => (),
            }
            let device_info = device_info_option.unwrap();
            match device_type {
                RIM_TYPEMOUSE => {
                    if let DeviceInfo::Mouse(info) = device_info {
                        device_list
                            .device_map
                            .insert(device_handle, device_list.mice.len());
                        device_list.mice.push(info);
                    } else {
                        panic!("Unreachable!");
                    }
                }
                RIM_TYPEKEYBOARD => {
                    if let DeviceInfo::Keyboard(info) = device_info {
                        device_list
                            .device_map
                            .insert(device_handle, device_list.keyboards.len());
                        device_list.keyboards.push(info);
                    } else {
                        panic!("Unreachable!");
                    }
                }
                RIM_TYPEHID => {
                    if let DeviceInfo::Joystick(info) = device_info {
                        if info.is_360_controller && !incl_360_devices {
                            continue;
                        }
                        device_list
                            .device_map
                            .insert(device_handle, device_list.joysticks.len());
                        device_list.joysticks.push(info);
                    } else {
                        panic!("Unreachable!");
                    }
                }
                _ => (),
            }
        }
    }
    device_list
}

pub unsafe fn raw_handle_to_name(device_handle: HANDLE) -> String {
    let mut name_buffer: [u16; 1024] = MaybeUninit::uninit().assume_init();
    let mut name_buffer_size: UINT = 1024;
    let result_2 = GetRawInputDeviceInfoW(
        device_handle,
        RIDI_DEVICENAME,
        name_buffer.as_mut_ptr() as LPVOID,
        &mut name_buffer_size,
    );
    if result_2 == -1i32 as UINT {
        return "Cannot obtain device name, continuing...".to_string();
    }
    let name_slice = &name_buffer[0..result_2 as usize];
    match OsString::from_wide(name_slice).into_string() {
        Ok(something) => something,
        Err(_) => panic!("String Conversion Failed"),
    }
}

pub unsafe fn raw_name_to_hid(name: String) -> Result<HANDLE, String> {
    let os_name: &OsStr = name.as_ref();
    let mut classname = os_name
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();
    let hid_handle = CreateFileW(
        classname.as_mut_ptr(),
        0,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        ptr::null_mut(),
        OPEN_EXISTING,
        0,
        ptr::null_mut(),
    );
    if hid_handle != INVALID_HANDLE_VALUE {
        return Ok(hid_handle);
    } else {
        return Err("Could not get the HID handle of device ".to_string() + &name);
    }
}

pub unsafe fn get_device_info(
    handle: HANDLE,
    name: String,
    serial: Option<String>,
) -> Option<DeviceInfo> {
    let mut data_buffer: [RID_DEVICE_INFO; 1] = MaybeUninit::uninit().assume_init();
    let mut data_buffer_size = mem::size_of::<RID_DEVICE_INFO>() as u32;
    data_buffer[0].cbSize = data_buffer_size;
    let result = GetRawInputDeviceInfoW(
        handle,
        RIDI_DEVICEINFO,
        data_buffer.as_mut_ptr() as LPVOID,
        &mut data_buffer_size,
    );
    assert!(result as INT != -1);
    let raw_info = data_buffer[0];

    return match raw_info.dwType {
        RIM_TYPEMOUSE => Some(DeviceInfo::Mouse(MouseInfo {
            name: name,
            handle: handle,
            serial: serial,
            info: raw_info,
        })),
        RIM_TYPEKEYBOARD => Some(DeviceInfo::Keyboard(KeyboardInfo {
            name: name,
            handle: handle,
            serial: serial,
            info: raw_info,
        })),
        RIM_TYPEHID => {
            if raw_info.u.hid().usUsagePage != 0x01
                || !(raw_info.u.hid().usUsage == 0x04 || raw_info.u.hid().usUsage == 0x05)
            {
                return None;
            }

            let mut preparsed_data_size: UINT = 1024;
            assert!(
                GetRawInputDeviceInfoW(
                    handle,
                    RIDI_PREPARSEDDATA,
                    ptr::null_mut(),
                    &mut preparsed_data_size
                ) == 0
            );
            let mut preparsed_data: Vec<u8> = garbage_vec(preparsed_data_size as usize);
            assert!(
                GetRawInputDeviceInfoW(
                    handle,
                    RIDI_PREPARSEDDATA,
                    preparsed_data.as_mut_ptr() as LPVOID,
                    &mut preparsed_data_size
                ) as i32
                    >= 0
            );
            let mut caps: HIDP_CAPS = MaybeUninit::uninit().assume_init();
            assert!(
                HidP_GetCaps(
                    preparsed_data.as_mut_ptr() as PHIDP_PREPARSED_DATA,
                    &mut caps
                ) == HIDP_STATUS_SUCCESS
            );

            let mut caps_length = caps.NumberInputButtonCaps;
            let mut p_button_caps: Vec<HIDP_BUTTON_CAPS> = garbage_vec(caps_length as usize);

            if caps_length != 0 {
                assert!(
                    HidP_GetButtonCaps(
                        HidP_Input,
                        p_button_caps.as_mut_ptr() as PHIDP_BUTTON_CAPS,
                        &mut caps_length,
                        preparsed_data.as_mut_ptr() as PHIDP_PREPARSED_DATA
                    ) == HIDP_STATUS_SUCCESS
                );
            }

            caps_length = caps.NumberInputValueCaps;
            let mut p_value_caps: Vec<HIDP_VALUE_CAPS> = garbage_vec(caps_length as usize);

            if caps_length != 0 {
                assert!(
                    HidP_GetValueCaps(
                        HidP_Input,
                        p_value_caps.as_mut_ptr() as PHIDP_VALUE_CAPS,
                        &mut caps_length,
                        preparsed_data.as_mut_ptr() as PHIDP_PREPARSED_DATA
                    ) == HIDP_STATUS_SUCCESS
                );
            }

            let is_360_controller = name.find("IG_") != None;

            Some(DeviceInfo::Joystick(JoystickInfo {
                name: name,
                handle: handle,
                serial: serial,
                info: raw_info,
                caps: caps,
                button_caps: p_button_caps.clone(),
                value_caps: p_value_caps.clone(),
                preparsed_data: preparsed_data,
                state: JoystickState::new(p_button_caps, p_value_caps),
                is_360_controller: is_360_controller,
            }))
        }
        _ => panic!("Unreachable!"),
    };
}

pub unsafe fn get_serial_number(handle: HANDLE) -> Option<String> {
    let mut string_buffer: [u16; 128] = [0u16; 128];
    let string_buffer_size = 256;
    let result = HidD_GetSerialNumberString(
        handle,
        string_buffer.as_mut_ptr() as PVOID,
        string_buffer_size,
    );
    let serial_string_unparsed = match OsString::from_wide(&string_buffer[0..128]).into_string() {
        Ok(something) => something,
        Err(_) => panic!("String Conversion Failed {}", result),
    };
    let mut serial_string: Option<String> = None;
    if result == 1 {
        let string_front = serial_string_unparsed.find("\0");
        if let Some(string_index) = string_front {
            if string_index > 10 {
                serial_string = Some(String::from(&serial_string_unparsed[0..string_index]));
            }
        }
    }
    serial_string
}
