use winapi::um::winuser::{RIDEV_INPUTSINK, RAWINPUTDEVICE, RegisterRawInputDevices};
use winapi::shared::minwindef::UINT; 
use winapi::shared::windef::HWND;
use devices::Devices;
use rawinput::produce_raw_device_list;
use manager::{XInputInclude, DeviceType};

use std::mem;

#[derive(Default)]
pub struct RawInputRegistrar {
    include_xinput: bool
}

impl RawInputRegistrar {
    pub fn new () -> Self {
        Self::default()
    }

    pub fn register_devices(&mut self, hwnd: HWND, reg_type: DeviceType,
    ) -> Result<Devices, &'static str> {
        let mut rid_vec: Vec<RAWINPUTDEVICE> = Vec::new();
        match reg_type {
            DeviceType::Mice => {
                let rid = RAWINPUTDEVICE {
	            usUsagePage: 1,
	            usUsage: 2,	// Mice
	            dwFlags: RIDEV_INPUTSINK,
	            hwndTarget: hwnd,
                };
                rid_vec.push(rid);
            },
            DeviceType::Joysticks(include_xinput) => {
                self.include_xinput = match include_xinput {
                    XInputInclude::True => true,
                    XInputInclude::False => false,
                };
                
                let rid = RAWINPUTDEVICE {
	            usUsagePage: 1,
	            usUsage: 4,	// Joysticks
	            dwFlags: RIDEV_INPUTSINK,
	            hwndTarget: hwnd,
                };
                rid_vec.push(rid);
                let rid = RAWINPUTDEVICE {
	            usUsagePage: 1,
	            usUsage: 5,	// Xbox Controllers
	            dwFlags: RIDEV_INPUTSINK,
	            hwndTarget: hwnd,
                };
                rid_vec.push(rid);
            },
            DeviceType::Keyboards => {
                let rid = RAWINPUTDEVICE {
	            usUsagePage: 1,
	            usUsage: 6,	// Keyboards
	            dwFlags: RIDEV_INPUTSINK,
	            hwndTarget: hwnd,
                };
                rid_vec.push(rid);
            }
        };
        unsafe{
            if RegisterRawInputDevices(
                rid_vec.as_ptr(), rid_vec.len() as UINT,
                mem::size_of::<RAWINPUTDEVICE>() as UINT,
            ) == 0 {
	        return Err("Registration of Controller Failed");
            }
        }
        Ok(produce_raw_device_list(self.include_xinput))
    }
    
}
