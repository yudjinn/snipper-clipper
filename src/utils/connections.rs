struct DBConnection {
	uri: String,
	port: u64,
	username: String,
	password: String,
	database: String
}

struct SSHConnection {
	
}

trait Connection {
	fn get(&self) -> Snippets;
	fn update(&self, data: <T>) -> Result<(),Error>;
}

impl Connection for DBConnection {
	fn get(&self) -> Snippets {
		todo!()
	}
	fn update(&self) -> Result<(),Error> {
		todo!()
	}
}