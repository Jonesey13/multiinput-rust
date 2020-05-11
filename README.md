# multiinput-rust

[Documentation](http://jonesey13.github.io/multiinput-rust/doc/multiinput/index.html)

A [windows rawinput](https://docs.microsoft.com/en-us/windows/win32/inputdev/raw-input) library for mice/keyboards/joysticks for use with rust. 

The original purpose of this library was to help me learn how the art of binding dll's to rust and to allow the use of joysticks in game development (e.g. alongside the glium library). Eventually this was used to develop games that had a separate mice for each player.

## Key Features

* Can differentiate between different keyboards/mice.
* It is intended to be single-purpose and lightweight and can be integrated with other libraries without interference (this is done by having a hidden background input window running).
* In principle this approach could support all HID devices, provide input to devices (e.g. force feedback) and should be able to break the 4 device limit on Xinput controllers.

## Known Limitations
* Some track pads are not picked up
* The application can crash if the wrong drivers are installed for a device (e.g. a joystick)
* XInput support is limited (see the docs for details)