use crate::actions::Command;
use std::error::Error;
use std::io::ErrorKind;
use std::{env, fs, path::Path};

// TODO: Alternate backends? Right now it's just JSON files. Maybe SQLite? PilaDB?
//       Something new? Daemon(s) could prevent many loads and handle locking.
// TODO: Configurable data location?
// TODO: Allow an idea of "stack of stacks"
// TODO: Figure out a good naming algorithm (maybe numeric?)

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// A single stack item.
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    // TODO: Update from "name" to "contents"?
    pub name: String,
    pub created: DateTime<Local>,
    pub succeeded: Option<DateTime<Local>>,
    pub failed: Option<DateTime<Local>>,
}

/// A stack of items.
// TODO: Is there a better stack type than Vec? We only ever perform one command
// per CLI invocation, so there isn't a huge need for stack optimization yet.
pub type Stack = Vec<Item>;

/// Save a stack of items.
// TODO: Create a custom error. This is returning raw filesystem errors.
pub fn save(command: &Command, items: Stack) -> Result<(), impl Error> {
    let data_path: String = sigi_file(&command.topic);
    let json: String = serde_json::to_string(&items).unwrap();
    let result = fs::write(&data_path, &json);
    if result.is_err() && result.as_ref().unwrap_err().kind() == ErrorKind::NotFound {
        fs::create_dir_all(sigi_path()).unwrap();
        fs::write(data_path, json)
    } else {
        result
    }
}

/// Load a stack of items.
// TODO: Create a cuustom error. This is returning raw serialization errors.
pub fn load(command: &Command) -> Result<Stack, impl Error> {
    let data_path: String = sigi_file(&command.topic);
    let read_result = fs::read_to_string(data_path);
    if read_result.is_err() && read_result.as_ref().unwrap_err().kind() == ErrorKind::NotFound {
        Ok(vec![])
    } else {
        let json = read_result.unwrap();
        serde_json::from_str(&json)
    }
}

fn sigi_path() -> String {
    let home = env::var("HOME").or_else(|_| env::var("HOMEDRIVE")).unwrap();
    let path = format!("{}/.local/share/sigi", home);
    Path::new(&path).to_string_lossy().to_string()
}

fn sigi_file(filename: &str) -> String {
    let path = format!("{}/{}.json", sigi_path(), filename);
    Path::new(&path).to_string_lossy().to_string()
}
