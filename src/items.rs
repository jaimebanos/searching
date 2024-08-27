use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, Eq, Clone)]
pub struct Item {
    isin: String,
    symbol: String,
    pub website: String,
    pub logo: Option<String>,
}

impl Item {
    pub fn new(path: &str) -> HashSet<Item> {
        let message: String = fs::read_to_string(path).unwrap();
        let items: HashSet<Item> = serde_json::from_str(&message).unwrap();

        return items;
    }
}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.isin.hash(state);
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.isin == other.isin
    }
}
