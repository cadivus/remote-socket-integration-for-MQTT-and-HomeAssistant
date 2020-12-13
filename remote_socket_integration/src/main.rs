pub mod devices;
pub mod config;
pub mod tools;
use crate::tools::mqtt::announce_devices;
use crate::tools::mqtt::device_listener;

use std::io;
use std::io::prelude::*;

fn main() {
	let conf = config::get_configuration();
	
	announce_devices::announce_from_config(&conf);
	device_listener::thread_listener_from_config(&conf);
		
	pause();
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    writeln!(stdout, "Press Enter to exit...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
