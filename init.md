# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust-based personal CLI/TUI utility collection focused on enhancing developer workflows, particularly git operations and project management.

## Build Commands

```bash
make build    # Clean build with release optimization
make test     # Run tests
make fix      # Format, fix, lint, and test (uses nightly)
```

Single test: `cargo test <test_name>`

## Requirements

- Rust nightly toolchain (pinned to `nightly-2024-12-07` in rust-toolchain.toml)
- Install with: `rustup install nightly`

## Architecture

### Library Modules (`src/lib.rs`)

- **fuzzy**: Core fuzzy selection TUI framework with builder pattern
  - Views: `SimpleView`, `PanesView` (with preview), `TabView` (multi-tab)
  - Supports list filtering, multi-select, keyboard navigation, custom previews
- **git**: Git operations (branch management, memoization, config, auth)
- **io**: Command execution, file I/O, stdin/stdout utilities

### Binaries (`src/bin/`)

Organized by interface type and domain:

**CLI Git** (`cli/git/`): `git-checkout`, `git-log`, `git-pull`, `git-push`, `git-rebase`, `git-stash`, `git-pull-request`, `git-pull-request-commit`

**TUI Git** (`tui/git/`): `git-hub-launcher`, `git-hub-pull-request-review-launcher`

**TUI Project** (`tui/project/`): `build-tool-launcher`, `development-starter`, `project-directory`, `project-task`

**TUI Misc** (`tui/misc/`): `filter`

**CLI Misc** (`cli/misc/`): `finder`

### Shell Integration

- `wrapper-alias/`: Zsh aliases pointing to `target/release/` binaries
- `wrapper-bin/`: Shell script wrappers for complex integrations (e.g., `bb` outputs commands to be `eval`'d)

## Key Dependencies

- **tui** + **crossterm**: Terminal UI rendering
- **structopt**: CLI argument parsing
- **tokio**: Async runtime
- **rayon**: Parallel processing
