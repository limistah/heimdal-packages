//! Schema validation tests

use jsonschema::JSONSchema;
use serde_json::Value;
use std::fs;

#[test]
fn test_package_schema_is_valid() {
    let schema_content =
        fs::read_to_string("schemas/package.schema.json").expect("Failed to read package schema");

    let schema: Value =
        serde_json::from_str(&schema_content).expect("Package schema is not valid JSON");

    // Verify schema can be compiled
    JSONSchema::options()
        .compile(&schema)
        .expect("Package schema cannot be compiled");
}

#[test]
fn test_group_schema_is_valid() {
    let schema_content =
        fs::read_to_string("schemas/group.schema.json").expect("Failed to read group schema");

    let schema: Value =
        serde_json::from_str(&schema_content).expect("Group schema is not valid JSON");

    JSONSchema::options()
        .compile(&schema)
        .expect("Group schema cannot be compiled");
}

#[test]
fn test_profile_schema_is_valid() {
    let schema_content =
        fs::read_to_string("schemas/profile.schema.json").expect("Failed to read profile schema");

    let schema: Value =
        serde_json::from_str(&schema_content).expect("Profile schema is not valid JSON");

    JSONSchema::options()
        .compile(&schema)
        .expect("Profile schema cannot be compiled");
}

#[test]
fn test_all_packages_validate_against_schema() {
    let schema_content =
        fs::read_to_string("schemas/package.schema.json").expect("Failed to read package schema");
    let schema_value: Value =
        serde_json::from_str(&schema_content).expect("Package schema is not valid JSON");
    let compiled_schema = JSONSchema::options()
        .compile(&schema_value)
        .expect("Failed to compile package schema");

    let mut validated_count = 0;
    for entry in walkdir::WalkDir::new("packages")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        let yaml_content = fs::read_to_string(entry.path())
            .expect(&format!("Failed to read {}", entry.path().display()));

        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_content)
            .expect(&format!("Invalid YAML in {}", entry.path().display()));

        let json_value: Value = serde_json::to_value(&yaml_value).expect(&format!(
            "Failed to convert YAML to JSON for {}",
            entry.path().display()
        ));

        let validation_result = compiled_schema.validate(&json_value);
        if let Err(errors) = validation_result {
            let error_messages: Vec<String> = errors.map(|e| e.to_string()).collect();
            panic!(
                "Package {} failed schema validation: {}",
                entry.path().display(),
                error_messages.join(", ")
            );
        }

        validated_count += 1;
    }

    assert!(validated_count > 0, "No packages were validated");
}

#[test]
fn test_all_groups_validate_against_schema() {
    let schema_content =
        fs::read_to_string("schemas/group.schema.json").expect("Failed to read group schema");
    let schema_value: Value =
        serde_json::from_str(&schema_content).expect("Group schema is not valid JSON");
    let compiled_schema = JSONSchema::options()
        .compile(&schema_value)
        .expect("Failed to compile group schema");

    let mut validated_count = 0;
    for entry in walkdir::WalkDir::new("groups")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        let yaml_content = fs::read_to_string(entry.path())
            .expect(&format!("Failed to read {}", entry.path().display()));

        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_content)
            .expect(&format!("Invalid YAML in {}", entry.path().display()));

        let json_value: Value = serde_json::to_value(&yaml_value).expect(&format!(
            "Failed to convert YAML to JSON for {}",
            entry.path().display()
        ));

        let validation_result = compiled_schema.validate(&json_value);
        if let Err(errors) = validation_result {
            let error_messages: Vec<String> = errors.map(|e| e.to_string()).collect();
            panic!(
                "Group {} failed schema validation: {}",
                entry.path().display(),
                error_messages.join(", ")
            );
        }

        validated_count += 1;
    }

    assert!(validated_count > 0, "No groups were validated");
}

#[test]
fn test_all_profiles_validate_against_schema() {
    let schema_content =
        fs::read_to_string("schemas/profile.schema.json").expect("Failed to read profile schema");
    let schema_value: Value =
        serde_json::from_str(&schema_content).expect("Profile schema is not valid JSON");
    let compiled_schema = JSONSchema::options()
        .compile(&schema_value)
        .expect("Failed to compile profile schema");

    let mut validated_count = 0;
    for entry in walkdir::WalkDir::new("profiles")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
    {
        let yaml_content = fs::read_to_string(entry.path())
            .expect(&format!("Failed to read {}", entry.path().display()));

        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_content)
            .expect(&format!("Invalid YAML in {}", entry.path().display()));

        let json_value: Value = serde_json::to_value(&yaml_value).expect(&format!(
            "Failed to convert YAML to JSON for {}",
            entry.path().display()
        ));

        let validation_result = compiled_schema.validate(&json_value);
        if let Err(errors) = validation_result {
            let error_messages: Vec<String> = errors.map(|e| e.to_string()).collect();
            panic!(
                "Profile {} failed schema validation: {}",
                entry.path().display(),
                error_messages.join(", ")
            );
        }

        validated_count += 1;
    }

    assert!(validated_count > 0, "No profiles were validated");
}
