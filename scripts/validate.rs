//! Validate YAML package database files
//!
//! This script:
//! 1. Loads JSON schemas from schemas/ directory
//! 2. Validates all YAML files against schemas
//! 3. Checks for duplicate package names
//! 4. Verifies cross-references between packages
//! 5. Validates filename matches package name
//! 6. Ensures minimum platform coverage
//!
//! Usage: cargo run --bin validate

use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Debug, Serialize, Deserialize)]
struct Platforms {
    apt: Option<String>,
    brew: Option<String>,
    dnf: Option<String>,
    pacman: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Dependencies {
    #[serde(default)]
    required: Vec<Dependency>,
    #[serde(default)]
    optional: Vec<Dependency>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dependency {
    package: String,
    reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageGroup {
    id: String,
    name: String,
    packages: GroupPackages,
}

#[derive(Debug, Serialize, Deserialize)]
struct GroupPackages {
    required: Vec<String>,
    #[serde(default)]
    optional: Vec<String>,
}

struct ValidationStats {
    packages_validated: usize,
    groups_validated: usize,
    errors: Vec<String>,
    warnings: Vec<String>,
}

fn main() -> Result<()> {
    println!("{}", "Validating Heimdal Packages Database".bold().cyan());
    println!();

    let mut stats = ValidationStats {
        packages_validated: 0,
        groups_validated: 0,
        errors: Vec::new(),
        warnings: Vec::new(),
    };

    // Load schemas
    print!("Loading JSON schemas... ");
    let package_schema_value = Box::leak(Box::new(load_schema("schemas/package.schema.json")?));
    let group_schema_value = Box::leak(Box::new(load_schema("schemas/group.schema.json")?));

    let package_schema = jsonschema::JSONSchema::options()
        .compile(package_schema_value)
        .context("Failed to compile package schema")?;
    let group_schema = jsonschema::JSONSchema::options()
        .compile(group_schema_value)
        .context("Failed to compile group schema")?;
    println!("{}", "✓".green());

    // Validate packages
    print!("Validating packages... ");
    let packages = validate_packages("packages", &package_schema, &mut stats)?;
    println!("{}", format!("✓ {} packages", packages.len()).green());

    // Validate groups
    print!("Validating groups... ");
    let groups = validate_groups("groups", &group_schema, &mut stats)?;
    println!("{}", format!("✓ {} groups", groups.len()).green());

    // Check for duplicates
    print!("Checking for duplicates... ");
    check_duplicates(&packages, &mut stats)?;
    println!("{}", "✓".green());

    // Validate cross-references
    print!("Validating cross-references... ");
    validate_cross_references(&packages, &groups, &mut stats)?;
    println!("{}", "✓".green());

    // Validate platform coverage
    print!("Checking platform coverage... ");
    validate_platform_coverage(&packages, &mut stats)?;
    println!("{}", "✓".green());

    // Print summary
    println!();
    println!("{}", "Validation Summary".bold());
    println!("  Packages: {}", stats.packages_validated);
    println!("  Groups: {}", stats.groups_validated);

    if !stats.warnings.is_empty() {
        println!();
        println!(
            "{}",
            format!("⚠ {} Warnings:", stats.warnings.len()).yellow()
        );
        for warning in &stats.warnings {
            println!("  {}", warning.yellow());
        }
    }

    if !stats.errors.is_empty() {
        println!();
        println!(
            "{}",
            format!("✗ {} Errors:", stats.errors.len()).red().bold()
        );
        for error in &stats.errors {
            println!("  {}", error.red());
        }
        std::process::exit(1);
    }

    println!();
    println!("{}", "All validations passed! ✓".green().bold());
    Ok(())
}

fn load_schema(path: &str) -> Result<serde_json::Value> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read schema: {}", path))?;
    let schema: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse schema: {}", path))?;
    Ok(schema)
}

fn validate_packages(
    dir: &str,
    schema: &jsonschema::JSONSchema,
    stats: &mut ValidationStats,
) -> Result<Vec<Package>> {
    let mut packages = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "yaml"))
    {
        let path = entry.path();
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        // Parse as JSON for schema validation
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse YAML: {}", path.display()))?;
        let json_value: serde_json::Value =
            serde_json::to_value(&yaml_value).context("Failed to convert YAML to JSON")?;

        // Validate against schema
        if let Err(errors) = schema.validate(&json_value) {
            for error in errors {
                stats.errors.push(format!("{}: {}", path.display(), error));
            }
            continue;
        }

        // Parse as Package struct
        let package: Package = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse package: {}", path.display()))?;

        // Validate filename matches package name
        let expected_filename = format!("{}.yaml", package.name);
        let actual_filename = path.file_name().unwrap().to_str().unwrap();
        if actual_filename != expected_filename {
            stats.errors.push(format!(
                "{}: Filename '{}' doesn't match package name '{}' (expected '{}')",
                path.display(),
                actual_filename,
                package.name,
                expected_filename
            ));
            continue;
        }

        // Validate popularity range
        if package.popularity > 100 {
            stats.errors.push(format!(
                "{}: Popularity {} exceeds maximum of 100",
                path.display(),
                package.popularity
            ));
        }

        // Validate tags are lowercase and hyphenated
        for tag in &package.tags {
            if !tag
                .chars()
                .all(|c| c.is_lowercase() || c == '-' || c.is_numeric())
            {
                stats.warnings.push(format!(
                    "{}: Tag '{}' should be lowercase with hyphens only",
                    path.display(),
                    tag
                ));
            }
        }

        stats.packages_validated += 1;
        packages.push(package);
    }

