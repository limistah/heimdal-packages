# Heimdal Packages Database - Architecture

This document describes the architecture, design decisions, and implementation details of the Heimdal Packages Database.

## Overview

The Heimdal Packages Database is a **community-maintained** collection of package metadata that is:
- Stored as **human-readable YAML** files in this repository
- **Compiled** into a binary database (`packages.db`) using Bincode
- **Distributed** via GitHub Releases
- **Downloaded** and cached by Heimdal on first use
- **Auto-updated** during `heimdal sync` (if older than 7 days)

## Design Principles

### 1. **Community-First**
- Easy to contribute (no Rust knowledge required)
- Clear contribution guidelines
- Automated validation and testing
- Fast feedback loop (CI/CD)

### 2. **Performance**
- Binary format for fast loading (<10ms)
- Indexed for O(1) lookups
- Minimal memory footprint
- Compressed for efficient distribution

### 3. **Offline-First**
- Local cache for offline use
- One-time download on first use
- Graceful degradation if update fails
- No embedded fallback (forces explicit download)

### 4. **Separation of Concerns**
- Data lives in this repo
- Code lives in main Heimdal repo
- Clear boundaries between data and logic
- Version control for data separate from code

## Architecture Decisions

### Why YAML instead of JSON/TOML?

**YAML** was chosen because:
- ✅ Most human-readable format
- ✅ Comments supported (important for documentation)
- ✅ Hierarchical structure maps well to data model
- ✅ GitHub UI can edit YAML files directly
- ✅ Multi-line strings without escaping
- ❌ Slower to parse (mitigated by compilation step)

**Alternatives considered**:
- JSON: More verbose, no comments, harder for humans
- TOML: Better than JSON, but less familiar to most contributors

### Why Bincode instead of MessagePack/JSON?

**Bincode** was chosen because:
- ✅ Fastest serialization in Rust ecosystem
- ✅ Smallest binary size (~50% smaller than JSON)
- ✅ Zero-copy deserialization
- ✅ Type-safe (compile-time checks)
- ❌ Rust-only (not a concern since Heimdal is Rust)

**Comparison** (approximate):
| Format | Size | Deserialize Time | Cross-Language |
|--------|------|------------------|----------------|
| JSON | 1.0x | 100ms | Yes |
| JSON (gzip) | 0.7x | 80ms | Yes |
| MessagePack | 0.6x | 40ms | Yes |
| Bincode | 0.5x | 10ms | No |

### Why No Embedded Fallback?

**No embedded fallback** was chosen to:
- ✅ Minimize binary size (save 300-400 KB)
- ✅ Force explicit download (better UX than stale data)
- ✅ Ensure users always have latest data
- ✅ Simplify update logic

**Trade-offs**:
- ❌ Requires internet on first use
- ❌ One extra command for users: `heimdal packages update`
- ✅ Mitigated by auto-download during wizard setup

### Why Auto-Update During Sync?

**Auto-update** during sync was chosen because:
- ✅ Seamless user experience (no manual updates)
- ✅ Ensures fresh data (package landscape changes frequently)
- ✅ Sync already requires network connection
- ✅ Updates are small (~500 KB)

**Update frequency**: 7 days
- Not too frequent (avoid unnecessary downloads)
- Not too infrequent (keep data fresh)
- Configurable in Heimdal settings

## Data Model

### Core Entities

```
Package
├── name: String
├── description: String
├── category: Category
├── popularity: u8 (0-100)
├── platforms: PlatformMap
├── dependencies: Dependencies
├── alternatives: Vec<String>
├── related: Vec<String>
├── tags: Vec<String>
└── metadata: Metadata

Mapping
├── canonical: String
├── platforms: PlatformMap
└── aliases: Vec<String>

Dependency
├── package: String
├── required: bool
└── reason: String

PackageGroup
├── id: String
├── name: String
├── description: String
├── category: String
├── packages: GroupPackages
└── platform_overrides: PlatformOverrides

Profile
├── id: String
├── name: String
├── description: String
├── type: ProfileType
├── packages: ProfilePackages
├── dotfiles: Vec<DotfileMapping>
└── hooks: Hooks

SuggestionPattern
├── files: Vec<String>
└── suggests: Vec<Suggestion>
```

### Relationships

