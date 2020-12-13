use crate::devices::remote_socket;

use rumqtt::{MqttClient, QoS};

pub fn apply_new_state(mqtt_client: &mut MqttClient, dev: &remote_socket::RemoteSocket, state: String) {
	
}
