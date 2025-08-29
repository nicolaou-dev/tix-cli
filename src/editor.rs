use crate::ffi::priority::Priority;
use anyhow::Result;
use std::process::Command;

// Embed template at compile time
const TICKET_TEMPLATE: &str = include_str!("../templates/ticket.txt");

pub fn open_editor_for_ticket() -> Result<(String, Option<String>, Priority)> {
    // Create a temporary file with template
    let temp_file = std::env::temp_dir().join(format!("tix_ticket_{}.txt", std::process::id()));

    // Use embedded template
    std::fs::write(&temp_file, TICKET_TEMPLATE)?;

    // Open editor (try EDITOR env var, fallback to vi)
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let status = Command::new(&editor).arg(&temp_file).status()?;

    if !status.success() {
        anyhow::bail!("Editor exited with error");
    }

    // Parse the file content
    let content = std::fs::read_to_string(&temp_file)?;
    std::fs::remove_file(&temp_file).ok(); // Clean up temp file

    // Parse the content
    let (title, body, priority) = parse_ticket_template(&content)?;
    
    // Abort if title is empty
    if title.is_empty() {
        anyhow::bail!("Aborting due to empty ticket message");
    }
    
    Ok((title, body, priority))
}

fn parse_ticket_template(content: &str) -> Result<(String, Option<String>, Priority)> {
    let lines = content.lines();
    let mut title = String::new();
    let mut priority = Priority::z;
    let mut in_header = false;
    let mut in_body = false;
    let mut body = Vec::new();

    for line in lines {
        if line.starts_with("---") {
            if !in_header {
                in_header = true; // First --- starts header
            } else {
                in_header = false; // Second --- ends header, starts body
                in_body = true;
            }
            continue;
        }

        if in_body {
            if !line.starts_with('#') {
                body.push(line);
            }
        } else if in_header {
            if line.starts_with("Title:") {
                title = line.strip_prefix("Title:").unwrap_or("").trim().to_string();
            } else if line.starts_with("Priority:") {
                let p = line
                    .strip_prefix("Priority:")
                    .unwrap_or("")
                    .trim()
                    .to_lowercase();
                priority = match p.as_str() {
                    "a" => Priority::a,
                    "b" => Priority::b,
                    "c" => Priority::c,
                    "z" => Priority::z,
                    _ => Priority::None,
                };
            }
        }
    }

    // Don't fail here - let the caller check if it's empty

    let body_text = body.join("\n");
    let body = if body_text.trim().is_empty() {
        None
    } else {
        Some(body_text)
    };

    Ok((title, body, priority))
}

