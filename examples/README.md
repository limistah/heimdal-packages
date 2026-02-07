# Examples Directory

This directory contains example YAML files demonstrating the structure and usage of different entity types in the Heimdal Packages database.

## Files

### Package Examples

#### `minimal-package.yaml`
The absolute minimum required to define a package. Use this as a starting point when adding new packages.

**Key features:**
- Minimal required fields only
- Simple platform mappings
- Basic tags

#### `full-package.yaml`
A comprehensive example showing all available fields and options for package definitions.

**Key features:**
- All optional fields demonstrated
- Detailed dependencies (required and optional)
- Platform-specific package names
- Complete metadata (website, license, source)
- Alternatives and related packages
- Comprehensive tagging

### Group Example

#### `package-group.yaml`
Shows how to create curated package collections for specific workflows.

**Key features:**
- Required and optional packages
- Platform-specific overrides
- macOS cask support
- Linux-specific packages

### Profile Example

#### `profile.yaml`
Demonstrates complete development environment setup profiles.

**Key features:**
- Organized package categories
- Dotfile mappings
- Pre and post-installation hooks
- Profile type classification

## Using These Examples

### Adding a New Package

1. Copy `minimal-package.yaml` as a starting point
2. Reference `full-package.yaml` for advanced features
3. Place in appropriate category directory: `packages/{category}/{name}.yaml`
4. Validate: `cargo run --bin validate`
5. Test compilation: `cargo run --bin compile`

### Creating a Package Group

1. Copy `package-group.yaml`
2. Customize for your workflow
3. Place in `groups/{id}.yaml`
4. Validate and test

### Creating a Profile

1. Copy `profile.yaml`
2. Define package categories
3. Add dotfiles and hooks as needed
4. Place in `profiles/{id}.yaml`
5. Validate and test

## Field Reference

### Package Categories

Valid values for `category`:
- `essential` - Core system utilities
- `editor` - Text editors and IDEs
- `terminal` - Terminal tools and utilities
- `language` - Programming languages and runtimes
- `container` - Docker, Kubernetes, container tools
- `infrastructure` - Terraform, Ansible, cloud tools
- `database` - Database systems and clients
- `network` - Network utilities
- `application` - General applications
- `shell` - Shell enhancements
- `git` - Version control tools
- `build` - Build systems and compilers
- `other` - Miscellaneous tools

### Group Categories

Valid values for group `category`:
- `development` - Development environment stacks
- `productivity` - Productivity tools and workflows
- `system` - System administration and operations
- `security` - Security and privacy tools
- `media` - Media creation and editing

### Profile Types

Valid values for profile `type`:
- `minimal` - Bare essentials only
- `developer` - Software development
- `devops` - Infrastructure and operations
- `designer` - Design and creative work
- `data-scientist` - Data analysis and ML
- `custom` - Custom configurations

## Validation Rules

### Package Names
- Must be lowercase
- Can contain letters, numbers, hyphens
- Must match filename (e.g., `git.yaml` → `name: git`)

### Tags
- Must be lowercase
- Can contain letters, numbers, hyphens only
- No spaces, underscores, or special characters
- Pattern: `^[a-z0-9-]+$`

### Popularity Scores
- Range: 0-100
- 90-100: Essential tools (git, vim, curl)
- 75-89: Very popular (neovim, tmux, docker)
- 50-74: Popular within domains (kubectl, terraform)
- 25-49: Niche or newer tools
- 0-24: Experimental or specialized

### Platform Mappings
- Must provide at least 2 platforms
- Use `null` if unavailable on a platform
- Common differences to watch for:
  - `docker` → `docker.io` on apt
  - `fd` → `fd-find` on apt
  - `node` → `nodejs` on apt/dnf
  - `python` → `python3` on apt/dnf

## Common Patterns

### Cross-Platform Tools
Most tools have consistent names:
```yaml
platforms:
  apt: tool-name
  brew: tool-name
  dnf: tool-name
  pacman: tool-name
```

### Platform-Specific Names
Some tools vary by platform:
```yaml
platforms:
  apt: nodejs      # Different on apt
  brew: node       # Standard name
  dnf: nodejs      # Different on dnf
  pacman: nodejs   # Different on pacman
```

### Language Runtimes
Often need platform-specific handling:
```yaml
platforms:
  apt: python3
  brew: python     # No version suffix
  dnf: python3
  pacman: python
```

### Package Managers
May be bundled or separate:
```yaml
# pip - separate on Linux, bundled on macOS
platforms:
  apt: python3-pip
  brew: null       # Comes with python
  dnf: python3-pip
  pacman: python-pip
```

## Testing Your Changes

Always run these commands after making changes:

```bash
# Validate YAML syntax and schema
cargo run --bin validate

# Compile to binary database
cargo run --bin compile

# Generate statistics
cargo run --bin stats
```

## Need Help?

- See [CONTRIBUTING.md](../CONTRIBUTING.md) for detailed guidelines
- Check existing packages in `packages/` for real-world examples
- Open an issue if you have questions

## Schema Files

Full JSON schemas are available in `schemas/`:
- `package.schema.json` - Package definitions
- `group.schema.json` - Package groups
- `profile.schema.json` - Development profiles
- `mapping.schema.json` - Cross-platform mappings
- `dependency.schema.json` - Dependency relationships
- `suggestion.schema.json` - File-based suggestions
- `template.schema.json` - Profile templates
