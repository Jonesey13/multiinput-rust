use winapi::*;
use user32::*;
use kernel32::*;
use event::*;
use devices::*;
use rawinput::*;

use std::ptr;
use std::mem;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::collections::VecDeque;
use std::thread;
use std::thread::JoinHandle;

use std::sync::mpsc::{
    Sender,
    Receiver,
    channel
};

enum Command {
    Register(DeviceType),
    GetEvent,
    GetJoystickState(usize),
    Finish,
    PrintDeviceList,
    GetDeviceStats,
}

/// Types of Raw Input Device
#[derive(PartialEq, Eq, Clone)]
pub enum DeviceType {
    Mice,
    Keyboards,
    Joysticks,
}

#[derive(Default)]
pub struct DeviceStats {
    pub number_of_mice: usize,
    pub number_of_keyboards: usize,
    pub number_of_joysticks: usize,
}

/// Manages Raw Input Processing
pub struct RawInputManager {
    joiner: Option<JoinHandle<()>>,
    sender: Sender<Command>,
    receiver: Receiver<Option<RawEvent>>,
    joystick_receiver: Receiver<Option<JoystickState>>,
    device_stats_receiver: Receiver<DeviceStats>,
}

impl RawInputManager {

    pub fn new() -> Result<RawInputManager, &'static str> {
        let (tx, rx) = channel();
        let (tx2, rx2) = channel();
        let (tx_joy, rx_joy) = channel();
        let (tx_stats, rx_stats) = channel();

        let joiner = thread::spawn(move || {
            let hwnd = setup_message_window();
            let mut event_queue = VecDeque::new();
            let mut devices = Devices::new();
            let mut exit = false;
            while !exit {
                match  rx.recv().unwrap() {
                    Command::Register(thing) =>
                    {devices = register_devices(hwnd, thing).unwrap();
                     tx2.send(None).unwrap();},
                    Command::GetEvent =>
                    tx2.send(get_event(&mut event_queue, &mut devices)).unwrap(),
                    Command::Finish => {exit = true;},
                    Command::GetJoystickState(id) =>
                        tx_joy.send(get_joystick_state(&devices, id)).unwrap(),
                    Command::PrintDeviceList =>
                        print_raw_device_list(&devices),
                    Command::GetDeviceStats =>
                        tx_stats.send(get_device_stats(&devices)).unwrap(),
                };
            };
        });
        Ok(RawInputManager{
            joiner: Some(joiner),
            sender: tx,
            receiver: rx2,
            joystick_receiver: rx_joy,
            device_stats_receiver: rx_stats,
        })
    }

    /// Allows Raw Input devices of type device_type to be received from the Input Manager
    pub fn register_devices(&mut self, device_type: DeviceType) {
        self.sender.send(Command::Register(device_type)).unwrap();
        self.receiver.recv().unwrap();
    }

    /// Get Event from the Input Manager
    pub fn get_event(&mut self) -> Option<RawEvent> {
        self.sender.send(Command::GetEvent).unwrap();
        self.receiver.recv().unwrap()
    }

    /// Get Joystick State from the Input Manager
    pub fn get_joystick_state(&mut self, id: usize) -> Option<JoystickState> {
        self.sender.send(Command::GetJoystickState(id)).unwrap();
        self.joystick_receiver.recv().unwrap()
    }

    /// Print List of Potential Input Devices
    pub fn print_device_list(& self) {
        self.sender.send(Command::PrintDeviceList).unwrap();
    }

    /// Get Device Stats (number of connected devices)
    pub fn get_device_stats(&self) -> DeviceStats{
        self.sender.send(Command::GetDeviceStats).unwrap();
        self.device_stats_receiver.recv().unwrap()
    }
}

impl Drop for RawInputManager {
    fn drop(&mut self) {
        self.sender.send(Command::Finish).unwrap();
        self.joiner.take().unwrap().join().unwrap();
    }
}



