use std::collections::HashMap;

use crate::utils::{collect_entries, compare_trees, get_commit_tree, get_head_commit};

pub fn status() {
    // Read the current commit hash from .minigit/HEAD
    let head_commit = match get_head_commit() {
        Some(commit) => commit,
        None => {
            println!("No commits yet.");
            return ();
        }
    };

    // Get the tree object associated with the current commit
    match get_commit_tree(&head_commit) {
        Ok(current_tree) => {
            // Collect the current working directory state
            let mut index = HashMap::new();
            collect_entries(".", &mut index);

            // Compare the working directory with the current tree
            compare_trees(&current_tree, &index);
        }
        Err(err) => {
            eprintln!("Error getting tree: {}", err);
        }
    }
}
