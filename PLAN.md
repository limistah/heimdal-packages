# Heimdal Packages Database

**Community-maintained package metadata for [Heimdal](https://github.com/limistah/heimdal)**

This repository contains package definitions, mappings, groups, profiles, and dependencies in YAML format. The data is compiled into a binary database (`packages.db`) that Heimdal downloads and caches locally.

## 📦 What's Inside

- **Packages** - Metadata for 40+ popular development tools
- **Mappings** - Cross-platform package name translations (apt/brew/dnf/pacman)
- **Dependencies** - Package dependency relationships
- **Groups** - Curated package collections (web-dev, rust-dev, etc.)
- **Profiles** - Complete development environment templates
- **Suggestions** - File pattern → package recommendations
- **Templates** - Profile configuration templates

## 🚀 Quick Start

### For Users

Heimdal automatically downloads and updates this database. You don't need to interact with this repo directly unless you want to contribute.

```bash
# Update package database manually
heimdal packages update

# Search packages
heimdal packages search neovim

# Install a package group
heimdal packages add-group web-dev
```

### For Contributors

Want to add a package, fix a mapping, or create a new group? You're in the right place!

```bash
# 1. Fork and clone
git clone https://github.com/limistah/heimdal-packages.git
cd heimdal-packages

# 2. Add your package
cat > packages/editors/helix.yaml << EOF
name: helix
description: "A post-modern text editor"
category: editor
popularity: 75

platforms:
  apt: helix
  brew: helix
  dnf: helix
  pacman: helix

tags:
  - editor
  - terminal
  - rust

website: https://helix-editor.com
EOF

# 3. Validate
cargo run --bin validate

# 4. Test compilation
cargo run --bin compile

# 5. Submit PR
git add packages/editors/helix.yaml
git commit -m "Add helix editor"
git push origin main
```

## 📂 Repository Structure

```
heimdal-packages/
├── packages/              # Individual package metadata (YAML)
│   ├── editors/
│   │   ├── neovim.yaml
│   │   ├── vim.yaml
│   │   └── emacs.yaml
│   ├── terminals/
│   │   ├── tmux.yaml
│   │   ├── zsh.yaml
│   │   └── fzf.yaml
│   ├── languages/
│   │   ├── node.yaml
│   │   ├── python.yaml
│   │   ├── rust.yaml
│   │   └── go.yaml
│   ├── containers/
│   │   ├── docker.yaml
│   │   ├── kubectl.yaml
│   │   └── helm.yaml
│   └── ...
│
├── mappings/              # Cross-platform package name mappings
│   ├── core.yaml          # Essential tools (git, vim, curl)
│   ├── editors.yaml       # Text editors
│   ├── terminals.yaml     # Terminal utilities
│   ├── languages.yaml     # Programming languages
│   ├── containers.yaml    # Docker/K8s tools
│   └── aliases.yaml       # Name normalization (nodejs→node)
│
├── dependencies/          # Package dependency relationships
│   ├── editors.yaml       # neovim → git, ripgrep, fzf
│   ├── languages.yaml     # node → yarn, npm
│   ├── containers.yaml    # docker → docker-compose
│   └── infrastructure.yaml # terraform → tflint
│
├── groups/                # Curated package collections
│   ├── web-dev.yaml
│   ├── rust-dev.yaml
│   ├── python-dev.yaml
│   ├── devops.yaml
│   └── ...
│
├── profiles/              # Complete development profiles
│   ├── minimal.yaml
│   ├── developer.yaml
│   ├── devops.yaml
│   └── ...
│
├── suggestions/           # File pattern → package suggestions
│   ├── languages.yaml     # package.json → node
│   ├── containers.yaml    # Dockerfile → docker
│   └── editors.yaml       # .vimrc → neovim
│
├── templates/             # Profile configuration templates
│   ├── minimal.yaml
│   ├── developer.yaml
│   ├── macos-desktop.yaml
│   └── linux-server.yaml
│
├── detection/             # Package detection/categorization rules
│   ├── categories.yaml    # essential, development, terminal, etc.
│   └── filters.yaml       # System packages to ignore
│
├── schemas/               # JSON schemas for validation
│   ├── package.schema.json
│   ├── mapping.schema.json
│   ├── group.schema.json
│   └── profile.schema.json
│
├── scripts/               # Build and validation tools
│   ├── compile.rs         # YAML → Binary database compiler
│   ├── validate.rs        # Schema validation
│   └── stats.rs           # Generate statistics
│
└── .github/
    └── workflows/
        ├── validate.yml   # Validate YAMLs on PR
        └── release.yml    # Build binary database on merge
```

## 📝 YAML Schemas

### Package Definition

```yaml
# packages/editors/neovim.yaml
name: neovim
description: "Hyperextensible Vim-based text editor"
category: editor
popularity: 90

# Cross-platform package names
platforms:
  apt: neovim
  brew: neovim
  dnf: neovim
  pacman: neovim
  mas: null  # Not available on Mac App Store

# Package relationships
dependencies:
  required:
    - package: git
      reason: "Required for plugin management"
  optional:
    - package: ripgrep
      reason: "Fast file content search"
    - package: fzf
      reason: "Fuzzy file finder integration"

alternatives:
  - vim
  - emacs

related:
  - ripgrep
  - fzf
  - fd

tags:
  - editor
  - vim
  - terminal
  - programming

# Additional metadata
website: https://neovim.io
license: Apache-2.0
source: https://github.com/neovim/neovim
```

### Package Mapping

```yaml
# mappings/languages.yaml
node:
  canonical: node
  platforms:
    apt: nodejs
    brew: node
    dnf: nodejs
    pacman: nodejs
  aliases:
    - nodejs
    - node.js
    - node-js

python:
  canonical: python
  platforms:
    apt: python3
    brew: python
    dnf: python3
    pacman: python
  aliases:
    - py
    - py3
    - python3
```

### Package Group

```yaml
# groups/web-dev.yaml
id: web-dev
name: "Web Development"
description: "Complete web development stack"
category: development

packages:
  required:
    - node
    - git
    - docker
  optional:
    - yarn
    - typescript
    - prettier

platform_overrides:
  macos:
    casks:
      - visual-studio-code
      - google-chrome
```

### Profile

```yaml
# profiles/developer.yaml
id: developer
name: "Developer"
description: "Complete development environment"
type: developer

packages:
  essential:
    - git
    - neovim
    - tmux
  terminal:
    - fzf
    - ripgrep
    - bat

dotfiles:
  - source: .bashrc
    target: ~/.bashrc
  - source: .config/nvim
    target: ~/.config/nvim
```

### Suggestion Pattern

```yaml
# suggestions/languages.yaml
patterns:
  - files:
      - package.json
      - yarn.lock
    suggests:
      - package: node
        priority: required
        reason: "Node.js project detected"
      - package: yarn
        priority: optional
        reason: "yarn.lock found"
```

## 🔨 Build Process

### Local Development

```bash
# Install dependencies
cargo build

# Validate all YAML files
cargo run --bin validate

# Compile to binary database
cargo run --bin compile

# Run tests
cargo test

# Generate statistics
cargo run --bin stats
```

### CI/CD Pipeline

1. **On PR**: Validate YAML schemas, check for duplicates, run tests
2. **On merge to main**: Compile `packages.db`, create GitHub release
3. **Versioning**: Semantic versioning (v1.0.0, v1.1.0, etc.)

## 📊 Database Statistics

Current database contains:
- **40+ packages** with full metadata
- **80+ package mappings** across 4 platforms
- **50+ dependency relationships**
- **15 curated package groups**
- **10 development profiles**
- **15+ file detection patterns**

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### Adding a Package

1. Create a new YAML file in the appropriate category folder
2. Fill in all required fields (name, description, category, platforms)
3. Add tags, dependencies, and related packages
4. Run validation: `cargo run --bin validate`
5. Submit a PR

### Adding a Package Group

1. Create a YAML file in `groups/`
2. List required and optional packages
3. Add platform-specific overrides if needed
4. Submit a PR

### Updating Mappings

1. Edit the appropriate file in `mappings/`
2. Add platform-specific package names
3. Include common aliases
4. Submit a PR

### Creating a Profile

1. Create a YAML file in `profiles/`
2. Define package lists by category
3. Add dotfile mappings and hooks
4. Submit a PR

## 📋 Validation Rules

All YAMLs must pass these checks:

- **Schema compliance** - Match JSON schema definitions
- **No duplicates** - Package names must be unique
- **Valid references** - Dependencies and related packages must exist
- **Platform coverage** - At least 2 platforms per package
- **Required fields** - name, description, category, platforms

## 🔄 Update Frequency

- **Automated updates**: Heimdal checks for updates during `heimdal sync`
- **Update interval**: Every 7 days
- **Manual update**: `heimdal packages update`
- **Cache location**: `~/.heimdal/cache/packages.db`

## 📜 License

MIT License - See LICENSE file for details

## 🔗 Links

- **Heimdal**: https://github.com/limistah/heimdal
- **Issues**: https://github.com/limistah/heimdal-packages/issues
- **Discussions**: https://github.com/limistah/heimdal-packages/discussions

## 🙏 Acknowledgments

Built with ❤️ by the Heimdal community

---

**Note**: This repository is automatically consumed by Heimdal. Users don't need to clone or interact with it directly unless contributing.
