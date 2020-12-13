use crate::devices::remote_socket;

use rumqtt::{MqttOptions, ReconnectOptions, SecurityOptions};
use std::sync::{Arc, Mutex};
use std::fs;
use serde_json::{Value};


pub struct Configuration {
	port: Arc<Mutex<Box<dyn serialport::SerialPort>>>,
	pub devices: Vec<remote_socket::RemoteSocket>,
	brokeraddress: String,
	brokerport: u16,
	brokeruser: String,
	brokerpassword: String,
}
