#![windows_subsystem = "windows"]

extern crate battery;
extern crate notify_rust;

use std::io;
use std::thread;
use std::time::Duration;
use notify_rust::Notification;

fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
    let mut battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to read battery information");
            return Err(e);
        }
        None => {
            eprintln!("No battery detected");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }
    };

    let time_left = battery.time_to_full().unwrap().value / 60.0;
    println!("There are {:?} minutes until the battery is full", time_left);
    loop {
        let charge = battery.state_of_charge().value * 100.0;
        println!("Currently at {:?}%", charge);

        if charge >= 99.7 {
            Notification::new()
                .summary("Battery is full")
                .icon("firefox")
                .appname("Battery Saver")
                .sound_name("battery-low")
                .body("The battery is now full")
                .show()
                .unwrap();
            return Ok(());
        }

        thread::sleep(Duration::from_secs(1));
        manager.refresh(&mut battery)?;
    }
}