    Ok(packages)
}

fn validate_groups(
    dir: &str,
    schema: &jsonschema::JSONSchema,
    stats: &mut ValidationStats,
) -> Result<Vec<PackageGroup>> {
    let mut groups = Vec::new();

    if !Path::new(dir).exists() {
        return Ok(groups);
    }

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "yaml"))
    {
        let path = entry.path();
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        // Parse as JSON for schema validation
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse YAML: {}", path.display()))?;
        let json_value: serde_json::Value =
            serde_json::to_value(&yaml_value).context("Failed to convert YAML to JSON")?;

        // Validate against schema
        if let Err(errors) = schema.validate(&json_value) {
            for error in errors {
                stats.errors.push(format!("{}: {}", path.display(), error));
            }
            continue;
        }

        let group: PackageGroup = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse group: {}", path.display()))?;

        stats.groups_validated += 1;
        groups.push(group);
    }

    Ok(groups)
}

fn check_duplicates(packages: &[Package], stats: &mut ValidationStats) -> Result<()> {
    let mut seen = HashMap::new();

    for package in packages {
        if let Some(first_occurrence) = seen.get(&package.name) {
            stats.errors.push(format!(
                "Duplicate package name '{}' (first seen in {})",
                package.name, first_occurrence
            ));
        } else {
            seen.insert(package.name.clone(), package.name.clone());
        }
    }

    Ok(())
}

fn validate_cross_references(
    packages: &[Package],
    groups: &[PackageGroup],
    stats: &mut ValidationStats,
) -> Result<()> {
    let package_names: HashSet<_> = packages.iter().map(|p| &p.name).collect();

    // Validate package dependencies
    for package in packages {
        for dep in &package.dependencies.required {
            if !package_names.contains(&dep.package) {
                stats.errors.push(format!(
                    "Package '{}' has unknown required dependency: '{}'",
                    package.name, dep.package
                ));
            }
        }
        for dep in &package.dependencies.optional {
            if !package_names.contains(&dep.package) {
                stats.errors.push(format!(
                    "Package '{}' has unknown optional dependency: '{}'",
                    package.name, dep.package
                ));
            }
        }
        for alt in &package.alternatives {
            if !package_names.contains(alt) {
                stats.warnings.push(format!(
                    "Package '{}' references unknown alternative: '{}'",
                    package.name, alt
                ));
            }
        }
        for rel in &package.related {
            if !package_names.contains(rel) {
                stats.warnings.push(format!(
                    "Package '{}' references unknown related package: '{}'",
                    package.name, rel
                ));
            }
        }
    }

    // Validate group packages
    for group in groups {
        for pkg_name in &group.packages.required {
            if !package_names.contains(pkg_name) {
                stats.errors.push(format!(
                    "Group '{}' references unknown required package: '{}'",
                    group.id, pkg_name
                ));
            }
        }
        for pkg_name in &group.packages.optional {
            if !package_names.contains(pkg_name) {
                stats.errors.push(format!(
                    "Group '{}' references unknown optional package: '{}'",
                    group.id, pkg_name
                ));
            }
        }
    }

    Ok(())
}

fn validate_platform_coverage(packages: &[Package], stats: &mut ValidationStats) -> Result<()> {
    for package in packages {
        let mut platform_count = 0;
        if package.platforms.apt.is_some() {
            platform_count += 1;
        }
        if package.platforms.brew.is_some() {
            platform_count += 1;
        }
        if package.platforms.dnf.is_some() {
            platform_count += 1;
        }
        if package.platforms.pacman.is_some() {
            platform_count += 1;
        }

        if platform_count < 2 {
            stats.warnings.push(format!(
                "Package '{}' has only {} platform mapping(s) (recommended: at least 2)",
                package.name, platform_count
            ));
        }
    }

    Ok(())
}
