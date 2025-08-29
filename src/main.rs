use clap::{Args, Parser, Subcommand};

use crate::ffi::{Status, priority::Priority};

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

    /// List tickets
    #[command(name = "ls")]
    List(ListArgs),

    /// Show detailed information about a ticket
    Show(ShowArgs),
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

#[derive(Args)]
struct ListArgs {
    /// Show detailed information
    #[arg(short, long)]
    long: bool,

    /// Filter by status (can be specified multiple times)
    #[arg(short = 's', long = "status", value_enum)]
    status: Vec<Status>,

    /// Filter by priority (can be specified multiple times)
    #[arg(short = 'p', long = "priority", value_enum)]
    priority: Vec<Priority>,
}

#[derive(Args)]
struct ShowArgs {
    /// Ticket ID
    ticket_id: String,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => handle_init(),
        Commands::Config(args) => handle_config(args),
        Commands::Switch(args) => handle_switch(args),
        Commands::Add(args) => handle_add(args),
        Commands::Mv(args) => handle_mv(args),
        Commands::List(args) => handle_list(args),
        Commands::Show(args) => handle_show(&args.ticket_id),
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

fn handle_list(args: ListArgs) -> anyhow::Result<()> {
    // Default to todo and doing if no status filter provided
    let statuses = if args.status.is_empty() {
        vec![Status::todo, Status::doing]
    } else {
        args.status
    };

    let tickets = ffi::list(args.long, statuses, args.priority)?;

    if tickets.is_empty() {
        println!("No tickets found.");
        return Ok(());
    }

    if args.long {
        // Find max title length for alignment
        let max_title_len = tickets.iter().map(|t| t.title.len()).max().unwrap_or(0);

        for ticket in tickets {
            // Detailed view with aligned columns: ID title [priority] status
            println!(
                "{} {:<width$} [{:?}] {:?}",
                ticket.id,
                ticket.title,
                ticket.priority,
                ticket.status,
                width = max_title_len
            );
        }
    } else {
        for ticket in tickets {
            // Simple view: just ID and title
            println!("{} {}", ticket.id, ticket.title);
        }
    }

    Ok(())
}

fn handle_show(ticket_id: &str) -> anyhow::Result<()> {
    // Check if the ticket_id contains a field specifier (e.g., "01K3XXX:title")
    if let Some(colon_pos) = ticket_id.find(':') {
        let id = &ticket_id[..colon_pos];
        let field = &ticket_id[colon_pos + 1..];
        
        match field {
            "title" => {
                let title = ffi::show_title(id)?;
                println!("{}", title);
            }
            "body" => {
                let body = ffi::show_body(id)?;
                println!("{}", body);
            }
            "status" => {
                let status = ffi::show_status(id)?;
                println!("{:?}", status);
            }
            "priority" => {
                let priority = ffi::show_priority(id)?;
                println!("{:?}", priority);
            }
            _ => {
                eprintln!("Unknown field: {}. Valid fields are: title, body, status, priority", field);
                std::process::exit(1);
            }
        }
    } else {
        // No field specifier, show the full ticket
        let ticket = ffi::show(ticket_id)?;
        println!("ID: {}", ticket.id);
        println!("Title: {}", ticket.title);
        println!("Status: {:?}", ticket.status);
        println!("Priority: {:?}", ticket.priority);
        if let Some(body) = ticket.body {
            println!("Body:\n{body}");
        }
    }
    Ok(())
}

