use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub created: DateTime<Local>,
    pub succeeded: Option<DateTime<Local>>,
    pub failed: Option<DateTime<Local>>,
}

pub type Items = Vec<Item>;
