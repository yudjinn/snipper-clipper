use uuid::Uuid;
use serde_json::{Serialize,Deserialize};
use anyhow::Error;

enum Language {
	Go,
	Rust,
	Python,
	C,
	Bash,
	Cpp,
	Java,
	Miscellaneous
}

impl FromStr for Language {
	type Err = ();
	
	fn from_str(input: &str) -> Result<Language, Self::Err> {
		match input.to_lowercase() {
			"go" => Ok(Language::Go),
			"rs" => Ok(Language::Rust),
			"py" => Ok(Language::Python),
			"c" => Ok(Language::C),
			"sh" => Ok(Language::Bash),
			"cpp" => Ok(Language::Cpp),
			"java" => Ok(Language::Java),
			_ => Ok(Language::Miscellaneous),
		}
	}
}

enum Storage {
	JSON{path: PathSpec},
	DB(DBConnection),
	SSH(SSHConnection)
}

#[derive(Serialize, Deserialize)]
struct Config {
	storage: Storage,
	theme: String,
}

impl Config {
	fn read(&mut self) -> Result<(), Error> {
		todo!();
	}
	
	fn default() -> Self {
		let mut path = xdg::ConfigFile("snipperclipper/config.toml");
		if let env_path = std::os::GetEnv("SC_CONFIG"){
			path = env_path.clone();
		}
		let json = serde_json::read(&path);
		let config: Config = serde_json::deserialize(json);
		return config
	}
}

trait Persist {
	pub fn load(&mut self) -> Result<<T>, Error>;
	pub fn save(&self) -> Result<(), Error>;
}

#[derive(Serialize, Deserialize)]
struct Snippets {
	snippets: Vec<Snippet>
}

impl Persist for Snippets {
	fn load(&mut self) -> Result<Snippets, Error> {
		match self.storage.get() {
			Ok(snips) => {
				self.snippets = snips;
				return Ok(());
			}
			Err(e) => {
				eprintln!("Could not retrieve snippets: {}", e);
				return Err(Error::Connection);
			}
		}
	}
	
	fn save(&self) -> Result<(),Error> {
		self.storage.update(&self.snippets)?;
	}
}

impl Snippets {
	fn add(&mut self, snip: Snippet) {
		self.snippets.push(snip);
		self.save();
	}
	
	fn find(s: &str) -> Option<&Snippet> {
		todo!()
	}
	
	fn get(s: &str) -> Option<&Snippet> {
		// get snippet by id
		todo!()
	}
	
	fn list_by_lang(lang: Language) -> &Vec<Snippet> {
		// list all with language
		todo!()
	}
	
	fn list_all(&self) -> &Vec<Snippet> {
		//show all
		return &self.snippets;
	}
}

#[derive(Serialize, Deserialize)]
struct Snippet {
	id: Uuid,
	name: String,
	folder: String,
	body: String,
	language: Language
}

impl Snippet {
	fn default() -> Self {
		Self {
			id: Uuid::new_v4(),
			name: String::from("New Snippet"),
			folder: String::new(),
			body: String::from("Lorem Ipsum"),
			language: Language::Miscellaneous
		}
	}
	
	pub fn from_string(full_name: &str, body: &str) -> Self {
		let (folder, name, language) = Snippet::parse_name(full_name);
		Self {
			id: Uuid::new_v4(),
			name,
			folder,
			body,
			language
		}
	}
	
	pub fn parse_name(raw: &str) -> (&str, &str, Language) {
		let mut folder = String::new();
		let mut name = String::new();
		let mut lang = Language::Miscellaneous;
		if raw.contains("/") {
			let split = raw.split_once("/").collect();
			folder = split[0];
			name = split[1];
			if name.contains(".") {
				let split =name.split_once(".").collect();
				name = split[0];
				lang = Language::from_str(split[1]).unwrap();
			}
		} else if raw.contains(".") {
			let split = raw.split_once(".").collect();
			name = split[0];
			lang = Language::from_str(split[1]).unwrap();
		} else {
			name = raw;
		}
		
		return (folder, name, lang);
		
	}
	
	pub fn from_stdin(raw: &str) -> Self {
		todo!()
	}
	
	fn print(&self) {
		// print to tty
		let output = serde_json::pretty_print(self);
		println!("{}", output);
		todo!() 
	}
}

