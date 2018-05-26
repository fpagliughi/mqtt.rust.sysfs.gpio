// main.rs
//
// The main source for the 'mqtt-rust-sysfs-gpio' application.
// This tests the use of the sysfs_gpio crate for interrupting on GPIO 
// pin state changes.
//

extern crate futures;
extern crate sysfs_gpio;
extern crate tokio_core;
extern crate paho_mqtt as mqtt;

use std::{env, process};
use std::sync::Arc;
use sysfs_gpio::{Direction, Edge, Pin};
use tokio_core::reactor::Core;
use futures::{Future, Stream};

// --------------------------------------------------------------------------
// Uses Tokio in the gpio crate to create a stream of input events. 
// For each one we emit an MQTT message using the pin number in the topic 
// and the value (as UTF-8 '1' or '0') as the payload.

fn stream(cli: mqtt::AsyncClient, pin_nums: Vec<u64>) -> sysfs_gpio::Result<()> {
    let pins: Vec<_> = pin_nums.iter().map(|&p| (p, Pin::new(p))).collect();
    let mut l = Core::new()?;
    let handle = l.handle();
    let cli = Arc::new(cli);

    for &(i, ref pin) in pins.iter() {
        pin.export()?;
        pin.set_direction(Direction::In)?;
        pin.set_edge(Edge::BothEdges)?;

        let cli_cb = cli.clone();

        handle.spawn(pin.get_value_stream(&handle)?
            .for_each(move |val| {
                let topic = format!("events/gpio/in/{}", i);
                let body = format!("{}", val);

                println!("Pin {} changed value to {}", i, val);

                let msg = mqtt::Message::new(&topic, body, 1);
                cli_cb.publish(msg);

                Ok(())
            })
            .map_err(|_| ()));
    }

    // Wait forever for events
    loop {
        l.turn(None)
    }
}

// --------------------------------------------------------------------------
// Main parses the command line for the list of pins to monitor, establishes
// a connection to the MQTT broker, and starts the stream running.

fn main() {
    let pins: Vec<u64> = env::args().skip(1)
            .map(|a| a.parse().expect("Pins must be specified as integers"))
            .collect();

    if pins.is_empty() {
        println!("Usage: ./tokio <pin> [pin ...]");
        process::exit(1);
    } 

    // Create an MQTT client & define connect options
    let cli = mqtt::AsyncClient::new("tcp://localhost:1883").unwrap_or_else(|err| {
        println!("Error creating the MQTT client: {}", err);
        process::exit(1);
    });

    let conn_opts = mqtt::ConnectOptions::new();

    // Connect and wait for it to complete or fail
    if let Err(e) = cli.connect(conn_opts).wait() {
        println!("Unable to connect to MQTT broker: {:?}", e);
        process::exit(1);
    }

    stream(cli, pins).unwrap();
}

