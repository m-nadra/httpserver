use std::collections::HashMap;

#[derive(Default)]
pub struct Static {
    statics: HashMap<String, String>,
}

impl Static {
    pub fn get_content_path(&self, path: &str) -> Option<String> {
        for (route, dir) in self.statics.iter() {
            if path.starts_with(route) {
                let mut file_path = dir.clone();
                file_path.push_str(&path.to_owned().split_off(route.len()));
                return Some(file_path);
            }
        }
        None
    }
    pub fn insert(&mut self, path: String, directory: String) {
        self.statics.insert(path, directory);
    }
}
