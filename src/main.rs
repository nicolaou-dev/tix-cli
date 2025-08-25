use clap::{Parser, Subcommand};

mod ffi;

#[derive(Parser)]
#[command(name = "tix")]
#[command(about = "Tix filesystem-based ticket management system", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new tix workspace
    Init,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => match ffi::init() {
            Ok(result) => println!("{result}"),
            Err(e) => eprintln!("Error: {e}"),
        },
    }
}
