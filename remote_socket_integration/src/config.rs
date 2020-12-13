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

impl Configuration {
	pub fn get_devices(&self) -> Vec<remote_socket::RemoteSocket> {
		self.devices.clone()
	}
	
	pub fn get_mqtt_options(&self) -> MqttOptions {
		let reconnection_options = ReconnectOptions::Always(10);
		
		let mut security = SecurityOptions::None;
        if !self.brokeruser.is_empty() {
        	security = SecurityOptions::UsernamePassword
        			(self.brokeruser.clone(), self.brokerpassword.clone());
        }
		
		MqttOptions::new("standard-sub", self.brokeraddress.clone(), self.brokerport)
                                    .set_keep_alive(10)
                                    .set_inflight(3)
                                    .set_request_channel_capacity(3)
                                    .set_reconnect_opts(reconnection_options)
                                    .set_clean_session(false)
                                    .set_security_opts(security)
	}
	
	pub fn get_mqtt_options_with_id(&self, client_id: &str) -> MqttOptions {
		let reconnection_options = ReconnectOptions::Always(10);
		
		let mut security = SecurityOptions::None;
        if !self.brokeruser.is_empty() {
        	security = SecurityOptions::UsernamePassword
        			(self.brokeruser.clone(), self.brokerpassword.clone());
        }
		
		MqttOptions::new(client_id, self.brokeraddress.clone(), self.brokerport)
                                    .set_keep_alive(10)
                                    .set_inflight(3)
                                    .set_request_channel_capacity(3)
                                    .set_reconnect_opts(reconnection_options)
                                    .set_clean_session(false)
                                    .set_security_opts(security)
	}
	
	pub fn get_get_serialport_arc(&self) -> Arc<Mutex<Box<dyn serialport::SerialPort>>> {
		self.port.clone()
	}
}

fn get_broker_config_from_file(config_path: std::path::PathBuf) -> (String, u16, String, String) {
	let contents = fs::read_to_string(config_path)
        .expect("Something went wrong reading the file");
        
    let json_v: Value = serde_json::from_str(contents.as_str()).unwrap();
    
    let broker = json_v["broker"].as_str().unwrap();
	let brokerport = json_v["brokerport"].as_u64().unwrap() as u16;
    let brokeruser = json_v["brokeruser"].as_str().unwrap();
    let brokerpassword = json_v["brokerpassword"].as_str().unwrap();
														  
	(broker.to_string(), brokerport, brokeruser.to_string(), brokerpassword.to_string())
}
