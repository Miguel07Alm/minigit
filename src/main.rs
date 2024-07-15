mod commands;
mod utils;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No command provided");
        return;
    }

    match args[1].as_str() {
        "init" => commands::init::init(),
        "add" => {
            if args.len() < 3 {
                eprintln!("No file specified to add");
            } else {
                commands::add::add(&args[2]);
            }
        }
        "commit" => {
            commands::commit::commit();
        }
        "status" => commands::status::status(),
        _ => eprintln!("Unknown command"),
    }
}
