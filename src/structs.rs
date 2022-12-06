use crate::utils::{
    connections::{Connection, JSON},
    error::{Error, Result},
};
use core::fmt::Debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Language {
    Go,
    Rust,
    Python,
    C,
    Bash,
    Cpp,
    Java,
    Miscellaneous,
}

impl FromStr for Language {
    type Err = Error;

    fn from_str(input: &str) -> Result<Language> {
        match input.to_lowercase().as_str() {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Storage {
    JSON(crate::utils::connections::JSON),
}

impl Storage {
    fn update<P>(&self, data: P) -> Result<P>
    where
        P: Serialize,
    {
        match self {
            Storage::JSON(json) => json.update(data),
        }
    }

    fn load<P>(&mut self) -> Result<P>
    where
        P: DeserializeOwned + Debug,
    {
        match self {
            Storage::JSON(json) => json.load(),
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Storage::JSON(JSON::default())
        // Storage::JSON(path)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    storage: Storage,
    theme: String,
}

impl Persist for Config {
    fn load(&mut self) -> Result<Self> {
        match self.storage.load() {
            Ok(config) => Ok(config),
            Err(e) => {
                eprintln!("Could not retrieve snippets: {}", e);
                return Self::default();
            }
        }
    }

    fn save(&self) -> Result<()> {
        todo!()
    }
}

impl Config {
    pub fn default<'st>() -> Result<Self> {
        let env = xdg::BaseDirectories::new()?;
        let mut path = env.get_config_home();
        path.push("snipperclipper/config.json");
        if let Some(env_path) = std::env::var("SC_CONFIG").ok() {
            path = PathBuf::from_str(&env_path).unwrap();
        }
        let config: Config = match serde_json::from_str(&std::fs::read_to_string(&path).unwrap()) {
            Ok(json) => json,
            Err(_) => Config {
                storage: Storage::JSON(JSON::new(&path)),
                theme: "Yes".to_string(),
            },
        };
        // config.storage = JSON::new(path.clone());
        return Ok(config);
    }
}

pub trait Persist {
    fn load(&mut self) -> Result<Self>
    where
        Self: Sized;
    fn save(&self) -> Result<()>;
}

impl Debug for dyn Persist {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Persist {{}}")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Snippets {
    storage: Storage,
    snippets: Vec<Snippet>,
}

impl Persist for Snippets {
    fn load(&mut self) -> Result<Self> {
        match self.storage.load() {
            Ok(snips) => Ok(snips),
            Err(e) => {
                eprintln!("Could not retrieve snippets: {}", e);
                return Self::default();
            }
        }
        // match self.storage {
        //     Storage::JSON(path) => {
        //         let env = xdg::BaseDirectories::new()?;
        //         let mut path = env.get_config_home();
        //         path.push("snipperclipper/config.json");
        // if let Some(env_path) = std::env::var("SC_CONFIG").ok() {
        // path = PathBuf::from_str(&env_path).unwrap();
        // }
        // }
        // }
    }

    fn save(&self) -> Result<()> {
        self.storage.update(self)?;
        Ok(())
    }
}

impl Snippets {
    pub fn add(&mut self, snip: Snippet) {
        let dbg = snip.clone();
        self.snippets.push(snip);
        println!("new vec: {:?}", &self.snippets);
        match self.save() {
            Ok(_) => {
                println!("Saved new snip: {:?}", dbg);
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
            }
        }
    }

    pub fn find(&self, s: &str) -> Option<Vec<Snippet>> {
        let snips = self.clone();
        let snips: Vec<Snippet> = snips
            .snippets
            .iter()
            .filter(|x| x.name.contains(s))
            .map(|x| x.to_owned())
            .collect();
        if snips.len() > 0 {
            Some(snips)
        } else {
            None
        }
    }

    pub fn get(&self, s: &str) -> Option<Snippet> {
        let snips = self.clone();
        let snips: Vec<&Snippet> = snips
            .snippets
            .iter()
            .filter(|x| x.name.contains(s))
            .collect();
        match snips.first() {
            Some(snip) => return Some(snip.to_owned().to_owned()),
            None => None,
        }
    }

    pub fn list_by_lang(&self, lang: &str) -> Result<()> {
        let lang_enum = Language::from_str(lang)?;
        if lang_enum == Language::Miscellaneous {
            println!("Could not scope by that language type.");
            return Ok(());
        }
        let snips = self.clone();
        let snips: Vec<&Snippet> = snips
            .snippets
            .iter()
            .filter(|x| x.language == lang_enum)
            .collect();
        if snips.len() > 0 {
            println!("Here are the [{}] snippets: {:#?}", &lang, snips);
        } else {
            println!("No snippets found for langauge [{}]", lang);
        }
        Ok(())
    }

    pub fn list_all(&self) -> &Vec<Snippet> {
        //show all
        println!("Here are the snippets:\n {:#?}", self.snippets);
        return &self.snippets;
    }

    pub fn default() -> Result<Self> {
        let def_snip = Snippet::default();
        let vec = vec![def_snip];
        let env = xdg::BaseDirectories::new()?;
        let mut path = env.get_config_home();
        path.push("snipperclipper/snippets.json");
        Ok(Self {
            snippets: vec,
            storage: Storage::JSON(JSON::new(&path)),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    id: Uuid,
    name: String,
    folder: String,
    pub body: String,
    language: Language,
}

impl Snippet {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("New Snippet"),
            folder: String::new(),
            body: String::from("Lorem Ipsum"),
            language: Language::Miscellaneous,
        }
    }

    pub fn from_string(full_name: &str, body: &str) -> Self {
        let (folder, name, language) = Snippet::parse_name(full_name);
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            folder: folder.to_string(),
            body: body.to_string(),
            language,
        }
    }

    pub fn parse_name(raw: &str) -> (&str, &str, Language) {
        let mut folder = "";
        let mut name = "";
        let l;
        let mut lang = Language::Miscellaneous;
        if raw.contains("/") {
            (folder, name) = raw.split_once("/").unwrap();
            if name.contains(".") {
                (name, l) = name.split_once(".").unwrap();
                lang = Language::from_str(l).unwrap();
            }
        } else if raw.contains(".") {
            (name, l) = name.split_once(".").unwrap();
            lang = Language::from_str(l).unwrap();
        } else {
            name = raw;
        }

        return (folder, name, lang);
    }

    pub fn from_stdin(raw: &str) -> Self {
        todo!()
    }

    pub fn print(&self) {
        // print to tty
        let output = serde_json::to_string_pretty(&self).unwrap();
        println!("{}", output);
        todo!()
    }
}
