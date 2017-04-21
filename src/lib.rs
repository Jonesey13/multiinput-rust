/*!
rawinput library for rust development on windows

# Usage Example

```no_run
extern crate multiinput;

use multiinput::*;
fn main() {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Joysticks(XInputInclude::True);
    manager.register_devices(DeviceType::Keyboards);
    manager.register_devices(DeviceType::Mice);
    'outer: loop{
        if let Some(event) = manager.get_event(){
            match event{
                RawEvent::KeyboardEvent(_,  KeyId::Escape, State::Pressed)
                    => break 'outer,
                _ => (),
            }
            println!("{:?}", event);
        }
    }
    println!("Finishing");
}
```
*/

extern crate libc;
extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate hid;

mod mouse;
pub mod event;
mod joystick;
mod rawinput;
mod keyboard;
pub mod devices;
pub mod manager;
mod registrar;

pub use event::*;
pub use manager::*;
pub use devices::*;