fn setup_message_window() -> HWND{
    let hwnd: HWND;
    unsafe{
        let hinstance = GetModuleHandleW(ptr::null());
        if hinstance == ptr::null_mut(){
            panic!("Instance Generation Failed");
        }
        let classname =
            OsStr::new("RawInput Hidden Window").encode_wide().chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        let wcex = WNDCLASSEXW{
            cbSize: (mem::size_of::<WNDCLASSEXW>()) as UINT,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hbrBackground: ptr::null_mut(),
            hCursor:  ptr::null_mut(),
            hIcon:  ptr::null_mut(),
            hIconSm:  ptr::null_mut(),
            hInstance: hinstance,
            lpfnWndProc: Some(DefWindowProcW),
            lpszClassName: classname.as_ptr(),
            lpszMenuName: ptr::null_mut(),
            style: 0,
        };
        let a = RegisterClassExW(&wcex);
        if a == 0{
	    panic!("Registering WindowClass Failed!");
        }

        hwnd = CreateWindowExW(0,
                               classname.as_ptr(),
                               classname.as_ptr(),
                               0,
                               CW_USEDEFAULT,
                               CW_USEDEFAULT,
                               CW_USEDEFAULT,
                               CW_USEDEFAULT,
                               HWND_MESSAGE,
                               ptr::null_mut(),
                               hinstance,
                               ptr::null_mut());
        if hwnd.is_null(){
            panic!("Window Creation Failed!");
        }
    }
    hwnd
}


fn register_devices( hwnd: HWND, reg_type: DeviceType,
                         ) -> Result<Devices, &'static str> {
    let mut rid_vec: Vec<RAWINPUTDEVICE> = Vec::new();
    if reg_type == DeviceType::Mice {
        let rid = RAWINPUTDEVICE {
	    usUsagePage: 1,
	    usUsage: 2,	// Mice
	    dwFlags: RIDEV_INPUTSINK,
	    hwndTarget: hwnd,
        };
        rid_vec.push(rid);
    }
    if reg_type == DeviceType::Joysticks {
        let rid = RAWINPUTDEVICE {
	    usUsagePage: 1,
	    usUsage: 4,	// Joysticks
	    dwFlags: RIDEV_INPUTSINK,
	    hwndTarget: hwnd,
        };
        rid_vec.push(rid);
    }
    if reg_type == DeviceType::Keyboards {
        let rid = RAWINPUTDEVICE {
	    usUsagePage: 1,
	    usUsage: 6,	// Keyboards
	    dwFlags: RIDEV_INPUTSINK,
	    hwndTarget: hwnd,
        };
        rid_vec.push(rid);
    }
    unsafe{
        if RegisterRawInputDevices(
            rid_vec.as_ptr(), rid_vec.len() as UINT,
            mem::size_of::<RAWINPUTDEVICE>() as UINT,
        ) == 0 {
	    return Err("Registration of Controller Failed");
        }
    }
    Ok(produce_raw_device_list())
}

/// Prints a list of all available raw input devices
fn print_raw_device_list (devices: &Devices) {;
    println!("Mice:");
    for mouse in devices.mice.clone() {
        println!("{:?}", mouse.names);
        println!("{:?}", mouse.serial);
    }
    println!("Keyboards:");
    for keyboard in devices.keyboards.clone() {
        println!("{:?}", keyboard.names);
        println!("{:?}", keyboard.serial);
    }
    println!("Hids:");
    for joystick in devices.joysticks.clone() {
        println!("{:?}", joystick.names);
        println!("{:?}", joystick.serial);
        for caps in joystick.value_caps {
            println!("{:?}", caps);
        }
    }
}

fn get_device_stats(devices: &Devices) -> DeviceStats {
    DeviceStats {
        number_of_mice: devices.mice.len(),
        number_of_keyboards: devices.keyboards.len(),
        number_of_joysticks: devices.joysticks.len(),
    }
}
