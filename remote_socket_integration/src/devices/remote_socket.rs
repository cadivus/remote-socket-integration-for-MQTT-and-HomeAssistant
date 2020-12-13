#[derive(Clone)]
pub struct RemoteSocket {
	pub name: String,
	pub manufacturer: String,
	pub model: String,
	pub remote: usize,
	pub device: usize,
}

impl RemoteSocket {
	pub fn get_remote(&self) -> usize {
		  self.remote
	}
	
	pub fn get_device(&self) -> usize {
		  self.device
	}
	
	pub fn get_name(&self) -> String {
		  self.name.clone()
	}
	
	pub fn get_manufacturer(&self) -> String {
		  self.manufacturer.clone()
	}
	
	pub fn get_model(&self) -> String {
		  self.model.clone()
	}
}  
