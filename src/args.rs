use walkdir::WalkDir;

pub fn get_folder() -> String {
    "./".to_string()
}

pub fn scan_files(folder: &str) -> Vec<String> {
    WalkDir::new(folder)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().display().to_string())
        .collect()
}
