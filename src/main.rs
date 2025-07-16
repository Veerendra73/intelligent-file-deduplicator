mod args;
mod hash;
mod report;

use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    println!("🔍 Starting Duplicate File Detection...");

    let folder = args::get_folder();
    let files = args::scan_files(&folder);

    println!("Files scanned: {}", files.len());

    let file_hashes = hash::create_hash_map(&files);

    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for (file, file_hash) in &file_hashes {
        groups.entry(file_hash.clone()).or_default().push(file.clone());
    }

    let mut duplicates_removed = 0;

    for (_hash, group) in &groups {
        if group.len() > 1 {
            println!("\nDuplicates found:");
            for file in group.iter().skip(1) {
                println!("Duplicate File: {}", file);

                let file_name = Path::new(file)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();

                fs::create_dir_all("quarantine").unwrap();
                let quarantine_path = format!("quarantine/{}", file_name);

                fs::rename(file, &quarantine_path).unwrap();
                println!("File moved to quarantine folder.");

                fs::remove_file(&quarantine_path).unwrap();
                println!("File deleted from quarantine folder.");

                duplicates_removed += 1;
            }
        }
    }

    report::save_report(files.len(), duplicates_removed);

    println!("\nProcess Completed. Total duplicate files removed: {}", duplicates_removed);
}

