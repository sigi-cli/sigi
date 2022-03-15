use std::error::Error;
use std::io::ErrorKind;
use std::{env, fs, path::Path};

// TODO: Alternate backends:
//       - Redis
//       - SQLite
//       - The existing version (JSON via Serde)
// TODO: Configurable data location?
// TODO: Allow an idea of "stack of stacks"

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

type ItemHistory = Vec<(String, DateTime<Local>)>;

/// A single stack item.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    // TODO: Update from "name" to "contents"?
    pub contents: String,
    pub history: ItemHistory,
}

impl Item {
    pub fn new(contents: &str) -> Self {
        Item {
            contents: contents.to_string(),
            history: vec![("created".to_string(), Local::now())],
        }
    }

    pub fn mark_completed(&mut self) {
        let event = ("completed".to_string(), Local::now());
        self.history.push(event);
    }

    pub fn mark_deleted(&mut self) {
        let event = ("deleted".to_string(), Local::now());
        self.history.push(event);
    }

    pub fn mark_restored(&mut self) {
        let event = ("restored".to_string(), Local::now());
        self.history.push(event);
    }
}

/// A stack of items.
// TODO: Is there a better stack type than Vec? We only ever perform one command
// per CLI invocation, so there isn't a huge need for stack optimization yet.
pub type Stack = Vec<Item>;

/// Save a stack of items.
// TODO: Create a custom error. This is returning raw filesystem errors.
pub fn save(stack_name: &str, items: Stack) -> Result<(), impl Error> {
    let data_path: String = sigi_file(stack_name);
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
// TODO: Create a custom error. This is returning raw serialization errors.
pub fn load(stack_name: &str) -> Result<Stack, impl Error> {
    let data_path: String = sigi_file(stack_name);
    let read_result = fs::read_to_string(data_path);
    if read_result.is_err() && read_result.as_ref().unwrap_err().kind() == ErrorKind::NotFound {
        return Ok(vec![]);
    }

    let json = read_result.unwrap();
    let result = serde_json::from_str(&json);

    if result.is_err() {
        let v1result = v1_load(&json);
        if let Ok(v1stack) = v1result {
            return Ok(v1_to_modern(v1stack));
        }
    }

    result
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

/// A single stack item. Used for backwards compatibility with versions of Sigi v1.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct V1Item {
    pub name: String,
    pub created: DateTime<Local>,
    pub succeeded: Option<DateTime<Local>>,
    pub failed: Option<DateTime<Local>>,
}

/// A stack of items. Used for backwards compatibility with versions of Sigi v1.
pub type V1Stack = Vec<V1Item>;

/// Attempt to read a V1 format file.
fn v1_load(json_blob: &str) -> Result<V1Stack, impl Error> {
    serde_json::from_str(json_blob)
}

fn v1_to_modern(v1stack: V1Stack) -> Stack {
    v1stack
        .into_iter()
        .map(|v1item| {
            // Translate the old keys to entries.
            let mut history: ItemHistory = vec![
                Some(("created", v1item.created)),
                v1item.succeeded.map(|dt| ("completed", dt)),
                v1item.failed.map(|dt| ("deleted", dt)),
            ]
            .into_iter()
            .flatten()
            .map(|(s, dt)| (s.to_string(), dt))
            .collect();
            history.sort_by_key(|(_, dt)| *dt);
            Item {
                contents: v1item.name,
                history,
            }
        })
        .collect()
}
