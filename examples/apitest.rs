extern crate multiinput;
extern crate time;

use multiinput::*;
use multiinput::RawEvent::*;

fn main(){
    //print_raw_device_list(devices.clone());
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Mice);
    manager.register_devices(DeviceType::Keyboards);
    manager.register_devices(DeviceType::Joysticks);

    print_raw_device_list();
    let start_time = time::precise_time_s();
    let mut current_time = time::precise_time_s() - start_time;
    while current_time < 10f64{
        while let Some(event) = manager.get_event(){
            println!("{:?}", event);
        }
        current_time = time::precise_time_s() - start_time;
    }
    println!("Finishing");
}
