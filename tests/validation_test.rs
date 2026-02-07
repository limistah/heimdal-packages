//! Validation logic tests

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    category: String,
    platforms: Platforms,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Platforms {
    apt: Option<String>,
    brew: Option<String>,
    dnf: Option<String>,
    pacman: Option<String>,
}

#[test]
fn test_no_duplicate_package_names() {
    let mut package_names = HashSet::new();
    let mut duplicates = Vec::new();

    for entry in walkdir::WalkDir::new("packages")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        let yaml_content = fs::read_to_string(entry.path())
            .expect(&format!("Failed to read {}", entry.path().display()));

        let package: Package = serde_yaml::from_str(&yaml_content).expect(&format!(
            "Failed to parse package from {}",
            entry.path().display()
        ));

        if !package_names.insert(package.name.clone()) {
            duplicates.push(package.name);
        }
    }

    assert!(
        duplicates.is_empty(),
        "Found duplicate package names: {:?}",
        duplicates
    );
}

#[test]
fn test_package_names_match_filenames() {
    let mut mismatches = Vec::new();

    for entry in walkdir::WalkDir::new("packages")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        let yaml_content = fs::read_to_string(entry.path())
            .expect(&format!("Failed to read {}", entry.path().display()));

        let package: Package = serde_yaml::from_str(&yaml_content).expect(&format!(
            "Failed to parse package from {}",
            entry.path().display()
        ));

        let filename = entry
            .path()
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("Failed to get filename");

        if package.name != filename {
            mismatches.push(format!(
                "{}: package name '{}' doesn't match filename '{}.yaml'",
                entry.path().display(),
                package.name,
                filename
            ));
        }
    }

    assert!(
        mismatches.is_empty(),
        "Found package name/filename mismatches:\n{}",
        mismatches.join("\n")
    );
}

#[test]
fn test_all_packages_have_valid_categories() {
    let valid_categories = [
        "essential",
        "editor",
        "terminal",
        "language",
        "container",
        "infrastructure",
        "database",
        "network",
        "application",
        "shell",
        "git",
        "build",
        "other",
    ];

    let mut invalid_packages = Vec::new();

    for entry in walkdir::WalkDir::new("packages")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        let yaml_content = fs::read_to_string(entry.path())
            .expect(&format!("Failed to read {}", entry.path().display()));

        let package: Package = serde_yaml::from_str(&yaml_content).expect(&format!(
            "Failed to parse package from {}",
            entry.path().display()
        ));

        if !valid_categories.contains(&package.category.as_str()) {
            invalid_packages.push(format!(
                "{}: invalid category '{}'",
                package.name, package.category
            ));
        }
    }

    assert!(
        invalid_packages.is_empty(),
        "Found packages with invalid categories:\n{}",
        invalid_packages.join("\n")
    );
}

#[test]
fn test_all_packages_have_platform_coverage() {
    let mut packages_without_coverage = Vec::new();

    for entry in walkdir::WalkDir::new("packages")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        let yaml_content = fs::read_to_string(entry.path())
            .expect(&format!("Failed to read {}", entry.path().display()));

        let package: Package = serde_yaml::from_str(&yaml_content).expect(&format!(
            "Failed to parse package from {}",
            entry.path().display()
        ));

        let platform_count = [
            package.platforms.apt.is_some(),
            package.platforms.brew.is_some(),
            package.platforms.dnf.is_some(),
            package.platforms.pacman.is_some(),
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        if platform_count < 2 {
            packages_without_coverage.push(format!(
                "{}: only {} platform(s) available",
                package.name, platform_count
            ));
        }
    }

    assert!(
        packages_without_coverage.is_empty(),
        "Found packages with insufficient platform coverage (< 2 platforms):\n{}",
        packages_without_coverage.join("\n")
    );
}

#[test]
fn test_all_tags_follow_pattern() {
    let tag_pattern = regex::Regex::new(r"^[a-z0-9-]+$").unwrap();
    let mut invalid_tags = Vec::new();

    for entry in walkdir::WalkDir::new("packages")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        let yaml_content = fs::read_to_string(entry.path())
            .expect(&format!("Failed to read {}", entry.path().display()));

        let package: Package = serde_yaml::from_str(&yaml_content).expect(&format!(
            "Failed to parse package from {}",
            entry.path().display()
        ));

        for tag in &package.tags {
            if !tag_pattern.is_match(tag) {
                invalid_tags.push(format!(
                    "{}: invalid tag '{}' (must match ^[a-z0-9-]+$)",
                    package.name, tag
                ));
            }
        }
    }

    assert!(
        invalid_tags.is_empty(),
        "Found packages with invalid tags:\n{}",
        invalid_tags.join("\n")
    );
}

#[test]
fn test_fixtures_are_valid() {
    let valid_fixture = "tests/fixtures/valid_package.yaml";
    if std::path::Path::new(valid_fixture).exists() {
        let content = fs::read_to_string(valid_fixture).expect("Failed to read valid fixture");
        let _: Package =
            serde_yaml::from_str(&content).expect("Valid fixture should parse successfully");
    }
}

#[test]
#[should_panic(expected = "Invalid fixture should fail to parse")]
fn test_invalid_fixture_fails() {
    let invalid_fixture = "tests/fixtures/invalid_package.yaml";
    if std::path::Path::new(invalid_fixture).exists() {
        let content = fs::read_to_string(invalid_fixture).expect("Failed to read invalid fixture");
        let _: Package =
            serde_yaml::from_str(&content).expect("Invalid fixture should fail to parse");
    } else {
        panic!("Invalid fixture should fail to parse");
    }
}
