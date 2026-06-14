// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use serde::Serialize;
use tauri::{Emitter, Manager};

#[derive(Serialize)]
struct FileEntry {
    name: String,
    path: String,
    is_dir: bool,
    is_git_repo: bool,
}

#[derive(Serialize)]
struct GitFileStatus {
    path: String,
    status: String, // "modified", "untracked", "deleted", "added"
}

#[derive(Serialize)]
struct LineDiff {
    line: u32,
    status: String, // "added", "modified", "deleted"
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_dir(path: String) -> Result<Vec<FileEntry>, String> {
    let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let metadata = entry.metadata().map_err(|e| e.to_string())?;
        let file_name = entry.file_name().into_string().unwrap_or_default();
        
        // Skip hidden files except .git
        if file_name.starts_with('.') && file_name != ".git" {
            continue;
        }

        let entry_path = entry.path();
        let is_git_repo = metadata.is_dir() && entry_path.join(".git").exists();

        files.push(FileEntry {
            name: file_name,
            path: entry_path.to_string_lossy().to_string(),
            is_dir: metadata.is_dir(),
            is_git_repo,
        });
    }

    files.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });

    Ok(files)
}

#[tauri::command]
fn get_git_branch(path: String) -> Result<String, String> {
    use std::process::Command;
    
    let output = Command::new("git")
        .args(&["-C", &path, "branch", "--show-current"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err("Not a git repository".to_string())
    }
}

#[tauri::command]
fn git_status(path: String) -> Result<Vec<GitFileStatus>, String> {
    use std::process::Command;
    use std::collections::HashMap;
    
    let output = Command::new("git")
        .args(&["-C", &path, "status", "--porcelain"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Ok(Vec::new()); // Not a git repo, return empty
    }

    let mut status_map: HashMap<String, String> = HashMap::new();
    
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if line.len() < 4 {
            continue;
        }
        
        let status_code = &line[0..2];
        let file_path = line[3..].trim().to_string();
        
        let status = match status_code.trim() {
            "M" | " M" | "MM" => "modified",
            "A" | "AM" => "added",
            "D" | " D" => "deleted",
            "??" => "untracked",
            "R" => "renamed",
            _ => "modified", // Default to modified for other cases
        };
        
        status_map.insert(file_path, status.to_string());
    }
    
    Ok(status_map.into_iter().map(|(path, status)| {
        GitFileStatus { path, status }
    }).collect())
}
#[tauri::command]
fn git_line_diff(path: String, file_path: String) -> Result<Vec<LineDiff>, String> {
    use std::process::Command;
    use regex::Regex;

    let output = Command::new("git")
        .args(&["-C", &path, "diff", "HEAD", "--unified=0", "--", &file_path])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let diff_text = String::from_utf8_lossy(&output.stdout);
    let mut results: Vec<LineDiff> = Vec::new();

    // Find all hunk header positions: @@ -old,count +new,count @@
    let hunk_re = Regex::new(r"@@ -\d+(?:,\d+)? \+(\d+)(?:,\d+)? @@").unwrap();
    let headers: Vec<(usize, usize, u32)> = hunk_re
        .captures_iter(&diff_text)
        .map(|cap| {
            let m = cap.get(0).unwrap();
            let start_line: u32 = cap[1].parse().unwrap_or(1);
            // Find the newline after the header (header may have trailing context like " @@ foo")
            let newline_pos = diff_text[m.end()..].find('\n').map(|p| m.end() + p + 1).unwrap_or(diff_text.len());
            (m.start(), newline_pos, start_line)
        })
        .collect();

    for (i, &(_hdr_start, body_start, start_line)) in headers.iter().enumerate() {
        // Body ends at next header or end of diff
        let body_end = if i + 1 < headers.len() {
            headers[i + 1].0
        } else {
            diff_text.len()
        };

        let hunk_body = &diff_text[body_start..body_end];
        let mut current_line = start_line;
        let mut added_lines: Vec<u32> = Vec::new();
        let mut deleted_count: u32 = 0;

        for line in hunk_body.lines() {
            if line.starts_with('+') {
                added_lines.push(current_line);
                current_line += 1;
            } else if line.starts_with('-') {
                deleted_count += 1;
            } else if !line.is_empty() {
                current_line += 1;
            }
        }

        let status = if deleted_count > 0 {
            "modified"
        } else {
            "added"
        };

        for line_num in &added_lines {
            results.push(LineDiff {
                line: *line_num,
                status: status.to_string(),
            });
        }
    }

    Ok(results)
}

#[tauri::command]
fn quick_open(path: String) -> Result<Vec<String>, String> {
    use std::process::Command;
    
    // Try common fd locations
    let fd_paths = vec!["fd", "/usr/bin/fd", "/usr/local/bin/fd", "/home/bungeist/.pi/agent/bin/fd"];
    let mut last_error = String::new();
    
    for fd_cmd in fd_paths {
        let result = Command::new(fd_cmd)
            .args(&["--type", "f", "--hidden", "--exclude", ".git"])
            .current_dir(&path)
            .output();
        
        match result {
            Ok(output) => {
                if output.status.success() {
                    let files = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .map(String::from)
                        .collect();
                    return Ok(files);
                } else {
                    last_error = format!("fd command failed: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                last_error = format!("{}: {}", fd_cmd, e);
                continue;
            }
        }
    }
    
    Err(format!("fd not found. Last error: {}. Install with: sudo apt install fd-find", last_error))
}

#[tauri::command]
fn get_home_dir() -> Result<String, String> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn zoxide_query(query: String) -> Result<Vec<String>, String> {
    use std::process::Command;
    
    // Try zoxide first
    let output = Command::new("zoxide")
        .args(&["query", "-l", &query])
        .output();
    
    if let Ok(output) = output {
        if output.status.success() {
            let paths = String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(String::from)
                .collect();
            return Ok(paths);
        }
    }
    
    // Fallback to zshz database if zoxide fails
    use std::fs;
    use std::path::Path;
    
    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    let zshz_db = Path::new(&home).join(".z");
    
    if zshz_db.exists() {
        let content = fs::read_to_string(&zshz_db)
            .map_err(|e| format!("Failed to read zshz database: {}", e))?;
        
        let mut entries: Vec<(String, f64)> = content
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 2 {
                    let path = parts[0].to_string();
                    let rank: f64 = parts[1].parse().unwrap_or(0.0);
                    
                    if query.is_empty() || path.to_lowercase().contains(&query.to_lowercase()) {
                        Some((path, rank))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        
        entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        let paths: Vec<String> = entries.into_iter().map(|(path, _)| path).collect();
        return Ok(paths);
    }
    
    Err("No z tool found (tried zoxide and zshz)".to_string())
}

#[tauri::command]
fn resolve_path(path: String, cwd: Option<String>) -> Result<String, String> {
    use std::path::PathBuf;
    
    let path_buf = PathBuf::from(&path);
    
    // If already absolute, return as-is
    if path_buf.is_absolute() {
        return Ok(path_buf.to_string_lossy().to_string());
    }
    
    // Use provided CWD or fall back to env current_dir
    let base_dir = if let Some(cwd_str) = cwd {
        PathBuf::from(cwd_str)
    } else {
        std::env::current_dir().map_err(|e| e.to_string())?
    };
    
    let absolute = base_dir.join(&path);
    
    // Canonicalize to resolve .. and . 
    match absolute.canonicalize() {
        Ok(canonical) => Ok(canonical.to_string_lossy().to_string()),
        Err(_) => Ok(absolute.to_string_lossy().to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Capture the actual current working directory before anything changes
            let cwd = std::env::current_dir()
                .ok()
                .and_then(|p| p.to_str().map(String::from))
                .unwrap_or_else(|| String::from("/"));
            
            // Capture command-line arguments
            let args: Vec<String> = std::env::args().collect();
            
            // Skip the first arg (executable path)
            let file_args: Vec<String> = args.into_iter().skip(1).collect();
            
            // If there are file arguments, emit them to the frontend with CWD
            if !file_args.is_empty() {
                // Clone for the async block
                let file_args_clone = file_args.clone();
                let cwd_clone = cwd.clone();
                let app_handle = app.handle().clone();
                
                // Wait for window to be ready before emitting
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(1500));
                    
                    if let Some(window) = app_handle.get_webview_window("main") {
                        // Create payload with both args and cwd
                        #[derive(Clone, serde::Serialize)]
                        struct CliPayload {
                            args: Vec<String>,
                            cwd: String,
                        }
                        
                        window.emit("cli-args", CliPayload {
                            args: file_args_clone,
                            cwd: cwd_clone,
                        }).unwrap();
                    }
                });
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            read_file,
            write_file,
            read_dir,
            get_git_branch,
            git_status,
            git_line_diff,
            quick_open,
            get_home_dir,
            zoxide_query,
            resolve_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
