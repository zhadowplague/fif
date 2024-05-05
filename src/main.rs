use std::fs;
use std::env;
use std::path::PathBuf;

fn search_directory(path:&PathBuf, content:&String) {
    let dir = fs::read_dir(path);
    if dir.is_err() {
        println!("Failed to open dir {}", path.to_str().unwrap_or("Err"));
        return;
    }
    for entry in dir.unwrap() {
        if entry.is_err() {
            println!("Failed to get dir entry {}", entry.unwrap_err());
            continue;
        }
        let u_entry = entry.unwrap();

        let entry_metadata = u_entry.metadata();
        if entry_metadata.is_err() {
            println!("Failed to aquire file metadata {}", entry_metadata.unwrap_err());
            continue;
        }
        let u_entry_metadata = entry_metadata.unwrap();

        if u_entry_metadata.is_file() {
            let file_content = fs::read_to_string(u_entry.path());
            if file_content.is_err(){
                continue;
            }
            let u_file_content = file_content.unwrap();
            if u_file_content.contains(content) {
                println!("Found in {}", u_entry.path().to_str().unwrap_or("Err"));
            }
        }
        else {
            let dir_path = u_entry.path();
            search_directory(&dir_path, &content);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3
    {
        println!("Usage: fif [textToFind] [filePath]");
        return;
    }

    let content_to_find = &args[1];
    let file_path = &args[2];
    let exe_path = process_path::get_executable_path();
    if exe_path.is_some() {
        let mut u_exe_path = exe_path.unwrap();
        u_exe_path.push(&file_path);
        search_directory(&u_exe_path, &content_to_find);
    }
    else {
        let mut path = PathBuf::new();
        path.push(&file_path);
        search_directory(&path, &content_to_find);
    }
}
