use crate::config::Configuration;


pub fn thread_listener_from_config(conf: &Configuration) {
	
}

fn create_mqtt_set_path(dev: &remote_socket::RemoteSocket) -> String{
	let mname = dev.get_name().replace(" ", "_");
	
	return format!("homeassistant/switch/{}/set", mname);
}

