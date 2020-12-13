use crate::devices::remote_socket;
use std::sync::{Arc, Mutex};
use std::io::Write;

#[allow(unused_must_use)]
pub fn write_to_serial(serialport_arc: Arc<Mutex<Box<dyn serialport::SerialPort>>>, dev: &remote_socket::RemoteSocket, state: String) {
	let output = create_serial_string(dev, state);	
	serialport_arc.lock().unwrap().write(output.as_bytes());
}

fn create_serial_string(dev: &remote_socket::RemoteSocket, state: String) -> String {
	let name = dev.get_name();
	let remote = dev.get_remote();
	let device = dev.get_device();
	let result = format!("{{name: \"{}\", state: \"{}\", remote: \"{}\", device: \"{}\"}}\n", name, state, remote, device);
	
	result
}
