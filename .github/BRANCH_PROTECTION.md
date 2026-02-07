# Branch Protection Rules

This document describes the required branch protection rules for the heimdal-packages repository.

## Setup Instructions

Go to: `https://github.com/limistah/heimdal-packages/settings/branches`

## Main Branch Protection

### Required Settings for `main` branch:

1. **Require a pull request before merging**
   - ✅ Enable
   - Require approvals: 1 (recommended)
   - Dismiss stale pull request approvals when new commits are pushed: ✅

2. **Require status checks to pass before merging**
   - ✅ Enable
   - Require branches to be up to date before merging: ✅
   - Status checks that are required:
     - `Validate Package Database`
     - `Security Audit`

3. **Require conversation resolution before merging**
   - ✅ Enable

4. **Require linear history**
   - ✅ Enable (prevents merge commits, requires rebase or squash)

5. **Do not allow bypassing the above settings**
   - ✅ Enable

6. **Restrict who can push to matching branches**
   - ✅ Enable
   - Allowed to push: (empty - no one can push directly)
   - Include administrators: ❌ (admins also must follow rules)

7. **Allow force pushes**
   - ❌ Disable (CRITICAL: prevents `git push --force`)

8. **Allow deletions**
   - ❌ Disable

## Dev Branch Protection

### Required Settings for `dev` branch:

1. **Require status checks to pass before merging**
   - ✅ Enable
   - Status checks that are required:
     - `Validate Package Database`
     - `Security Audit`

2. **Require linear history**
   - ✅ Enable

3. **Allow force pushes**
   - ❌ Disable

## Workflow

```
feature-branch → dev → main
     ↓          ↓      ↓
   commits    PR+CI  PR+CI
              (auto) (manual)
```

### Development Flow:

1. **Feature Development**
   ```bash
   git checkout dev
   git pull origin dev
   git checkout -b feature/my-feature
   # Make changes
   git commit -m "feat: add new feature"
   git push origin feature/my-feature
   ```

2. **Create PR to dev**
   - CI runs automatically
   - Requires all checks to pass
   - Merge to `dev` after approval

3. **Release to main**
   ```bash
   git checkout dev
   git pull origin dev
   git checkout -b release/v1.x.x
   git push origin release/v1.x.x
   ```
   - Create PR from `release/v1.x.x` → `main`
   - CI runs automatically
   - Requires manual approval
   - Triggers release workflow on merge

## Enforcement

- ❌ **NEVER** `git push --force` to `main` or `dev`
- ❌ **NEVER** push directly to `main`
- ✅ **ALWAYS** use PRs for merging to `main`
- ✅ **ALWAYS** ensure CI passes before merging
- ✅ `dev` is the default branch for development
- ✅ `main` is protected for releases only

## CI Checks Required

All PRs must pass:

1. **Rust Formatting** (`cargo fmt --check`)
2. **Clippy Lints** (`cargo clippy -- -D warnings`)
3. **YAML Schema Validation** (`cargo run --bin validate`)
4. **Database Compilation** (`cargo run --bin compile`)
5. **Deserialization Test** (included in compile step)
6. **Security Audit** (`cargo audit`)

## Violations

If branch protection is bypassed:

1. Revert the commit immediately
2. Investigate how it happened
3. Review and strengthen protection rules
4. Document the incident

---

**Important**: These rules ensure database integrity and prevent breaking changes from reaching production.
