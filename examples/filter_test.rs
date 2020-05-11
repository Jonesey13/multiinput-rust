extern crate multiinput;

use multiinput::*;
fn main() {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Joysticks(XInputInclude::True));
    manager.register_devices(DeviceType::Keyboards);
    manager.register_devices(DeviceType::Mice);
    let devices = manager.get_device_list();

    //Filter to pickup events from the first keyboard only
    let keyboard = devices.keyboards.first().unwrap();
    manager.filter_devices(vec![keyboard.name.clone()]);
    //manager.unfilter_devices();

    println!("{:?}", devices);
    'outer: loop {
        if let Some(event) = manager.get_event() {
            match event {
                RawEvent::KeyboardEvent(_, KeyId::Escape, State::Pressed) => break 'outer,
                _ => (),
            }
            println!("{:?}", event);
        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    println!("Finishing");
}
