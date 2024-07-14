use crate::utils::*;
use chrono::Utc;
use std::{collections::HashMap, fs};

pub fn commit(message: &str, author: &str) {
    // Step 1: Generate the tree hash
    let mut index = HashMap::new();
    collect_entries(".", &mut index);
    let tree_hash = write_tree(&index);

    // Step 2: Get the parent commit hash
    let parent_commit = get_head_commit();

    // Step 3: Create the commit content
    let timestamp = Utc::now().timestamp();
    let commit_content = format!(
        "tree {}\nparent {}\nauthor {} {}\n\n{}",
        tree_hash,
        parent_commit.unwrap_or_default(),
        author,
        timestamp,
        message
    );

    // Step 4: Calculate the commit hash
    let commit_hash = hash_object(commit_content.as_bytes(), "commit");

    // Step 5: Store the commit object
    let dir_path = format!(".minigit/objects/{}", &commit_hash[0..2]);
    let file_path = format!("{}/{}", dir_path, &commit_hash[2..]);

    fs::create_dir_all(&dir_path).expect("Failed to create directories");
    fs::write(&file_path, commit_content).expect("Failed to write commit");

    // Step 6: Update the HEAD to point to the new commit
    fs::write(".minigit/HEAD", &commit_hash).expect("Failed to update HEAD");

    println!("Created commit with hash {}", commit_hash);
}
