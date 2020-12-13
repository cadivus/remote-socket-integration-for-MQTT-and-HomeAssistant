use crate::config::Configuration;
use crate::devices::remote_socket;
use crate::tools::mqtt::device_controller;
use crate::tools::serialport::state_sender;

use rumqtt::{MqttClient, QoS};
use std::thread;


pub fn thread_listener_from_config(conf: &Configuration) {
	let devices = conf.get_devices();
	
	for dev in devices.into_iter() {
		let path = create_mqtt_set_path(&dev);
		let mqtt_options = conf.get_mqtt_options_with_id(&path.replace("/", "_"));
		let port = conf.get_get_serialport_arc();
		
		thread::spawn(move || {
			let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
			mqtt_client.subscribe(path.clone(), QoS::AtLeastOnce).unwrap();
			
			
			
			for notification in notifications {
				match notification {
					rumqtt::Notification::Publish(nnn) => {
						let dev_state = String::from_utf8_lossy(&nnn.payload);
						
						device_controller::apply_new_state(&mut mqtt_client, &dev, dev_state.to_string());
						state_sender::write_to_serial(port.clone(), &dev, dev_state.to_string());
					},
					_ => println!("{} will nicht", path.clone()),
				}
			}
		});
	}
}

fn create_mqtt_set_path(dev: &remote_socket::RemoteSocket) -> String{
	let mname = dev.get_name().replace(" ", "_");
	
	return format!("homeassistant/switch/{}/set", mname);
}

