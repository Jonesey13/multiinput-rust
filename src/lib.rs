/*!
rawinput library for rust development on windows

# Usage Example

```no_run
extern crate rawinput;
use rawinput::*;
use rawinput::RawEvent::*;
fn main() {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::All);
    'outer: loop{
        if let Some(event) = manager.get_event(){
            match event{
                KeyboardEvent(id,  KeyId::Return, State::Pressed)
                               => println!("Keyboard Number {:?} Pressed Return", id),
                KeyboardEvent(id,  KeyId::Escape, State::Pressed)
                               => break 'outer,
                _ => (),
            }
        }
    }
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
mod hidmod;
mod rawinput;
mod keyboard;
mod devices;
pub mod manager;

pub use rawinput::*;
pub use event::*;
pub use manager::*;
