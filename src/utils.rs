use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use sha1::{Digest, Sha1};
pub fn hash_object(data: &[u8], obj_type: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(format!("{} {}\0", obj_type, data.len()).as_bytes());
    hasher.update(data);
    hex::encode(hasher.finalize())
}
pub fn process_entry(entry: &Path) {
    if entry.is_dir() {
        println!("{:?} is a directory. Entering directory.", entry);
        match fs::read_dir(entry) {
            Ok(sub_entries) => {
                for sub_entry in sub_entries {
                    match sub_entry {
                        Ok(sub_entry) => process_entry(&sub_entry.path()),
                        Err(err) => eprintln!("Error reading subdirectory: {}", err),
                    }
                }
            }
            Err(err) => eprintln!("Error reading directory: {}", err),
        }
    } else {
        let content = match fs::read(entry) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Failed to read file {:?}: {}", entry, err);
                return;
            }
        };

        let hash = hash_object(&content, "blob");
        println!("hash: {}", &hash);

        let dir_path = format!(".minigit/objects/{}", &hash[0..2]);
        println!("dir_path: {}", &dir_path);

        let file_path = format!("{}/{}", dir_path, &hash[2..]);
        println!("file_path: {}", &file_path);

        if !Path::new(&file_path).exists() {
            match fs::create_dir_all(&dir_path) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Failed to create directories {}: {}", dir_path, err);
                    return;
                }
            }

            match fs::write(&file_path, &content) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Failed to write object {}: {}", file_path, err);
                    return;
                }
            }
        }

        println!("Added file {} to index with hash {}", entry.display(), hash);
    }
}

pub fn collect_entries(path: &str, index: &mut HashMap<String, String>) {
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Error reading directory {}: {}", path, err);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error reading entry: {}", err);
                continue;
            }
        };
        let path = entry.path();

        if path.is_dir() {
            collect_entries(path.to_str().unwrap(), index);
        } else {
            let content = match fs::read(&path) {
                Ok(content) => content,
                Err(err) => {
                    eprintln!("Failed to read file {:?}: {}", path, err);
                    continue;
                }
            };
            let hash = hash_object(&content, "blob");
            index.insert(path.to_str().unwrap().to_string(), hash);
        }
    }
}

pub fn write_tree(index: &HashMap<String, String>) -> String {
    let mut tree_entries = Vec::new();

    for (path, hash) in index {
        let entry = format!("{} {}\0{}", "100644", path, hash);
        tree_entries.push(entry);
    }

    let tree_content = tree_entries.join("");
    hash_object(tree_content.as_bytes(), "tree")
}

pub fn get_head_commit() -> Option<String> {
    match fs::read_to_string(".minigit/HEAD") {
        Ok(commit_hash) => Some(commit_hash.trim().to_string()),
        Err(_) => None,
    }
}
