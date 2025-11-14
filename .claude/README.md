# Claude Code Configuration

This directory contains configuration for Claude Code to streamline development on the Color Bang! project.

## Files

### `settings.json` (Project-wide)
Auto-approves common cargo commands for all developers:
- `cargo build`
- `cargo run`
- `cargo check`
- `cargo test`
- `cargo clippy`
- `cargo fmt`

This file is committed to the repository.

### `settings.local.json` (User-specific)
Personal settings that override project settings. This file is gitignored.
You can add your own custom approvals here.

### `commands/`
Custom slash commands available to all developers:
- `/build-release` - Build and run in release mode
- `/check-code` - Quick compilation check

Add new `.md` files here to create custom commands.

### `hooks/`
Hook scripts for advanced automation:
- `approve-cargo.sh` - Example hook for auto-approving cargo commands

## Usage

When using Claude Code in this project, cargo commands will be automatically approved without prompting. You can still use your personal `settings.local.json` for additional customizations.

## Adding New Commands

Create a new `.md` file in `commands/`:

```bash
echo "Your prompt here" > .claude/commands/my-command.md
```

Then use it with:
```
/my-command
```