```
Package ──┬─> alternatives: [Package]
          ├─> related: [Package]
          └─> dependencies: [Dependency]
                               │
                               └─> package: Package

PackageGroup ──> packages: [Package]

Profile ──> packages: [Package]

SuggestionPattern ──> suggests: [Suggestion]
                                    │
                                    └─> package: Package

Mapping ──> canonical: Package
```

### Indexing Strategy

The compiled database includes indexes for fast lookups:

```rust
pub struct PackageDatabase {
    // Raw data
    pub packages: Vec<PackageInfo>,
    pub mappings: HashMap<String, PackageMapping>,
    pub dependencies: HashMap<String, Vec<Dependency>>,
    pub groups: Vec<PackageGroup>,
    pub profiles: Vec<Profile>,
    pub suggestions: Vec<SuggestionPattern>,
    
    // Indexes (built during compilation)
    pub index_by_name: HashMap<String, usize>,           // O(1) name lookup
    pub index_by_category: HashMap<Category, Vec<usize>>, // O(1) category filter
    pub index_by_tag: HashMap<String, Vec<usize>>,        // O(1) tag search
    
    // Metadata
    pub version: u32,
    pub last_updated: DateTime<Utc>,
}
```

**Lookup complexity**:
- By name: O(1) via `index_by_name`
- By category: O(1) via `index_by_category`
- By tag: O(1) via `index_by_tag`
- Fuzzy search: O(n) but with early termination

## Build Pipeline

### Compilation Process

```
┌─────────────────────────────────────────────────────────────┐
│ 1. YAML Files (Source of Truth)                             │
│    packages/**/*.yaml                                        │
│    mappings/**/*.yaml                                        │
│    dependencies/**/*.yaml                                    │
│    groups/*.yaml                                             │
│    profiles/*.yaml                                           │
│    suggestions/*.yaml                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Validation (CI on PR)                                    │
│    - Schema validation (JSON Schema)                        │
│    - Duplicate detection                                    │
│    - Cross-reference validation                             │
│    - Syntax checking                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Compilation (CI on merge)                                │
│    - Parse all YAML files                                   │
│    - Build data structures                                  │
│    - Create indexes                                         │
│    - Serialize to Bincode                                   │
│    - Generate checksum (SHA-256)                            │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Release (GitHub)                                         │
│    - packages.db (binary database)                          │
│    - packages.db.sha256 (checksum)                          │
│    - release-notes.md (changelog)                           │
│    - stats.json (database statistics)                       │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│ 5. Distribution (Heimdal downloads)                         │
│    - Download packages.db                                   │
│    - Verify checksum                                        │
│    - Cache to ~/.heimdal/cache/packages.db                  │
│    - Deserialize with Bincode                               │
└─────────────────────────────────────────────────────────────┘
```

### CI/CD Workflows

**On Pull Request** (`.github/workflows/validate.yml`):
```yaml
name: Validate
on: [pull_request]
jobs:
  validate:
    - Checkout code
    - Setup Rust
    - Run: cargo run --bin validate
    - Run: cargo run --bin compile
    - Run: cargo test
    - Comment on PR with validation results
```

**On Merge to Main** (`.github/workflows/release.yml`):
```yaml
name: Release
on:
  push:
    branches: [main]
jobs:
  build-and-release:
    - Checkout code
    - Setup Rust
    - Compile database
    - Generate checksum
    - Generate changelog
    - Generate statistics
    - Create GitHub Release
    - Upload artifacts:
        - packages.db
        - packages.db.sha256
        - CHANGELOG.md
        - stats.json
```

## Versioning Strategy

### Semantic Versioning

Database releases follow SemVer:
- **Major** (v2.0.0): Breaking schema changes, incompatible with older Heimdal
- **Minor** (v1.1.0): New packages, groups, or features (backward compatible)
- **Patch** (v1.0.1): Bug fixes, data corrections (backward compatible)

### Compatibility Matrix

| Database Version | Heimdal Version | Status |
|------------------|-----------------|--------|
| v1.x.x | v1.0.0+ | ✅ Compatible |
| v2.x.x | v2.0.0+ | ✅ Compatible |
| v2.x.x | v1.x.x | ❌ Incompatible |

Heimdal checks database version on load and warns if incompatible.

## Validation Rules

### Package Validation

- ✅ Name must be unique across all packages
- ✅ Name must match filename (e.g., `neovim.yaml` → `name: neovim`)
- ✅ Description required and non-empty
- ✅ Category must be valid enum value
- ✅ Popularity must be 0-100
- ✅ At least 2 platform mappings required
- ✅ Dependencies must reference existing packages
- ✅ Alternatives must reference existing packages
- ✅ Related must reference existing packages
- ✅ Tags must be lowercase, hyphenated

