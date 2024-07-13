use std::{fs, io};

pub fn init() -> Result<(), std::io::Error> {
    fs::create_dir_all(".minigit/objects")?;
    fs::create_dir_all(".minigit/refs/heads")?;
    fs::create_dir_all(".minigit/refs/tags")?;
    Ok(())
}
