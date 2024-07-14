use crate::utils::*;
use chrono::Utc;
use std::{collections::HashMap, fs, io::Write};

pub fn commit(message: &str, author: &str) {
    // Step 1: Generate the tree hash
    let mut index = HashMap::new();
    collect_entries(".", &mut index);
    let tree_hash = write_tree(&index);

    // Step 2: Get the parent commit hash
    let parent_commit = get_head_commit();

    // Step 3: Create the commit content
    let timestamp = Utc::now().timestamp();
    let commit_content = if let Some(parent) = parent_commit {
        format!(
            "tree {}\nparent {}\nauthor {} {}\n\n{}",
            tree_hash, parent, author, timestamp, message
        )
    } else {
        format!(
            "tree {}\nauthor {} {}\n\n{}",
            tree_hash, author, timestamp, message
        )
    };

    // Step 4: Calculate the commit hash
    let commit_hash = hash_object(commit_content.as_bytes(), "commit");

    // Step 5: Store the commit object
    let dir_path = format!(".minigit/objects/{}", &commit_hash[0..2]);
    let file_path = format!("{}/{}", dir_path, &commit_hash[2..]);

    fs::create_dir_all(&dir_path).expect("Failed to create directories");
    fs::write(&file_path, commit_content).expect("Failed to write commit");

    // Step 6: Update the HEAD to point to the new commit
    fs::write(".minigit/HEAD", &commit_hash).expect("Failed to update HEAD");

    // Step 7: Append the new commit to the log
    let log_entry = format!("{}\n", commit_hash);
    fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(".minigit/log")
        .expect("Failed to open log file")
        .write_all(log_entry.as_bytes())
        .expect("Failed to write to log file");

    println!("Created commit with hash {}", commit_hash);
}
