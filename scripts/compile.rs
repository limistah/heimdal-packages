//! Compile YAML package database to binary format
//!
//! This script:
//! 1. Loads all YAML files from packages/, mappings/, etc.
//! 2. Validates cross-references
//! 3. Builds indexes
//! 4. Serializes to Bincode format
//! 5. Generates SHA-256 checksum
//!
//! Usage: cargo run --bin compile

use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Package {
    name: String,
    description: String,
    category: String,
    popularity: u8,
    platforms: Platforms,
    #[serde(default)]
    dependencies: Dependencies,
    #[serde(default)]
    alternatives: Vec<String>,
    #[serde(default)]
    related: Vec<String>,
    tags: Vec<String>,
    website: Option<String>,
    license: Option<String>,
    source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Platforms {
    apt: Option<String>,
    brew: Option<String>,
    dnf: Option<String>,
    pacman: Option<String>,
    mas: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Dependencies {
    #[serde(default)]
    required: Vec<Dependency>,
    #[serde(default)]
    optional: Vec<Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Dependency {
    package: String,
    reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageGroup {
    id: String,
    name: String,
    description: String,
    category: String,
    packages: GroupPackages,
    #[serde(default)]
    platform_overrides: HashMap<String, PlatformOverride>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GroupPackages {
    required: Vec<String>,
    #[serde(default)]
    optional: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlatformOverride {
    #[serde(default)]
    packages: Vec<String>,
    #[serde(default)]
    casks: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompiledDatabase {
    version: u32,
    last_updated: String,
    packages: Vec<Package>,
    groups: Vec<PackageGroup>,
    // Indexes
    index_by_name: HashMap<String, usize>,
    index_by_category: HashMap<String, Vec<usize>>,
    index_by_tag: HashMap<String, Vec<usize>>,
}

fn main() -> Result<()> {
    println!("{}", "Compiling Heimdal Packages Database".bold().cyan());
    println!();

    // Load packages
    print!("Loading packages... ");
    let packages = load_packages("packages")?;
    println!("{}", format!("✓ {} packages", packages.len()).green());

    // Load groups
    print!("Loading groups... ");
    let groups = load_groups("groups")?;
    println!("{}", format!("✓ {} groups", groups.len()).green());

    // Validate cross-references
    print!("Validating references... ");
    validate_references(&packages, &groups)?;
    println!("{}", "✓ All references valid".green());

    // Build indexes
    print!("Building indexes... ");
    let index_by_name = build_name_index(&packages);
    let index_by_category = build_category_index(&packages);
    let index_by_tag = build_tag_index(&packages);
    println!("{}", "✓ Indexes built".green());

    // Create database
    let db = CompiledDatabase {
        version: 1,
        last_updated: chrono::Utc::now().to_rfc3339(),
        packages,
        groups,
        index_by_name,
        index_by_category,
        index_by_tag,
    };

    // Serialize to bincode (using default config for simplicity)
    print!("Serializing to bincode... ");
    let encoded = bincode::serialize(&db)?;
    println!("{}", format!("✓ {} bytes", encoded.len()).green());

    // Test deserialization immediately
    print!("Testing deserialization... ");
    let decoded: CompiledDatabase = bincode::deserialize(&encoded)?;
    println!(
        "{}",
        format!(
            "✓ {} packages, {} groups",
            decoded.packages.len(),
            decoded.groups.len()
        )
        .green()
    );

    // Write to file
    fs::create_dir_all("target")?;
    fs::write("target/packages.db", &encoded)?;
    println!("{}", "✓ Wrote target/packages.db".green());

    // Generate checksum
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(&encoded);
    let checksum = format!("{:x}", hash);
    fs::write("target/packages.db.sha256", &checksum)?;
    println!("{}", "✓ Wrote target/packages.db.sha256".green());

    println!();
    println!("{}", "Database compiled successfully!".bold().green());
    println!("  Version: {}", db.version);
    println!("  Packages: {}", db.packages.len());
    println!("  Groups: {}", db.groups.len());
    println!("  Size: {} KB", encoded.len() / 1024);
    println!("  SHA-256: {}", &checksum[..16]);

    Ok(())
}

fn load_packages(dir: &str) -> Result<Vec<Package>> {
    let mut packages = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "yaml"))
    {
        let content = fs::read_to_string(entry.path())?;
        let package: Package = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", entry.path().display()))?;

        // Validate filename matches package name
        let expected_name = format!("{}.yaml", package.name);
        let actual_name = entry.path().file_name().unwrap().to_str().unwrap();
        if actual_name != expected_name {
            anyhow::bail!(
                "Filename mismatch: {} should be {}",
                entry.path().display(),
                expected_name
            );
        }

        packages.push(package);
    }

    Ok(packages)
}

fn load_groups(dir: &str) -> Result<Vec<PackageGroup>> {
    let mut groups = Vec::new();

    if !Path::new(dir).exists() {
        return Ok(groups);
    }

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "yaml"))
    {
        let content = fs::read_to_string(entry.path())?;
        let group: PackageGroup = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", entry.path().display()))?;
        groups.push(group);
    }

    Ok(groups)
}

fn validate_references(packages: &[Package], groups: &[PackageGroup]) -> Result<()> {
    let package_names: std::collections::HashSet<_> = packages.iter().map(|p| &p.name).collect();

    // Validate package dependencies
    for pkg in packages {
        for dep in &pkg.dependencies.required {
            if !package_names.contains(&dep.package) {
                anyhow::bail!(
                    "Package '{}' has unknown required dependency: '{}'",
                    pkg.name,
                    dep.package
                );
            }
        }
        for dep in &pkg.dependencies.optional {
            if !package_names.contains(&dep.package) {
                anyhow::bail!(
                    "Package '{}' has unknown optional dependency: '{}'",
                    pkg.name,
                    dep.package
                );
            }
        }
    }

    // Validate group packages
    for group in groups {
        for pkg_name in &group.packages.required {
            if !package_names.contains(pkg_name) {
                anyhow::bail!(
                    "Group '{}' references unknown package: '{}'",
                    group.id,
                    pkg_name
                );
            }
        }
        for pkg_name in &group.packages.optional {
            if !package_names.contains(pkg_name) {
                anyhow::bail!(
                    "Group '{}' references unknown package: '{}'",
                    group.id,
                    pkg_name
                );
            }
        }
    }

    Ok(())
}

fn build_name_index(packages: &[Package]) -> HashMap<String, usize> {
    packages
        .iter()
        .enumerate()
        .map(|(i, p)| (p.name.clone(), i))
        .collect()
}

fn build_category_index(packages: &[Package]) -> HashMap<String, Vec<usize>> {
    let mut index: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, pkg) in packages.iter().enumerate() {
        index.entry(pkg.category.clone()).or_default().push(i);
    }
    index
}

fn build_tag_index(packages: &[Package]) -> HashMap<String, Vec<usize>> {
    let mut index: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, pkg) in packages.iter().enumerate() {
        for tag in &pkg.tags {
            index.entry(tag.clone()).or_default().push(i);
        }
    }
    index
}
