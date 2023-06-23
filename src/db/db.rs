use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonDb {
    version: String,
    key_size: u32,
    pub domains: HashMap<String, Account>
}

pub type Account = HashMap<String, Vec<String>>;

