use std::path::Path;
use std::{collections::HashMap, fs};

const CACHE_FILE: &str = "/tmp/poke_cache.json";

pub fn get_cached_response_for_id(id: u32) -> Option<String> {
    let map = get_map();
    let cache = map.get(&id.to_string())?;
    Some(cache.to_owned())
}

pub fn set_cached_response_for_id(id: u32, response: &str) -> bool {
    let mut map = get_map();
    map.insert(id.to_string(), response.to_owned());
    let data = serde_json::to_string(&map).expect("expected to be able to serialize hashmap");
    fs::write(CACHE_FILE, data).expect("Unable to write file");
    true
}

fn get_map() -> HashMap<String, String> {
    let exists = Path::new(CACHE_FILE).exists();
    if !exists {
        return HashMap::new();
    }
    let data = fs::read_to_string(CACHE_FILE).expect("Unable to read file");
    let map: HashMap<String, String> = serde_json::from_str(&data).expect("Wrong json format");
    map
}
