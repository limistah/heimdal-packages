## Description
<!-- Describe your changes in detail -->

## Type of Change
<!-- Mark the relevant option with an "x" -->

- [ ] New package addition
- [ ] Package update (version, platforms, etc.)
- [ ] Package removal
- [ ] New group addition
- [ ] Group update
- [ ] Bug fix (non-breaking change)
- [ ] Documentation update
- [ ] CI/workflow update

## Packages Modified
<!-- List the packages affected by this change -->

- 

## Testing
<!-- Describe the testing you've done -->

- [ ] Ran `cargo run --bin validate` successfully
- [ ] Ran `cargo run --bin compile` successfully
- [ ] Verified deserialization works (compile includes test)
- [ ] Tested with heimdal locally (if applicable)

## Validation Checklist

- [ ] YAML files follow the schema in `schemas/`
- [ ] Package names match filename (e.g., `git.yaml` for package `git`)
- [ ] All dependencies/alternatives reference existing packages
- [ ] Platform-specific packages have correct identifiers
- [ ] Popularity scores are between 0-100
- [ ] Tags are descriptive and consistent with existing packages
- [ ] No duplicate packages

## Database Impact

- **Packages added**: 0
- **Packages modified**: 0
- **Packages removed**: 0
- **Expected database size change**: +/- X KB

## Additional Notes
<!-- Any additional information, breaking changes, migration notes, etc. -->

---

**For Maintainers:**

- [ ] CI checks passed
- [ ] Changes reviewed and approved
- [ ] Database compiles without errors
- [ ] Deserialization test passed
- [ ] Ready to merge
