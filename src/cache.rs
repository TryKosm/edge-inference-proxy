use std::collections::HashMap;

pub struct Cache {
    pub values: HashMap<String, String>,
}

impl Cache {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn put(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }
}
