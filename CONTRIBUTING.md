# Contributing to Heimdal Packages

Thank you for your interest in contributing to the Heimdal Packages Database! This guide will help you add packages, fix mappings, and improve the database.

## 🎯 What Can You Contribute?

- **New packages** - Add package metadata for tools not yet in the database
- **Package mappings** - Fix or add cross-platform package names
- **Package groups** - Create curated collections for specific workflows
- **Profiles** - Complete environment templates
- **Dependencies** - Define package relationships
- **Suggestions** - File patterns that suggest package installations
- **Documentation** - Improve examples and guides

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (for validation and compilation tools)
- Git
- Text editor (VSCode, Vim, etc.)

### Setup

```bash
# 1. Fork the repository on GitHub

# 2. Clone your fork
git clone https://github.com/YOUR_USERNAME/heimdal-packages.git
cd heimdal-packages

# 3. Install dependencies
cargo build

# 4. Create a branch for your changes
git checkout -b add-my-package
```

## 📦 Adding a Package

### Step 1: Choose the Right Category

Packages are organized by category:
- `editors/` - Text editors and IDEs
- `terminals/` - Terminal emulators and multiplexers
- `languages/` - Programming languages and runtimes
- `containers/` - Docker, Kubernetes, and container tools
- `infrastructure/` - Terraform, Ansible, cloud tools
- `databases/` - Database systems and clients
- `git/` - Git and version control tools
- `build/` - Build systems and compilers
- `network/` - Network utilities
- `shell/` - Shell enhancements and utilities
- `other/` - Miscellaneous tools

### Step 2: Create the YAML File

Create a file named `packages/{category}/{package-name}.yaml`:

```yaml
# packages/editors/helix.yaml
name: helix
description: "A post-modern text editor"
category: editor
popularity: 75  # 0-100, based on usage/stars

# Cross-platform package names
platforms:
  apt: helix           # Debian/Ubuntu
  brew: helix          # macOS Homebrew
  dnf: helix           # Fedora/RHEL
  pacman: helix        # Arch Linux
  mas: null            # Mac App Store (use ID if available)

# Dependencies (optional)
dependencies:
  required: []         # Must be installed
  optional: []         # Recommended but not required

# Related packages
alternatives:          # Similar tools that can replace this
  - vim
  - neovim
  - emacs

related:               # Tools that work well with this
  - ripgrep
  - fd

# Search tags
tags:
  - editor
  - terminal
  - rust

# Metadata (optional but recommended)
website: https://helix-editor.com
license: MPL-2.0
source: https://github.com/helix-editor/helix
```

### Step 3: Validate Your Changes

```bash
# Validate YAML syntax and schema
cargo run --bin validate

# Should output:
# ✓ packages/editors/helix.yaml is valid
# ✓ All 41 packages validated successfully
```

### Step 4: Test Compilation

```bash
# Compile to binary database
cargo run --bin compile

# Should output:
# Compiling packages...
# ✓ Compiled 41 packages
# ✓ Generated packages.db (512 KB)
```

### Step 5: Submit Pull Request

```bash
git add packages/editors/helix.yaml
git commit -m "Add helix editor"
git push origin add-my-package

# Then create a PR on GitHub
```

## 🗺️ Adding Package Mappings

Package mappings handle cross-platform package name differences.

### Example: Adding a Language Mapping

Edit `mappings/languages.yaml`:

```yaml
# Add a new language
zig:
  canonical: zig       # Canonical name used in Heimdal
  platforms:
    apt: zig
    brew: zig
    dnf: zig
    pacman: zig
  aliases:             # Common name variations
    - ziglang
```

### Example: Adding a Complex Mapping

Some packages have very different names across platforms:

```yaml
# mappings/databases.yaml
postgresql:
  canonical: postgresql
  platforms:
    apt: postgresql-14        # Specific version on apt
    brew: postgresql@14       # Version suffix on brew
    dnf: postgresql-server    # Different name on dnf
    pacman: postgresql
  aliases:
    - postgres
    - psql
    - pg
```

## 👥 Adding Package Groups

Package groups are curated collections for specific workflows.

Create `groups/my-stack.yaml`:

```yaml
id: my-stack
name: "My Development Stack"
description: "Custom stack for my workflow"
category: development

packages:
  required:            # Must-have packages
    - rust
    - neovim
    - tmux
  
  optional:            # Nice-to-have packages
    - ripgrep
    - bat
    - fd

# Platform-specific additions
platform_overrides:
  macos:
    packages:
      - mas            # Mac App Store CLI
    casks:
      - iterm2
      - docker
  
  linux:
    packages:
      - build-essential
```

## 📝 Adding Profiles

Profiles are complete environment templates.

Create `profiles/my-profile.yaml`:

