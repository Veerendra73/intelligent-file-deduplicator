use std::fs::File;
use std::io::Write;
use serde::Serialize;

#[derive(Serialize)]
struct SimpleReport {
    scanned_files: usize,
    removed_duplicates: usize,
}

pub fn save_report(scanned: usize, removed: usize) {
    let report = SimpleReport {
        scanned_files: scanned,
        removed_duplicates: removed,
    };

    let json = serde_json::to_string_pretty(&report).unwrap();
    let mut file = File::create("report.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("Report saved to report.json");
}
