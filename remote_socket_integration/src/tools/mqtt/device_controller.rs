use crate::devices::remote_socket;

use rumqtt::{MqttClient, QoS};
use std::io;
use std::io::Write;

pub fn apply_new_state(mqtt_client: &mut MqttClient, dev: &remote_socket::RemoteSocket, state: String) {
	write_to_mqtt(mqtt_client, dev, state.clone());
	
	let stdout = io::stdout();
	let _ = writeln!(&mut stdout.lock(), "{} {}", dev.get_name(), state);	
}

fn write_to_mqtt(mqtt_client: &mut MqttClient, dev: &remote_socket::RemoteSocket, state: String) {
	let mname = dev.get_name().replace(" ", "_");
	let path = format!("homeassistant/switch/{}/state", mname);
	
	mqtt_client.publish(path, QoS::AtLeastOnce, false, state).unwrap();
}


