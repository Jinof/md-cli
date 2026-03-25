# md-cli Roadmap

## v0.1.0 (Current)
- [x] Parse markdown into node tree by heading structure
- [x] Show node content by path
- [x] Replace node content
- [x] Insert new node after existing
- [x] Delete node
- [x] Multi-platform CLI (macOS, Linux, Windows)
- [x] Install via cargo + crates.io
- [x] Install via npm
- [x] GitHub Actions CI/CD

---

## v0.2.0 - Enhanced Editing
- [ ] Edit by heading title (not just path): `md-cli edit "## Installation" "new content"`
- [ ] Move node to different position
- [ ] Rename heading (update path references)
- [ ] Batch edit multiple nodes

---

## v0.3.0 - Better Node Types
- [ ] Support nested lists (list items as sub-nodes)
- [ ] Support footnotes
- [ ] Support task lists ( `- [ ]` )
- [ ] Smart table editing (cell-level operations)
- [ ] YAML frontmatter support

---

## v0.4.0 - Developer Experience
- [ ] JSON output mode (`--json` flag for machine parsing)
- [ ] Interactive mode (`md-cli edit --interactive`)
- [ ] Dry-run mode (`--dry-run` to preview changes)
- [ ] Diff output (`md-cli diff` before applying changes)
- [ ] Watch mode (`--watch` for file changes)

---

## v0.5.0 - Integration
- [ ] Language Server Protocol (LSP) for VS Code / Neovim
- [ ] Git pre-commit hook integration
- [ ] Pre-commit CI for documentation consistency
- [ ] API bindings (Python, Node.js)

---

## Future Ideas
- [ ] AI-assisted editing (suggest edits based on content)
- [ ] Markdown linter/formatter
- [ ] Table of contents generator
- [ ] Cross-references between sections
- [ ] Collaborative editing support

---

## Bug Tracking
- [ ] Handle markdown with no headings (plain text files)
- [ ] Handle deeply nested headings (>6 levels)
- [ ] Handle files with CRLF line endings (Windows)
- [ ] Preserve original line endings in edited files
