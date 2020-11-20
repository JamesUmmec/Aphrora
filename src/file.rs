use std::fs;

/// Try get pages file under a root path,
/// which enable approach to different pages easier.
pub fn try_under_root(root_path: &str, file_path: &str) -> Option<String> {
    match fs::read_to_string(root_path.to_owned() + file_path) {
        Ok(file_string) => Option::Some(file_string),
        Err(_) => Option::None,
    }
}
