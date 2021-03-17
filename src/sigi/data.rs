use crate::sigi::items::Items;
use std::error::Error;
use std::io::ErrorKind;
use std::{env, fs};

// TODO: For non-unixy systems, need to use std::path::MAIN_SEPARATOR
const SIGI_DATA_PATH: &str = ".local/share/sigi";

// TODO: Allow an idea of "stack of stacks"
// TODO: Allow namespaces
// TODO: Figure out a good naming algorithm (maybe numeric?)

pub fn save(topic: &str, items: Items) -> Result<(), impl Error> {
    let data_path: String = sigi_file(topic);
    let json: String = serde_json::to_string(&items).unwrap();
    let result = fs::write(&data_path, &json);
    if result.is_err() && result.as_ref().unwrap_err().kind() == ErrorKind::NotFound {
        fs::create_dir_all(sigi_path()).unwrap();
        fs::write(data_path, json)
    } else {
        result
    }
}

pub fn load(topic: &str) -> Result<Items, impl Error> {
    let data_path: String = sigi_file(topic);
    let read_result = fs::read_to_string(data_path);
    if read_result.is_err() && read_result.as_ref().unwrap_err().kind() == ErrorKind::NotFound {
        Ok(vec![])
    } else {
        let json = read_result.unwrap();
        serde_json::from_str(&json)
    }
}

fn sigi_path() -> String {
    format!("{}/{}", env::var("HOME").unwrap(), SIGI_DATA_PATH)
}

fn sigi_file(filename: &str) -> String {
    format!("{}/{}.json", sigi_path(), filename)
}
