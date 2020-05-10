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

extern crate winapi;

pub mod devices;
pub mod event;
mod joystick;
mod keyboard;
pub mod manager;
mod mouse;
mod rawinput;
mod registrar;

pub use devices::*;
pub use event::*;
pub use manager::*;
