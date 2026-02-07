# Heimdal Packages

> Community-maintained package metadata database for [Heimdal](https://github.com/limistah/heimdal)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Packages](https://img.shields.io/badge/packages-42-green.svg)](#packages)
[![Database Size](https://img.shields.io/badge/database-20KB-orange.svg)](#database)

Heimdal Packages is the centralized metadata repository for the Heimdal universal package manager. It contains structured YAML definitions for packages, groups, profiles, and cross-platform mappings that get compiled into a fast binary database format.

## Overview

Heimdal is a universal dotfile and system configuration manager that works across multiple package managers (apt, brew, dnf, pacman). This repository maintains the package metadata that Heimdal uses to:

- **Install packages** across different platforms with consistent naming
- **Suggest tools** based on project files and development patterns
- **Group packages** into curated collections for specific workflows
- **Manage dependencies** between tools and libraries
- **Provide profiles** for complete environment setups

### How It Works

1. **Package definitions** are written in YAML files (human-readable)
2. **Validation** ensures schema compliance and cross-reference integrity
3. **Compilation** transforms YAML into Bincode binary format (fast loading)
4. **Distribution** via GitHub Releases as `packages.db`
5. **Integration** with Heimdal client that downloads and caches the database

```
packages.yaml → validate → compile → packages.db → Heimdal CLI
```

## Repository Structure

```
heimdal-packages/
├── packages/            # Package definitions (42 packages)
│   ├── editors/        # Text editors and IDEs
│   ├── terminals/      # Terminal tools and multiplexers
│   ├── languages/      # Programming language runtimes
│   ├── containers/     # Docker, Kubernetes, container tools
│   ├── infrastructure/ # Terraform, Ansible, cloud tools
│   ├── databases/      # Database systems and clients
│   ├── git/           # Version control tools
│   ├── build/         # Build systems and compilers
│   ├── network/       # Network utilities
│   ├── shell/         # Shell enhancements
│   └── other/         # Miscellaneous tools
│
├── groups/             # Package group collections
├── profiles/           # Complete environment templates
├── mappings/           # Cross-platform package name mappings
├── dependencies/       # Package dependency definitions
├── suggestions/        # Smart suggestion patterns
├── templates/          # Profile templates
│
├── schemas/            # JSON schemas for validation
│   ├── package.schema.json
│   ├── group.schema.json
│   ├── profile.schema.json
│   ├── mapping.schema.json
│   ├── dependency.schema.json
│   ├── suggestion.schema.json
│   └── template.schema.json
│
└── scripts/            # Validation and compilation tools
    ├── validate.rs     # YAML validation script
    ├── compile.rs      # Binary database compiler
    └── stats.rs        # Statistics generator
```

## Packages

Current package count: **42 packages** across 11 categories

### Categories

| Category | Count | Examples |
|----------|-------|----------|
| **Terminals** | 8 | bat, fd, fzf, htop, jq, ripgrep, tmux, tree |
| **Build Tools** | 7 | cmake, delta, gh, lazygit, make, pipenv, yarn |
| **Languages** | 6 | go, node, npm, pip, python, rust |
| **Containers** | 5 | docker, docker-compose, helm, k9s, kubectl |
| **Editors** | 4 | emacs, helix, neovim, vim |
| **Shell** | 4 | bash, fish, starship, zsh |
| **Network** | 2 | curl, wget |
| **Infrastructure** | 2 | ansible, terraform |
| **Databases** | 2 | postgresql, redis |
| **Git** | 1 | git |
| **Other** | 1 | pandoc |

### Platform Support

All packages support multiple platforms:

- **apt** - Debian, Ubuntu (Linux)
- **brew** - macOS (Homebrew)
- **dnf** - Fedora, RHEL, CentOS (Linux)
- **pacman** - Arch Linux, Manjaro (Linux)
- **mas** - Mac App Store (macOS)

## Database

The compiled database is optimized for performance:

- **Format**: Bincode (binary serialization)
- **Current Size**: ~20KB (42 packages)
- **Target Size**: <30KB
- **Load Time**: <10ms average
- **Update Frequency**: Auto-sync every 7 days

### Binary Format Advantages

- **Fast loading**: Deserialization is ~100x faster than JSON parsing
- **Small size**: Binary format is more compact than JSON/YAML
- **Type safety**: Schema enforced at compile time
- **Cross-platform**: Works identically on all operating systems

## Development

### Prerequisites

- Rust 1.70+ (for validation and compilation)
- Git
- Text editor

### Setup

```bash
# Clone the repository
git clone https://github.com/limistah/heimdal-packages.git
cd heimdal-packages

# Install Rust dependencies
cargo build

# Validate all packages
cargo run --bin validate

# Compile database
cargo run --bin compile

# Generate statistics
cargo run --bin stats
```

### Scripts

#### Validate (`scripts/validate.rs`)

Validates all YAML files against JSON schemas and checks integrity:

```bash
cargo run --bin validate
```

**Checks performed:**
- YAML syntax validation
- JSON schema compliance
- Duplicate package names
- Cross-references (alternatives, dependencies, related packages)
- Platform coverage (at least 2 platforms per package)

#### Compile (`scripts/compile.rs`)

Compiles YAML files into binary database:

```bash
cargo run --bin compile
```

**Output:** `target/packages.db` (Bincode binary format)

#### Stats (`scripts/stats.rs`)

Generates repository statistics:

```bash
cargo run --bin stats
```

**Displays:**
- Total package count
- Packages by category
- Database size
- Platform coverage
- Validation status

### Adding a Package

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed instructions.

**Quick example:**

```yaml
# packages/editors/helix.yaml
name: helix
description: "Modern modal text editor"
category: editor
popularity: 75

platforms:
  apt: null
  brew: helix
  dnf: null
  pacman: helix

dependencies:
  required: []
  optional: []

alternatives:
  - vim
  - neovim

related:
  - ripgrep
  - fd

tags:
  - editor
  - rust
  - modern

website: https://helix-editor.com
license: MPL-2.0
source: https://github.com/helix-editor/helix
```

Then validate and compile:

```bash
cargo run --bin validate
cargo run --bin compile
```

## Integration with Heimdal

The compiled database is consumed by the main Heimdal CLI tool:

### Download & Cache

```bash
# Heimdal downloads packages.db from GitHub Releases
heimdal sync

# Database cached locally
~/.heimdal/cache/packages.db
```

### Usage

```bash
# Search for packages
heimdal packages search editor

# Get package info
heimdal packages info neovim

# Install packages
heimdal install neovim ripgrep fd

# Install package groups
heimdal install group:web-dev

# Apply profiles
heimdal profile apply developer
```

### Auto-Update

Heimdal automatically checks for database updates every 7 days and downloads the latest version from GitHub Releases.

## Package Schema

Each package YAML file follows this structure:

```yaml
name: string                    # Canonical package name
description: string             # Short description (1-2 sentences)
category: enum                  # Package category
popularity: number              # 0-100 popularity score

platforms:                      # Cross-platform package names
  apt: string | null           # Debian/Ubuntu
  brew: string | null          # macOS Homebrew
  dnf: string | null           # Fedora/RHEL
  pacman: string | null        # Arch Linux
  mas: number | null           # Mac App Store ID

dependencies:
  required: string[]           # Must be installed
  optional: string[]           # Recommended

alternatives: string[]         # Similar replaceable tools
related: string[]              # Complementary tools

tags: string[]                 # Search tags (lowercase, hyphens only)

website: string                # Official website (optional)
license: string                # Software license (optional)
source: string                 # Source repository (optional)
```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for:

- How to add packages, groups, and profiles
- Cross-platform mapping guidelines
- Validation requirements
- Style guide and naming conventions
- Testing procedures
- Commit message format

### Quick Contribution Workflow

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/heimdal-packages.git

# 2. Create branch
git checkout -b add-my-package

# 3. Add your package
vim packages/editors/my-editor.yaml

# 4. Validate
cargo run --bin validate

# 5. Compile
cargo run --bin compile

# 6. Commit and push
git add packages/editors/my-editor.yaml
git commit -m "Add my-editor package"
git push origin add-my-package

# 7. Open Pull Request on GitHub
```

## CI/CD Automation

Automated workflows ensure quality:

- **PR Validation**: All pull requests are automatically validated
- **Schema Checks**: JSON schema compliance enforced
- **Cross-Reference Checks**: Dependencies and alternatives verified
- **Compilation Tests**: Database compilation must succeed
- **Auto-Release**: Successful merges trigger GitHub Releases
- **Version Tagging**: Semantic versioning applied automatically

## Roadmap

### v1.0.0 (Released 2026-02-07)
- [x] Core package definitions (42 packages)
- [x] Validation scripts and comprehensive testing
- [x] Compilation to binary format (Bincode)
- [x] JSON schema definitions
- [x] Comprehensive documentation and examples
- [x] Package groups (7 groups)
- [x] Development profiles (5 profiles)
- [x] Cross-platform mappings (5 mapping files)
- [x] Dependency management (3 dependency files)
- [x] Smart suggestions (4 suggestion patterns)
- [x] Profile templates (2 templates)
- [x] CI/CD automation (GitHub Actions)
- [x] Full test suite (19 tests)
- [x] CI/CD workflows

### v1.1.0 (Future)
- [ ] Expand to 100+ packages
- [ ] Smart suggestion patterns
- [ ] Package dependency graphs
- [ ] Profile templates
- [ ] Advanced cross-platform mappings
- [ ] Package search optimization

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Projects

- **[Heimdal](https://github.com/limistah/heimdal)** - Universal dotfile and system configuration manager
- **[Homebrew](https://brew.sh/)** - macOS package manager
- **[Nix](https://nixos.org/)** - Declarative package manager
- **[Chezmoi](https://www.chezmoi.io/)** - Dotfile manager

## Credits

Maintained by [@limistah](https://github.com/limistah) and [contributors](https://github.com/limistah/heimdal-packages/graphs/contributors).

Package metadata sourced from official documentation, package repositories, and community contributions.

---

**Questions?** Open an issue or discussion on GitHub.

**Want to contribute?** See [CONTRIBUTING.md](CONTRIBUTING.md) to get started!
