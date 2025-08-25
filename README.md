# Tix CLI

Lightweight filesystem-based ticket management system.

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

## Usage

Initialize a new tix workspace:

```bash
tix init
```

More commands coming soon!

## License

MIT OR Apache-2.0