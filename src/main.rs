use clap::{Args, Parser, Subcommand};
use std::time::Instant;

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

    /// Amend a ticket (modify title, body, priority)
    Amend(AmendArgs),

    /// Undo the last change
    Undo,

    /// Redo the last undone change
    Redo,

    /// Show commit history
    Log(LogArgs),

    /// List all local projects
    Projects,

    /// Remote repository operations
    Remote(RemoteArgs),

    /// Push changes to remote repository
    Push(PushArgs),

    /// Pull changes from remote repository
    Pull,

    /// Clone a remote repository
    Clone(CloneArgs),
}

#[derive(Args)]
struct AmendArgs {
    /// Ticket ID
    ticket_id: String,

    /// New ticket title
    #[arg(short, long)]
    title: Option<String>,

    /// New ticket body
    #[arg(short, long)]
    body: Option<String>,

    /// New ticket priority
    #[arg(short, long)]
    priority: Option<Priority>,
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

    /// Ticket status
    #[arg(short, long)]
    status: Option<Status>,
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

#[derive(Args)]
struct LogArgs {
    /// Show one line per commit
    #[arg(long)]
    oneline: bool,

    /// Limit number of commits
    #[arg(short, long)]
    limit: Option<i32>,

    /// Show commits since date (e.g., "2 days ago")
    #[arg(short, long)]
    since: Option<String>,
}

#[derive(Args)]
struct RemoteArgs {
    #[command(subcommand)]
    command: Option<RemoteCommands>,

    /// Show remote URLs (when listing)
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum RemoteCommands {
    /// Add a remote repository
    Add(RemoteAddArgs),
}

#[derive(Args)]
struct RemoteAddArgs {
    /// Remote repository URL
    url: String,
}

#[derive(Args)]
struct CloneArgs {
    /// Repository URL to clone
    url: String,
}

#[derive(Args)]
struct PushArgs {
    /// Force push (--force)
    #[arg(long)]
    force: bool,

    /// Force push with lease (--force-with-lease)
    #[arg(long)]
    force_with_lease: bool,
}


fn main() {
    let cli = Cli::parse();

    let start = Instant::now();
    let result = match cli.command {
        Commands::Init => handle_init(),
        Commands::Config(args) => handle_config(args),
        Commands::Switch(args) => handle_switch(args),
        Commands::Add(args) => handle_add(args),
        Commands::Mv(args) => handle_mv(args),
        Commands::List(args) => handle_list(args),
        Commands::Show(args) => handle_show(&args.ticket_id),
        Commands::Amend(args) => handle_amend(args),
        Commands::Undo => handle_undo(),
        Commands::Redo => handle_redo(),
        Commands::Log(args) => handle_log(args),
        Commands::Projects => handle_projects(),
        Commands::Remote(args) => handle_remote(args),
        Commands::Push(args) => handle_push(args),
        Commands::Pull => handle_pull(),
        Commands::Clone(args) => handle_clone(args),
    };
    let duration = start.elapsed();

    match result {
        Ok(_) => {
            eprintln!("Command completed in {:.2?}", duration);
        }
        Err(err) => {
            eprintln!("Command failed in {:.2?}: {err}", duration);
            std::process::exit(1);
        }
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
    let (title, body, priority, status) = if let Some(title) = args.title {
        // Use provided arguments
        (title, args.body, args.priority, args.status)
    } else {
        // Open editor for interactive input
        editor::open_editor_for_ticket()?
    };

    let result = ffi::add(&title, body.as_deref(), priority, status)?;
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
                eprintln!(
                    "Unknown field: {}. Valid fields are: title, body, status, priority",
                    field
                );
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

fn handle_amend(args: AmendArgs) -> anyhow::Result<()> {
    // Check if any flags are provided
    let has_flags = args.title.is_some() || args.body.is_some() || args.priority.is_some();

    if has_flags {
        // Use provided flags directly - if user provided it, pass it
        ffi::amend(&args.ticket_id, args.title.as_deref(), args.body.as_deref(), args.priority)?;
    } else {
        // No flags provided - open editor (returns only changed fields)
        let current_ticket = ffi::show(&args.ticket_id)?;
        let (title_opt, body_opt, priority_opt) = editor::open_editor_for_ticket_amend(&current_ticket)?;
        
        ffi::amend(&args.ticket_id, title_opt.as_deref(), body_opt.as_deref(), priority_opt)?;
    }

    println!("Ticket {} amended successfully", args.ticket_id);
    Ok(())
}

fn handle_undo() -> anyhow::Result<()> {
    let result = ffi::undo()?;
    println!("{result}");
    Ok(())
}

fn handle_redo() -> anyhow::Result<()> {
    let result = ffi::redo()?;
    println!("{result}");
    Ok(())
}

fn handle_log(args: LogArgs) -> anyhow::Result<()> {
    let result = ffi::log(args.oneline, args.limit, args.since.as_deref())?;
    
    if let Ok(mut pager) = std::process::Command::new("less")
        .args(["-R", "-F"])
        .stdin(std::process::Stdio::piped())
        .spawn()
    {
        if let Some(stdin) = pager.stdin.as_mut() {
            use std::io::Write;
            let _ = stdin.write_all(result.as_bytes());
        }
        let _ = pager.wait();
    } else {
        print!("{result}");
    }
    
    Ok(())
}

fn handle_projects() -> anyhow::Result<()> {
    let projects = ffi::projects()?;
    
    if projects.is_empty() {
        println!("No projects found.");
        return Ok(());
    }

    for (i, project) in projects.iter().enumerate() {
        if i == 0 {
            println!("* {}", project);
        } else {
            println!("  {}", project);
        }
    }
    
    Ok(())
}

fn handle_remote(args: RemoteArgs) -> anyhow::Result<()> {
    match args.command {
        Some(RemoteCommands::Add(add_args)) => {
            let result = ffi::remote_add(&add_args.url)?;
            println!("{result}");
        }
        None => {
            // List remotes
            let result = ffi::remote(args.verbose)?;
            
            if result.is_empty() {
                println!("No remotes configured.");
            } else {
                print!("{result}");
            }
        }
    }
    
    Ok(())
}

fn handle_push(args: PushArgs) -> anyhow::Result<()> {
    let result = ffi::push(args.force, args.force_with_lease)?;
    println!("{result}");
    Ok(())
}

fn handle_pull() -> anyhow::Result<()> {
    let result = ffi::pull()?;
    println!("{result}");
    Ok(())
}

fn handle_clone(args: CloneArgs) -> anyhow::Result<()> {
    let result = ffi::clone(&args.url)?;
    println!("{result}");
    Ok(())
}
