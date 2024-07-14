use std::fs;

pub fn init() {
    fs::create_dir_all(".minigit/objects").expect("Failed to create .minigit directory");
    fs::create_dir_all(".minigit/refs/heads").expect("Failed to create .minigit directory");
    fs::create_dir_all(".minigit/refs/tags").expect("Failed to create .minigit directory");
    fs::write(".minigit/HEAD", "ref: refs/heads/master").expect("Failed to create HEAD file");
    println!("Initialized empty minigit repository");
}
