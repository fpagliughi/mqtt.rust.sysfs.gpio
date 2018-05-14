# mqtt-sysfs-gpio
An example MQTT Rust application to monitor and control Linux GPIO pins via MQTT.

This uses the Rust SYSFS GPIO and Paho Rust crates to build a small application that monitors a selection of GPIO pins and publishes a message each time the state of one of the pins changes.

**rust-sysfs-gpio**
https://github.com/rust-embedded/rust-sysfs-gpio

**Paho MQTT Rust Client**
https://github.com/eclipse/paho.mqtt.rust

This initial implementation monitors changes to the state of a list of input pins, as requested by the user from the command line, then publishes a message to an MQTT broker each time a state change is detected.

The application is quite literally a simple combination of an example program from each of those projects: the _tokio_ example app from **rust-sysfs-gpio** and _async_publish_ from the Paho MQTT client.

The next version will also go in the other direction - accepting commands via MQTT and setting output pins according to the received commands.

This application was developed and tested on an UDOO NEO board (https://www.udoo.org/udoo-neo/), although it should work on any Linux system that can export GPIO pins to user space via _sysfs_, such as an RPi, BeagleBone, etc. Please report an issue if it fails on any particular hardware. 