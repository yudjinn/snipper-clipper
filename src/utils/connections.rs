use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::{Error, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub trait Connection {
    fn load<U>(&self) -> Result<U>
    where
        U: DeserializeOwned;
    fn update<U>(&self, data: U) -> Result<U>
    where
        U: Serialize;
}

// not currently working TODO
#[derive(Debug, Serialize, Deserialize)]
pub struct DBConnection {
    uri: String,
    port: u64,
    username: String,
    password: String,
    database: String,
}

impl Connection for DBConnection {
    fn load<U>(&self) -> Result<U>
    where
        U: DeserializeOwned,
    {
        todo!()
    }
    fn update<U>(&self, data: U) -> Result<U>
    where
        U: Serialize,
    {
        todo!()
    }
}

// not currently working TODO
#[derive(Debug, Serialize, Deserialize)]
pub struct SSHConnection {
    uri: String,
    port: u64,
    username: String,
    password: String,
}

impl Connection for SSHConnection {
    fn load<U>(&self) -> Result<U>
    where
        U: DeserializeOwned,
    {
        todo!()
    }
    fn update<U>(&self, data: U) -> Result<U>
    where
        U: Serialize,
    {
        todo!()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JSON {
    path: PathBuf,
}

impl JSON {
    pub fn new(path: &PathBuf) -> Self {
        Self { path: path.clone() }
    }

    pub fn default() -> Self {
        let path = PathBuf::new();
        Self { path }
    }
}

impl Connection for JSON {
    fn load<U>(&self) -> Result<U>
    where
        U: DeserializeOwned,
    {
        let path = self.path.clone();
        let contents = std::fs::read_to_string(&path).unwrap().to_owned();
        match serde_json::from_str(&contents) {
            Ok(obj) => Ok(obj),
            Err(e) => Err(Error::msg(format!("Could not load from JSON: {}", e))),
        }
    }

    fn update<U>(&self, data: U) -> Result<U>
    where
        U: Serialize,
    {
        match serde_json::to_string_pretty(&data) {
            Ok(val) => {
                // let mut file = std::fs::File::open(&self.path).unwrap();
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&self.path)?;
                file.write_all(&val.into_bytes())?;
                Ok(data)
            }
            Err(e) => Err(Error::msg(format!("Could not serialize to JSON: {}", e))),
        }
    }
}