```yaml
id: my-profile
name: "My Complete Setup"
description: "Everything I need for development"
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
  
  languages:
    - rust
    - node
    - python

dotfiles:
  - source: .bashrc
    target: ~/.bashrc
  - source: .config/nvim
    target: ~/.config/nvim
  - source: .tmux.conf
    target: ~/.tmux.conf

hooks:
  post_install:
    - command: "nvim +PlugInstall +qall"
      description: "Install Neovim plugins"
```

## 🔍 Adding File Suggestions

Suggestion patterns help Heimdal recommend packages based on project files.

Edit `suggestions/languages.yaml`:

```yaml
patterns:
  # Add a new language detection pattern
  - files:
      - build.zig
      - build.zig.zon
    suggests:
      - package: zig
        priority: required
        reason: "Zig project detected"
      - package: zls
        priority: optional
        reason: "Zig Language Server"
```

## 🔗 Adding Dependencies

Define package relationships in `dependencies/{category}.yaml`:

```yaml
# dependencies/editors.yaml
helix:
  required: []         # Helix has no required dependencies
  optional:
    - package: ripgrep
      reason: "Fast file content search"
    - package: fd
      reason: "Fast file name search"
```

## ✅ Validation Checklist

Before submitting a PR, ensure:

- [ ] YAML syntax is valid
- [ ] Schema validation passes (`cargo run --bin validate`)
- [ ] Package name is unique
- [ ] At least 2 platform mappings provided
- [ ] Category is appropriate
- [ ] Description is clear and concise
- [ ] Tags are relevant
- [ ] Dependencies reference existing packages
- [ ] Compilation succeeds (`cargo run --bin compile`)
- [ ] No typos or formatting issues

## 🎨 Style Guide

### Naming Conventions

- **Package names**: lowercase, hyphens for multi-word (e.g., `docker-compose`)
- **File names**: match package name exactly (e.g., `docker-compose.yaml`)
- **Canonical names**: prefer official name from upstream project

### Descriptions

- Start with capital letter
- No period at the end
- Be concise but descriptive
- Focus on what the tool does, not how

**Good**:
```yaml
description: "Fast line-oriented search tool"
```

**Bad**:
```yaml
description: "this is a grep alternative written in rust."
```

### Popularity Scores

- **90-100**: Essential tools used by most developers (git, vim, curl)
- **75-89**: Very popular tools with large user base (neovim, tmux, docker)
- **50-74**: Popular within specific domains (kubectl, terraform, ansible)
- **25-49**: Niche tools or newer alternatives (helix, zellij)
- **0-24**: Experimental or very specialized tools

### Tags

- Use lowercase
- Separate words with hyphens
- Be specific but not redundant
- Include search terms users might use

**Good**:
```yaml
tags:
  - editor
  - vim
  - terminal
  - rust
```

**Bad**:
```yaml
tags:
  - Editor
  - text_editor
  - programming editor
  - best editor
```

## 🐛 Reporting Issues

Found a problem? Please open an issue with:

- **Package name** or affected file
- **Description** of the issue
- **Expected behavior**
- **Actual behavior**
- **Platform** (if relevant)

## 📊 Testing Your Changes

### Manual Testing

After compiling, you can test locally:

```bash
# Compile database
cargo run --bin compile

# Copy to Heimdal cache
cp target/packages.db ~/.heimdal/cache/packages.db

# Test with Heimdal
heimdal packages search helix
heimdal packages info helix
```

### Automated Testing

CI will automatically:
- Validate all YAML files
- Check for duplicate package names
- Verify cross-references
- Compile database
- Run schema validation

## 📜 Commit Message Guidelines

Use clear, descriptive commit messages:

```bash
# Good
git commit -m "Add helix editor to packages"
git commit -m "Fix kubectl mapping for Fedora"
git commit -m "Add web-dev package group"

# Bad
git commit -m "update"
git commit -m "fix stuff"
git commit -m "WIP"
```

Format:
```
<type>: <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New package, group, or profile
- `fix`: Bug fix or correction
- `docs`: Documentation updates
- `chore`: Maintenance tasks
- `refactor`: Restructuring without behavior change

## 🎉 Recognition

Contributors will be:
- Listed in the release notes
- Mentioned in the project README
- Given credit in the compiled database metadata

## 💬 Getting Help

- **Questions**: Open a discussion on GitHub
- **Issues**: Report bugs via GitHub issues
- **Chat**: Join the Heimdal Discord (link in main repo)

## 📚 Resources

- [YAML Syntax Guide](https://yaml.org/spec/1.2.2/)
- [JSON Schema Documentation](https://json-schema.org/)
- [Heimdal Documentation](https://github.com/limistah/heimdal#readme)

---

Thank you for contributing! Every package added makes Heimdal better for everyone. 🚀
