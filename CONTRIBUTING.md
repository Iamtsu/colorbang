# Contributing to Color Bang!

Thank you for your interest in contributing to Color Bang! This document provides guidelines for contributing to the project.

## Branch Naming Convention

All pull requests must follow semantic release branch naming conventions:

### Accepted Branch Prefixes:

- `feature/` - For new features or enhancements
- `bugfix/` - For bug fixes

### Examples:

✅ **Valid branch names:**
- `feature/add-boss-enemy`
- `feature/power-ups`
- `bugfix/collision-detection`
- `bugfix/sound-loading-error`

❌ **Invalid branch names:**
- `main`
- `my-changes`
- `patch-1`
- `fix-bug`

## How to Contribute

### 1. Fork the Repository

Fork the repository to your own GitHub account.

### 2. Create a Feature or Bugfix Branch

Clone your fork and create a branch following the naming convention:

```bash
# For a new feature
git checkout -b feature/your-feature-name

# For a bug fix
git checkout -b bugfix/your-bug-fix
```

### 3. Make Your Changes

- Write clean, readable code
- Follow the existing code style
- Test your changes thoroughly
- Ensure the game builds and runs: `cargo run`

### 4. Commit Your Changes

Write clear commit messages describing what you changed and why:

```bash
git add .
git commit -m "Add new enemy type with special behavior"
```

### 5. Push to Your Fork

```bash
git push origin feature/your-feature-name
```

### 6. Create a Pull Request

1. Go to the original repository on GitHub
2. Click "New Pull Request"
3. Select your fork and branch
4. Provide a clear description of your changes
5. Submit the PR

**Note:** PRs from branches that don't follow the naming convention will be automatically rejected by our CI checks.

## Code Guidelines

- **Rust Style**: Follow standard Rust conventions
- **Performance**: Be mindful of performance in the game loop
- **Safety**: Avoid unnecessary `unsafe` code unless required for performance
- **Comments**: Add comments for complex logic

## Questions?

If you have questions about contributing, feel free to open an issue for discussion before starting work.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
