//! Generate statistics for Heimdal Packages Database
//!
//! This script:
//! 1. Counts packages by category
//! 2. Calculates database size
//! 3. Shows platform coverage
//! 4. Displays validation status
//! 5. Generates markdown output for badges
//!
//! Usage: cargo run --bin stats

use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum DependencyItem {
    Simple(String),
    Detailed { package: String, reason: String },
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Dependencies {
    #[serde(default)]
    required: Vec<DependencyItem>,
    #[serde(default)]
    optional: Vec<DependencyItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct PackageGroup {
    id: String,
    name: String,
}

struct DatabaseStats {
    total_packages: usize,
    packages_by_category: HashMap<String, Vec<String>>,
    total_groups: usize,
    total_profiles: usize,
    total_mappings: usize,
    platform_coverage: PlatformCoverage,
    database_size: Option<u64>,
    total_tags: usize,
}

struct PlatformCoverage {
    apt: usize,
    brew: usize,
    dnf: usize,
    pacman: usize,
}

fn main() -> Result<()> {
    println!("{}", "Heimdal Packages Database Statistics".bold().cyan());
    println!();

    let mut stats = DatabaseStats {
        total_packages: 0,
        packages_by_category: HashMap::new(),
        total_groups: 0,
        total_profiles: 0,
        total_mappings: 0,
        platform_coverage: PlatformCoverage {
            apt: 0,
            brew: 0,
            dnf: 0,
            pacman: 0,
        },
        database_size: None,
        total_tags: 0,
    };

    // Count packages
    print!("Scanning packages... ");
    scan_packages(&mut stats)?;
    println!("{}", format!("✓ {} packages", stats.total_packages).green());

    // Count groups
    print!("Scanning groups... ");
    scan_groups(&mut stats)?;
    println!("{}", format!("✓ {} groups", stats.total_groups).green());

    // Count profiles
    print!("Scanning profiles... ");
    scan_profiles(&mut stats)?;
    println!("{}", format!("✓ {} profiles", stats.total_profiles).green());

    // Count mappings
    print!("Scanning mappings... ");
    scan_mappings(&mut stats)?;
    println!("{}", format!("✓ {} mappings", stats.total_mappings).green());

    // Check database size
    print!("Checking database size... ");
    if let Some(size) = get_database_size() {
        stats.database_size = Some(size);
        println!("{}", format!("✓ {} bytes", size).green());
    } else {
        println!("{}", "⚠ Not compiled yet".yellow());
    }

    println!();
    display_stats(&stats);

    Ok(())
}

fn scan_packages(stats: &mut DatabaseStats) -> Result<()> {
    let packages_dir = Path::new("packages");
    if !packages_dir.exists() {
        return Ok(());
    }

    let mut all_tags = std::collections::HashSet::new();

    for entry in WalkDir::new(packages_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        let path = entry.path();
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let package: Package = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))?;

        stats.total_packages += 1;

        // Count by category
        stats
            .packages_by_category
            .entry(package.category.clone())
            .or_default()
            .push(package.name.clone());

        // Count platform coverage
        if package.platforms.apt.is_some() {
            stats.platform_coverage.apt += 1;
        }
        if package.platforms.brew.is_some() {
            stats.platform_coverage.brew += 1;
        }
        if package.platforms.dnf.is_some() {
            stats.platform_coverage.dnf += 1;
        }
        if package.platforms.pacman.is_some() {
            stats.platform_coverage.pacman += 1;
        }

        // Collect unique tags
        for tag in &package.tags {
            all_tags.insert(tag.clone());
        }
    }

    stats.total_tags = all_tags.len();

    Ok(())
}

fn scan_groups(stats: &mut DatabaseStats) -> Result<()> {
    let groups_dir = Path::new("groups");
    if !groups_dir.exists() {
        return Ok(());
    }

    for _entry in WalkDir::new(groups_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        stats.total_groups += 1;
    }

    Ok(())
}

