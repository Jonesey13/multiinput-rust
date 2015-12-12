# multiinput-rust
An rawinput library for mice/keyboards/joysticks for use with rust. Atm this only works on windows. The primary purpose of this library is to help me learn how the art of binding dll's to rust and to allow the use of joysticks in game development (e.g. alongside the glium library). While SDL/SFML for rust also offers these capabilities there are some differences:

* This library can differentiate between different keyboards/mice.
* It is intended to be single-purpose and lightweight and can be integrated with other libraries (e.g. glium) without interference.
* In principle the rawinput technique can support all HID devices, provide input to devices (e.g. force feedback) and should be able to break the 4 device limit on Xinput devices.

I would like to warn people that I am an inexperienced hobbyist and should not be trusted. Please post any issues you find with this library!

[Documentation](http://jonesey13.github.io/multiinput-rust/doc/multiinput/index.html)
