use crate::customerror::Result;
use std::{
    collections::HashMap,
    fs::{write, File, OpenOptions},
    io::Read,
    path::Path,
};

pub struct Cache {
    cache: HashMap<String, String>,
    file_path: String,
}

impl Cache {
    pub fn new(file_path: &str) -> Self {
        let cache = match Self::read_cache(file_path) {
            Ok(cache) => cache,
            Err(_) => HashMap::new(),
        };

        Cache {
            cache,
            file_path: file_path.to_string(),
        }
    }

    fn read_cache(file_path: &str) -> Result<HashMap<String, String>> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Ok(HashMap::new());
        }

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let cache: HashMap<String, String> = serde_json::from_str(&contents)?;

        Ok(cache)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.cache.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.cache.get(&key)
    }

    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

impl Drop for Cache {
    fn drop(&mut self) {
        let _ = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.file_path);

        // TODO: handle fs::write result
        if let Ok(str_cache) = serde_json::to_string(&self.cache) {
            let _ = write(&self.file_path, str_cache);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::test_utils::setup_cache_dir;

    use super::Cache;
    use tempfile::TempDir;
    #[test]
    fn create_new_cache_if_file_not_exists() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let cache = Cache::new(&temp_dir.path().to_string_lossy());
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn can_deserialize_file_to_hashmap() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let cache_path = format!("{}/{}", temp_dir.path().to_string_lossy(), ".cache.json");
        setup_cache_dir(&cache_path);
        let contents = fs::read_to_string(cache_path.clone()).expect("yoooo");
        println!("contents: {}", contents);
        let cache = Cache::new(&cache_path);
        assert_eq!(cache.size(), 1);
    }

    #[test]
    fn can_insert() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut cache = Cache::new(&temp_dir.path().to_string_lossy());
        cache.set("fooo".to_string(), "bar".to_string());
        assert_eq!(cache.size(), 1);
        assert_eq!(
            cache.get("fooo".to_string()).unwrap().to_string(),
            "bar".to_string()
        );
    }
}
