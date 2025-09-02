# Tix CLI

A git-native project management tool that treats tickets as filesystem entities and projects as git branches.

**This project is managed by tix. You can find the ticket data at:** https://github.com/nicolaou-dev/.tix

## Related Repositories

- **[tix](https://github.com/nicolaou-dev/tix)** - Core Zig library providing the C API
- **[tix-cli](https://github.com/nicolaou-dev/tix-cli)** - This command-line interface (Rust)
- **[tix-ui](https://github.com/nicolaou-dev/tix-ui)** - Web interface for tix workspaces _(planned)_

## What is Tix?

Tix leverages git's powerful version control as its foundation, with each project as a git branch. Tickets are stored as directories on your filesystem with [ULID](https://github.com/nicolaou-dev/ulid.zig) identifiers, providing:

**Benefits:**

- **Git-native** - Uses familiar git workflows (push, pull, branches)
- **Filesystem-based** - Tickets are directories you can see (but you should only edit via tix)
- **Unix-friendly** - Works seamlessly with grep, find, awk, and other Unix tools
- **AI-ready** - Your data stays local; use any AI to query, analyze, or generate reports
- **Data ownership** - No vendor lock-in; you own all your data in simple files
- **Distributed** - Works offline, syncs when ready
- **No database** - Just files and git history
- **Time-ordered** - ULIDs naturally sort by creation time
- **Atomic operations** - Every change is immediately committed
- **Extensible** - Add custom attributes as files

**Structure:**

```
my-app/                            # Your actual project code
├── src/
├── package.json
└── .tix/                          # Ticket data (separate git repository managed by tix)
    ├── .git/                      # Git history for tickets
    ├── 01HQXW5P7R8ZYFG9K3NMVBCXSD/
    │   ├── s=t                    # Status: b=backlog, t=todo, w=doing, d=done
    │   ├── p=a                    # Priority: a=high, b=medium, c=low, z=default
    │   ├── title.md               # Ticket title
    │   └── body.md                # Ticket description
    └── 01HQXW6QA2TMDFE4H8RNJYWKPB/
        └── ...
```

**Every tix operation = git commit** - providing complete audit trail and history.

Unlike Jira/Linear, **your data stays yours** - no vendor lock-in, no API limitations, no central server dependencies.

Tix makes it easy to work with your data: always use tix commands to modify tickets, but when you need queries not implemented in [tix core](https://github.com/nicolaou-dev/tix) or tix CLI, you can easily query your data using Unix commands or AI tools. All your data lives in `.tix` directories (locally and in the cloud - github/gitlab/etc).

**Example queries not implemented in tix yet**

```bash
# Unix tools work directly on your data
tix ls | grep "login"                          # Search ticket titles
or cd .tix && grep -r "login" */title.md

find . -name "s=t" | wc -l                     # Count todo tickets
```

## Philosophy

Tix follows the **Unix philosophy** - do one thing well. The core focuses on essential ticket operations (add, show, list, move, sync). Advanced features like user assignment, time tracking, custom workflows, and integrations are built as `tix-*` extensions that follow filesystem conventions.

**Extension model:**

- **No plugin API** - extensions are just executables named `tix-command`
- **Filesystem as API** - extensions read/write files in `.tix/` directories
- **Zero coordination** - extensions discover each other through file conventions
- **Any language** - write extensions in whatever language you prefer

**Example: User assignment with `tix-assign` extension**

```bash
# Seamless user experience - tix core delegates to tix-users executable
tix assign 01HQXW5P7R8ZYFG9K3NMVBCXSD alice
# Creates .tix/01HQXW5P7R8ZYFG9K3NMVBCXSD/assigned_alice

tix unassign 01HQXW5P7R8ZYFG9K3NMVBCXSD
# Removes assignment file

tix ls | tix filter --assigned alice
# Pipeline: core lists tickets, extension filters by assignment
```

## Performance

Tix is **blazingly fast** because it's built with native code (Zig core library, Rust CLI). Operations that take seconds in web-based tools happen **instantly** with tix - no network requests, no database queries, just native code working directly with your filesystem.

**Distribution benefits:**

- **No central server** - your data is cloned locally and in the cloud
- **Works offline** - full functionality without internet
- **True ownership** - you control where your data lives

## Installation

### Quick Install (Unix)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/nicolaou-dev/tix-cli/releases/latest/download/tix-installer.sh | sh
```

### Quick Install (Windows)

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/nicolaou-dev/tix-cli/releases/latest/download/tix-installer.ps1 | iex"
```

### Build from Source

```bash
git clone https://github.com/nicolaou-dev/tix-cli
cd tix-cli
cargo build --release
# Binary will be at target/release/tix
```

## Commands

### Workspace Management

```bash
tix init                           # Initialize new tix workspace
                                   # Initialized empty tix repository

tix config user.name "Your Name"   # Set configuration
tix config user.email "you@example.com"
tix config user.name               # Get configuration value
                                   # Your Name
```

### Ticket Management

```bash
tix add                            # Open editor to create ticket
                                   # Created ticket: 01HQXW5P7R8ZYFG9K3NMVBCXSD...

tix add -t "Fix bug" -b "Details" -p a -s todo
                                   # Create with flags
                                   # Created ticket: 01HQXW5P7R8ZYFG9K3NMVBCXSD...

tix ls                             # List tickets (shows todo + doing by default, not backlog)
                                   # 01HQXW5P7R8ZYFG9K3NMVBCXSD Fix bug
                                   # 01HQXW6QA2TMDFE4H8RNJYWKPB Add feature

tix ls -l                          # Detailed view with status/priority
                                   # 01HQXW5P7R8ZYFG9K3NMVBCXSD Fix bug      [a] todo
                                   # 01HQXW6QA2TMDFE4H8RNJYWKPB Add feature  [b] doing

tix ls -s todo -s doing            # Filter by multiple statuses
tix ls -p a -p b                   # Filter by multiple priorities
tix ls -s todo -p a                # Combine status and priority filters

tix ls | grep "login"              # Search ticket titles
                                   # 01HQXW5P7R8ZYFG9K3NMVBCXSD Fix login bug
                                   # 01HQXW6QA2TMDFE4H8RNJYWKPB Update login page

tix show 01HQXW5P7R8ZYFG9K3NMVBCXSD                   # Show full ticket details
                                   # ID: 01HQXW5P7R8ZYFG9K3NMVBCXSD
                                   # Title: Fix bug
                                   # Status: todo
                                   # Priority: a
                                   # Body: Details...

tix show 01HQXW5P7R8ZYFG9K3NMVBCXSD:title             # Show specific field
                                   # Fix bug

tix mv 01HQXW5P7R8ZYFG9K3NMVBCXSD doing  # Update ticket status (full ULID required)
                                   # Ticket 01HQXW5P7R8ZYFG9K3NMVBCXSDXXXXXXXXXXXXXXXXXX status updated to doing

tix amend 01HQXW5P7R8ZYFG9K3NMVBCXSD                  # Open editor to modify ticket
tix amend 01HQXW5P7R8ZYFG9K3NMVBCXSD -t "New title"   # Update specific field
                                   # Ticket 01HQXW5P7R8ZYFG9K3NMVBCXSD amended successfully
```

### History & Navigation

```bash
tix undo                           # Undo last change
                                   # Undid last change

tix redo                           # Redo last undone change
                                   # Redid last undone change

tix log                            # Show change history (opens in pager)
tix log --oneline                  # Show one line per entry
tix log -1                         # Show one line per entry (short)
tix log -l 10                      # Limit number of entries (short)
tix log --limit 10                 # Limit number of entries
tix log -s "2 days ago"            # Show changes since date (short)
tix log --since "2 days ago"       # Show changes since date

tix switch project-name            # Switch to different project
                                   # Switched to project project-name

tix switch -c new-project          # Create and switch to new project
                                   # Created and switched to new project new-project

tix projects                       # List all projects
                                   # * main
                                   #   project-name
                                   #   new-project
```

### Remote Operations

```bash
tix remote                         # List remotes
                                   # origin

tix remote -v                      # List with URLs
                                   # origin  git@github.com:user/repo.git

tix remote add git@github.com:user/repo.git
                                   # Added remote 'origin'

tix pull                           # Pull changes from remote
                                   # Pulling...
                                   # Pulled changes from remote

tix push                           # Push changes to remote (always uses -u)
                                   # Pushing...
                                   # Pushed changes to remote

tix push -f                        # Force push (short)
tix push --force                   # Force push
tix push --force-with-lease        # Safer force push

tix clone git@github.com:user/repo.git
                                   # Repository cloned successfully
```

### Priority Levels

- `a` - High priority
- `b` - Medium priority
- `c` - Low priority
- `z` - Default priority
- `none` - Use default

### Status Values

- `backlog` - Not started (default for new tickets)
- `todo` - Ready to work
- `doing` - In progress
- `done` - Completed

**Note:** File representation uses single letters: `s=b`, `s=t`, `s=w`, `s=d`

## License

MIT OR Apache-2.0
