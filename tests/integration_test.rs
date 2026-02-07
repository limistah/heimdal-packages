//! Integration tests for database compilation and deserialization

use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_database_compilation() {
    // Run the compile script
    let output = Command::new("cargo")
        .args(["run", "--bin", "compile"])
        .output()
        .expect("Failed to run compile command");

    assert!(
        output.status.success(),
        "Compilation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that database file was created
    let db_path = Path::new("target/packages.db");
    assert!(
        db_path.exists(),
        "Database file was not created at target/packages.db"
    );

    // Check that checksum file was created
    let checksum_path = Path::new("target/packages.db.sha256");
    assert!(
        checksum_path.exists(),
        "Checksum file was not created at target/packages.db.sha256"
    );

    // Check database file is non-empty
    let metadata = fs::metadata(db_path).expect("Failed to read database metadata");
    let size_bytes = metadata.len();
    assert!(size_bytes > 0, "Database file is empty (0 bytes)");
}

#[test]
fn test_validation_passes() {
    // Run the validation script
    let output = Command::new("cargo")
        .args(["run", "--bin", "validate"])
        .output()
        .expect("Failed to run validate command");

    assert!(
        output.status.success(),
        "Validation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("All validations passed"),
        "Validation did not pass successfully"
    );
}

#[test]
fn test_stats_generation() {
    // Run the stats script
    let output = Command::new("cargo")
        .args(["run", "--bin", "stats"])
        .output()
        .expect("Failed to run stats command");

    assert!(
        output.status.success(),
        "Stats generation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("DATABASE OVERVIEW"),
        "Stats output missing database overview"
    );
    assert!(
        stdout.contains("Packages:"),
        "Stats output missing package count"
    );
}

#[test]
fn test_all_packages_have_files() {
    // Get list of all package YAML files
    let packages_dir = Path::new("packages");
    assert!(packages_dir.exists(), "packages directory does not exist");

    let mut package_count = 0;
    for entry in walkdir::WalkDir::new(packages_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        package_count += 1;

        // Verify file is readable
        let content = fs::read_to_string(entry.path())
            .expect(&format!("Failed to read {}", entry.path().display()));

        // Verify it's valid YAML
        let _: serde_yaml::Value = serde_yaml::from_str(&content)
            .expect(&format!("Invalid YAML in {}", entry.path().display()));
    }

    assert!(
        package_count > 0,
        "No package files found in packages directory"
    );
}

#[test]
fn test_required_directories_exist() {
    let required_dirs = [
        "packages",
        "groups",
        "profiles",
        "mappings",
        "dependencies",
        "suggestions",
        "templates",
        "schemas",
        "scripts",
    ];

    for dir in &required_dirs {
        let path = Path::new(dir);
        assert!(
            path.exists() && path.is_dir(),
            "Required directory '{}' does not exist",
            dir
        );
    }
}

#[test]
fn test_required_schemas_exist() {
    let required_schemas = [
        "schemas/package.schema.json",
        "schemas/group.schema.json",
        "schemas/profile.schema.json",
        "schemas/mapping.schema.json",
        "schemas/dependency.schema.json",
        "schemas/suggestion.schema.json",
        "schemas/template.schema.json",
    ];

    for schema in &required_schemas {
        let path = Path::new(schema);
        assert!(
            path.exists() && path.is_file(),
            "Required schema '{}' does not exist",
            schema
        );

        // Verify it's valid JSON
        let content = fs::read_to_string(path).expect(&format!("Failed to read {}", schema));
        let _: serde_json::Value =
            serde_json::from_str(&content).expect(&format!("Invalid JSON in {}", schema));
    }
}