fn scan_profiles(stats: &mut DatabaseStats) -> Result<()> {
    let profiles_dir = Path::new("profiles");
    if !profiles_dir.exists() {
        return Ok(());
    }

    for _entry in WalkDir::new(profiles_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        stats.total_profiles += 1;
    }

    Ok(())
}

fn scan_mappings(stats: &mut DatabaseStats) -> Result<()> {
    let mappings_dir = Path::new("mappings");
    if !mappings_dir.exists() {
        return Ok(());
    }

    for _entry in WalkDir::new(mappings_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        stats.total_mappings += 1;
    }

    Ok(())
}

fn get_database_size() -> Option<u64> {
    let db_path = Path::new("target/packages.db");
    if db_path.exists() {
        fs::metadata(db_path).ok().map(|m| m.len())
    } else {
        None
    }
}

fn display_stats(stats: &DatabaseStats) {
    println!("{}", "═══════════════════════════════════════".bold());
    println!("{}", "            DATABASE OVERVIEW".bold().cyan());
    println!("{}", "═══════════════════════════════════════".bold());
    println!();

    // Overview
    println!("{}", "Overview:".bold().white());
    println!("  Packages:  {}", stats.total_packages.to_string().green());
    println!("  Groups:    {}", stats.total_groups.to_string().green());
    println!("  Profiles:  {}", stats.total_profiles.to_string().green());
    println!("  Mappings:  {}", stats.total_mappings.to_string().green());
    println!("  Tags:      {}", stats.total_tags.to_string().green());

    if let Some(size) = stats.database_size {
        let size_kb = size as f64 / 1024.0;
        println!(
            "  DB Size:   {} ({:.2} KB)",
            format!("{} bytes", size).green(),
            size_kb
        );
    } else {
        println!("  DB Size:   {}", "Not compiled".yellow());
    }

    println!();

    // Categories
    println!("{}", "Packages by Category:".bold().white());
    let mut categories: Vec<_> = stats.packages_by_category.iter().collect();
    categories.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    for (category, packages) in categories {
        println!(
            "  {:15} {} packages",
            format!("{}:", category).cyan(),
            packages.len().to_string().green()
        );
    }

    println!();

    // Platform coverage
    println!("{}", "Platform Coverage:".bold().white());

    if stats.total_packages == 0 {
        // Avoid division by zero when there are no packages
        println!("  apt:       0 packages (0%)");
        println!("  brew:      0 packages (0%)");
        println!("  dnf:       0 packages (0%)");
        println!("  pacman:    0 packages (0%)");
    } else {
        let total = stats.total_packages as f64;
        println!(
            "  apt:       {} packages ({:.0}%)",
            stats.platform_coverage.apt.to_string().green(),
            (stats.platform_coverage.apt as f64 / total) * 100.0
        );
        println!(
            "  brew:      {} packages ({:.0}%)",
            stats.platform_coverage.brew.to_string().green(),
            (stats.platform_coverage.brew as f64 / total) * 100.0
        );
        println!(
            "  dnf:       {} packages ({:.0}%)",
            stats.platform_coverage.dnf.to_string().green(),
            (stats.platform_coverage.dnf as f64 / total) * 100.0
        );
        println!(
            "  pacman:    {} packages ({:.0}%)",
            stats.platform_coverage.pacman.to_string().green(),
            (stats.platform_coverage.pacman as f64 / total) * 100.0
        );
    }

    println!();

    // Badge markdown
    println!("{}", "Badge Markdown (for README):".bold().white());
    println!(
        "  {}",
        format!(
            "[![Packages](https://img.shields.io/badge/packages-{}-green.svg)](#packages)",
            stats.total_packages
        )
        .cyan()
    );

    if let Some(size) = stats.database_size {
        let size_kb = (size as f64 / 1024.0).round() as u64;
        println!(
            "  {}",
            format!(
                "[![Database Size](https://img.shields.io/badge/database-{}KB-orange.svg)](#database)",
                size_kb
            )
            .cyan()
        );
    }

    println!();
    println!("{}", "═══════════════════════════════════════".bold());
}
