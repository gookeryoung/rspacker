use std::env;
use dirs;

pub fn get_cache_env() {
    let mut cache_dir = env::var("RSPACKER_CACHE_DIR").unwrap().to_string();
    if cache_dir.is_empty() {
    } else {
        cache_dir = cache_dir + "";
    }
}