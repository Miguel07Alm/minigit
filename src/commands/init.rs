use std::{fs, io, path::Path};

pub fn init() {
    fs::create_dir_all(".minigit/objects").expect("Failed to create .minigit directory");
    fs::create_dir_all(".minigit/refs/heads").expect("Failed to create .minigit directory");
    fs::create_dir_all(".minigit/refs/tags").expect("Failed to create .minigit directory");
    fs::write(".minigit/HEAD", "ref: refs/heads/master").expect("Failed to create HEAD file");
    println!("Initialized empty minigit repository");

    if !Path::new(".minigit/config").exists() {
        configure_minigit();
    }
}
pub fn configure_minigit() {
    // Solicitar el nombre del usuario
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();

    // Solicitar el correo del usuario
    println!("Enter your email:");
    let mut email = String::new();
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line");
    let email = email.trim();

    // Crear el contenido del archivo de configuración
    let config_content = format!("[user]\nname = \"{}\"\nemail = \"{}\"\n", name, email);

    // Escribir el archivo de configuración
    fs::create_dir_all(".minigit").expect("Failed to create .minigit directory");
    fs::write(".minigit/config", config_content).expect("Failed to write config file");

    println!("Configuration written to .minigit/config");
}
