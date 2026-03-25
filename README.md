# md-cli

Markdown Node Editor - Parse and edit markdown files by heading structure.

## Features

- **Parse**: Show markdown as a tree of nodes indexed by heading path
- **Show**: View content of any node
- **Replace**: Replace node content while preserving structure
- **Insert**: Insert new nodes after existing ones
- **Delete**: Remove nodes

## Installation

### Cargo (from source)

```bash
cargo install md-cli
```

### From Git

```bash
cargo install --git https://github.com/Jinof/md-cli.git
```

### Pre-built binaries

Download from the [Releases](https://github.com/Jinof/md-cli/releases) page.

| Platform | Download |
|----------|----------|
| macOS (x86_64) | `md-cli-x86_64-apple-darwin.tar.gz` |
| macOS (ARM64) | `md-cli-aarch64-apple-darwin.tar.gz` |
| Linux (x86_64) | `md-cli-x86_64-unknown-linux-musl.tar.gz` |
| Windows (x86_64) | `md-cli-x86_64-pc-windows-msvc.zip` |

### Build from source

```bash
git clone https://github.com/Jinof/md-cli.git
cd md-cli
cargo build --release
./target/release/md-cli --help
```

## Usage

```bash
# Parse and show node tree
md-cli parse README.md

# Show specific node
md-cli show README.md 1.2.3

# Replace node content
md-cli replace README.md 1.2.3 "New content here"

# Insert new node after existing
md-cli insert README.md 1.2 "## New Section"

# Delete a node
md-cli delete README.md 1.2.3
```

## Node Path

Nodes are referenced by path based on heading structure:

```
0           # Top-level heading (first H1)
0.1         # First H2 under first H1
0.1.2       # Third child node under 0.1
0.1.0       # Non-heading content (paragraph, code block) under 0.1
```

## License

MIT
