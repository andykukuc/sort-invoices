use chrono::Local;
use std::fs;
use std::io::Write;
use std::path::Path;

const BASE: &str = "/mnt/samba_pool/samba/invoices";
const LOG_PATH: &str = "/var/log/sort-invoices.log";

fn log_message(message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let line = format!("[{}] {}\n", timestamp, message);
    print!("{}", line);

    if let Ok(mut file) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
    {
        let _ = file.write_all(line.as_bytes());
    }
}

fn dest_subfolder(ext: &str) -> Option<&'static str> {
    match ext.to_lowercase().as_str() {
        "pdf" => Some("PDF"),
        "xlsm" | "xlsx" | "xls" => Some("Excel"),
        "doc" | "docx" => Some("Word"),
        _ => None,
    }
}

fn sort_folder(folder_name: &str) {
    let folder_path = Path::new(BASE).join(folder_name);

    if !folder_path.is_dir() {
        log_message(&format!("SKIP: {} not found", folder_name));
        return;
    }

    let entries = match fs::read_dir(&folder_path) {
        Ok(e) => e,
        Err(e) => {
            log_message(&format!("ERROR: cannot read {}: {}", folder_name, e));
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let filename = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };

        // Skip hidden/temp files
        if filename.starts_with('.') || filename.starts_with('~') {
            continue;
        }

        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e,
            None => {
                log_message(&format!("SKIPPED: {}/{} (no extension)", folder_name, filename));
                continue;
            }
        };

        match dest_subfolder(ext) {
            Some(subfolder) => {
                let subfolder_path = folder_path.join(subfolder);
                if let Err(e) = fs::create_dir_all(&subfolder_path) {
                    log_message(&format!("ERROR: could not create {}: {}", subfolder_path.display(), e));
                    continue;
                }
                let dest = subfolder_path.join(&filename);
                match fs::rename(&path, &dest) {
                    Ok(_) => log_message(&format!(
                        "MOVED: {}/{} -> {}/",
                        folder_name, filename, subfolder
                    )),
                    Err(e) => log_message(&format!(
                        "ERROR: could not move {}/{}: {}",
                        folder_name, filename, e
                    )),
                }
            }
            None => {
                log_message(&format!(
                    "SKIPPED: {}/{} (unknown extension: {})",
                    folder_name, filename, ext
                ));
            }
        }
    }
}

fn main() {
    log_message("--- sort-invoices started ---");

    let base = Path::new(BASE);
    let entries = match fs::read_dir(base) {
        Ok(e) => e,
        Err(e) => {
            log_message(&format!("ERROR: cannot read base dir: {}", e));
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let folder_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };

        // Skip hidden folders and the archive folder
        if folder_name.starts_with('.') || folder_name.to_lowercase() == "archive" {
            continue;
        }

        sort_folder(&folder_name);
    }

    log_message("--- sort-invoices complete ---");
}
