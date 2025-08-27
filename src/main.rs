use clap::{Args, Parser, Subcommand};

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

    /// Get or set configuration values
    Config(ConfigArgs),

    /// Switch to a different ticket
    Switch(SwitchArgs),
}

#[derive(Args)]
struct ConfigArgs {
    /// Configuration key (e.g., user.name)
    key: String,

    /// Configuration value (if setting)
    value: Option<String>,
}

#[derive(Args)]
struct SwitchArgs {
    /// Project name
    project: String,

    /// Create the project if it doesn't exist
    #[arg(short, long)]
    create: bool,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => handle_init(),
        Commands::Config(args) => handle_config(args),
        Commands::Switch(args) => handle_switch(args),
    };

    if let Err(err) = result {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn handle_init() -> anyhow::Result<()> {
    let result = ffi::init()?;
    println!("{result}");
    Ok(())
}

fn handle_config(args: ConfigArgs) -> anyhow::Result<()> {
    if let Some(value) = args.value {
        ffi::config_set(&args.key, &value)?;
    } else {
        let value = ffi::config_get(&args.key)?;
        println!("{value}");
    }
    Ok(())
}

fn handle_switch(args: SwitchArgs) -> anyhow::Result<()> {
    let result = ffi::switch(&args.project, args.create)?;
    println!("{result}");
    Ok(())
}