### Mapping Validation

- ✅ Canonical name must reference existing package
- ✅ At least 2 platform mappings required
- ✅ Aliases must not conflict with other package names

### Group Validation

- ✅ ID must be unique across all groups
- ✅ All packages must exist in database
- ✅ No circular dependencies

### Profile Validation

- ✅ ID must be unique across all profiles
- ✅ All packages must exist in database
- ✅ Dotfile sources must be valid paths

## Performance Characteristics

### Memory Usage

**Database size**: ~500 KB in memory
- 100+ packages × ~3 KB each = ~300 KB
- Indexes: ~150 KB
- Metadata: ~50 KB

**Total heap allocation**: ~500 KB (negligible)

### Load Time

**Cold load** (from disk):
- Read file: ~2ms
- Deserialize: ~5ms
- Build indexes: ~3ms
- **Total**: ~10ms

**Hot load** (cached):
- Already in memory: 0ms

### Search Performance

- **Name lookup**: O(1) via HashMap - <1µs
- **Category filter**: O(1) via index - <10µs
- **Tag search**: O(1) via index - <10µs
- **Fuzzy search**: O(n) with early termination - <100µs

## Security Considerations

### Download Integrity

1. **HTTPS only** - All downloads over encrypted connection
2. **Checksum verification** - SHA-256 hash checked before use
3. **Signature** (future) - GPG signature verification

### Malicious Data

- **Schema validation** - Malformed data rejected
- **Size limits** - Prevent DoS via huge files
- **Rate limiting** - GitHub CDN handles this

### User Trust

- **Open source** - All data visible in GitHub
- **Community review** - PRs reviewed before merge
- **Audit trail** - Git history shows all changes
- **Reproducible builds** - CI process is public

## Extensibility

### Adding New Entity Types

To add a new entity type (e.g., "Snippet"):

1. Define schema in `schemas/snippet.schema.json`
2. Add YAML files in `snippets/*.yaml`
3. Update compiler in `scripts/compile.rs`
4. Update database struct in Heimdal
5. Bump minor version

### Custom User Data

Users can extend the database locally:

```
~/.config/heimdal/
└── overrides/
    ├── packages/
    │   └── my-tool.yaml       # Custom package
    ├── groups/
    │   └── my-stack.yaml      # Custom group
    └── mappings/
        └── custom.yaml        # Custom mappings
```

Heimdal merges user overrides with main database on load.

## Migration Strategy

### From Embedded to External

**Phase 1**: Keep embedded database, add external loader
**Phase 2**: Make external database required, remove embedded
**Phase 3**: (current) No embedded fallback

### Backward Compatibility

Heimdal maintains compatibility:
- Old Heimdal versions work with old database format
- New Heimdal versions check database version
- Clear error messages if incompatible

## Future Enhancements

### Planned Features

- [ ] **Incremental updates** - Download only changed files
- [ ] **Compression** - Gzip database for smaller downloads
- [ ] **CDN distribution** - Faster downloads worldwide
- [ ] **Package ratings** - Community voting on quality
- [ ] **Usage statistics** - Track popular packages
- [ ] **Localization** - Multi-language descriptions
- [ ] **Versioning** - Support multiple package versions
- [ ] **Platform detection** - Auto-suggest based on OS

### Possible Extensions

- **Package templates** - Code scaffolding templates
- **Config snippets** - Example configurations
- **Dotfile templates** - Pre-configured dotfiles
- **Script library** - Common automation scripts

## Monitoring & Metrics

### Database Metrics

Tracked in `stats.json`:
```json
{
  "version": "1.0.0",
  "packages": 102,
  "mappings": 85,
  "dependencies": 156,
  "groups": 15,
  "profiles": 10,
  "suggestions": 18,
  "size_bytes": 524288,
  "last_updated": "2026-02-07T00:00:00Z"
}
```

### Download Metrics

GitHub provides:
- Download counts per release
- Geographic distribution
- Referrer tracking

## References

- [Bincode Format](https://github.com/bincode-org/bincode)
- [YAML Specification](https://yaml.org/spec/1.2.2/)
- [JSON Schema](https://json-schema.org/)
- [Semantic Versioning](https://semver.org/)

---

**Last Updated**: February 7, 2026
