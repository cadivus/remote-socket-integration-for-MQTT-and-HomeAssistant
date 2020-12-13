use crate::config::Configuration;
use crate::devices::remote_socket;
use rumqtt::{MqttClient, QoS};

pub fn announce_from_config(conf: &Configuration) {
	let devices = conf.get_devices();
	
	let (mut mqtt_client, _) = MqttClient::start(conf.get_mqtt_options()).unwrap();
	
	for dev in devices.iter() {
		let path = create_mqtt_path(dev);
		let message = create_mqtt_string(dev);
		
		mqtt_client.publish(path, QoS::AtLeastOnce, false, message).unwrap();
	}
}

fn create_mqtt_string(dev: &remote_socket::RemoteSocket) -> String{
	let mut result: String = "{\n".to_owned();
	
	let name = dev.get_name();
	let mname = dev.get_name().replace(" ", "_");
	let manufacturer = dev.get_manufacturer();
	let model = dev.get_model();
	
	result.push_str(&format!("  \"name\": \"{}\",\n", name));
	result.push_str(&format!("  \"command_topic\": \"homeassistant/switch/{}/set\",\n", mname));
	result.push_str(&format!("  \"state_topic\": \"homeassistant/switch/{}/state\",\n", mname));
	result.push_str("  \"device\": {\n");
	result.push_str(&format!("    \"identifiers\":[\"mybridge_{}\"],\n", mname));
	result.push_str(&format!("    \"manufacturer\":\"{}\",\n", manufacturer));
	result.push_str(&format!("    \"model\":\"{}\",\n", model));
	result.push_str(&format!("    \"name\":\"{}\"\n", name));
	result.push_str("  },\n");
	result.push_str(&format!("  \"unique_id\": \"mybridge_{}\"\n", mname));
	result.push_str("}\n");
	
	result
}

fn create_mqtt_path(dev: &remote_socket::RemoteSocket) -> String{
	let mname = dev.get_name().replace(" ", "_");
	let result = format!("homeassistant/switch/{}/config", mname);
	
	result
}

