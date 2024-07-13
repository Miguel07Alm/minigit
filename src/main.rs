mod commands;
use clap::Parser;
use commands::init::init;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// git init
    init: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    println!("{:?}", args.init);
    if args.init.is_some_and(|arg| arg.eq("init")) {
        println!("s");
        init()?;
    }
    Ok(())
}
