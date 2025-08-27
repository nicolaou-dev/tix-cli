use clap::{Args, Parser, Subcommand};

use crate::ffi::{Status, add::Priority};

mod editor;
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

    /// Add a new ticket
    Add(AddArgs),

    /// Update ticket status
    Mv(MvArgs),
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

#[derive(Args)]
struct AddArgs {
    /// Ticket title
    #[arg(short, long)]
    title: Option<String>,

    /// Ticket body
    #[arg(short, long)]
    body: Option<String>,

    /// Ticket priority
    #[arg(short, long, default_value = "none")]
    priority: Priority,
}

#[derive(Args)]
struct MvArgs {
    /// Ticket ID
    ticket_id: String,

    /// New status
    status: Status,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => handle_init(),
        Commands::Config(args) => handle_config(args),
        Commands::Switch(args) => handle_switch(args),
        Commands::Add(args) => handle_add(args),
        Commands::Mv(args) => handle_mv(args),
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

fn handle_add(args: AddArgs) -> anyhow::Result<()> {
    let (title, body, priority) = if let Some(title) = args.title {
        // Use provided arguments
        (title, args.body, args.priority)
    } else {
        // Open editor for interactive input
        editor::open_editor_for_ticket()?
    };

    let result = ffi::add(&title, body.as_deref(), priority)?;
    println!("{result}");
    Ok(())
}

fn handle_mv(args: MvArgs) -> anyhow::Result<()> {
    ffi::mv(&args.ticket_id, args.status)?;
    println!(
        "Ticket {} status updated to {:?}",
        args.ticket_id, args.status
    );
    Ok(())
}
