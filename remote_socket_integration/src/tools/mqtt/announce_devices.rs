use crate::config::Configuration;

pub fn announce_from_config(conf: &Configuration) {
	
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

