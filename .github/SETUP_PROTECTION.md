# Setting Up Branch Protection Rules

This guide walks you through setting up branch protection rules on GitHub for both repositories.

## Prerequisites

- Admin access to both repositories:
  - `limistah/heimdal`
  - `limistah/heimdal-packages`

## Part 1: heimdal-packages Repository

### Step 1: Navigate to Branch Settings

1. Go to: https://github.com/limistah/heimdal-packages
2. Click **Settings** (top right)
3. Click **Branches** (left sidebar under "Code and automation")

### Step 2: Add Rule for `main` Branch

Click **Add rule** or **Add branch protection rule**

**Branch name pattern**: `main`

Check these boxes:

- ✅ **Require a pull request before merging**
  - ✅ Require approvals: **1**
  - ✅ Dismiss stale pull request approvals when new commits are pushed
  
- ✅ **Require status checks to pass before merging**
  - ✅ Require branches to be up to date before merging
  - In the search box, type and select:
    - `Validate Package Database` (will appear after first CI run)
    - `Security Audit` (will appear after first CI run)
  
- ✅ **Require conversation resolution before merging**

- ✅ **Require linear history**

- ✅ **Do not allow bypassing the above settings**

- ✅ **Restrict who can push to matching branches**
  - Leave the list **empty** (no one can push directly)
  
- **Include administrators**: ❌ **Uncheck** this box
  - This ensures even admins must follow the rules

- **Allow force pushes**: ❌ **Ensure this is unchecked**

- **Allow deletions**: ❌ **Ensure this is unchecked**

Click **Create** or **Save changes**

### Step 3: Add Rule for `dev` Branch

Click **Add rule** again

**Branch name pattern**: `dev`

Check these boxes:

- ✅ **Require status checks to pass before merging**
  - ✅ Require branches to be up to date before merging
  - Select:
    - `Validate Package Database`
    - `Security Audit`

- ✅ **Require linear history**

- **Allow force pushes**: ❌ **Ensure unchecked**

- **Allow deletions**: ❌ **Ensure unchecked**

Click **Create** or **Save changes**

### Step 4: Set Default Branch

1. Still in **Settings** → **Branches**
2. Under "Default branch", ensure **dev** is selected
3. If not, click the switch icon and select **dev**

---

## Part 2: heimdal Repository

### Step 1: Navigate to Branch Settings

1. Go to: https://github.com/limistah/heimdal
2. Click **Settings** (top right)
3. Click **Branches** (left sidebar)

### Step 2: Add Rule for `main` Branch

Click **Add rule**

**Branch name pattern**: `main`

Check these boxes:

- ✅ **Require a pull request before merging**
  - ✅ Require approvals: **1**
  - ✅ Dismiss stale pull request approvals when new commits are pushed
  
- ✅ **Require status checks to pass before merging**
  - ✅ Require branches to be up to date before merging
  - Select:
    - `Test (ubuntu-latest, stable)`
    - `Test (macos-latest, stable)`
    - `Build (ubuntu-latest)` (optional but recommended)
    - `Build (macos-latest)` (optional but recommended)
  
- ✅ **Require conversation resolution before merging**

- ✅ **Require linear history**

- ✅ **Do not allow bypassing the above settings**

- ✅ **Restrict who can push to matching branches**
  - Leave empty
  
- **Include administrators**: ❌ **Uncheck**

- **Allow force pushes**: ❌ **Ensure unchecked**

- **Allow deletions**: ❌ **Ensure unchecked**

Click **Create** or **Save changes**

### Step 3: Add Rule for `dev` Branch

Click **Add rule**

**Branch name pattern**: `dev`

Check these boxes:

- ✅ **Require status checks to pass before merging**
  - ✅ Require branches to be up to date before merging
  - Select:
    - `Test (ubuntu-latest, stable)`
    - `Test (macos-latest, stable)`

- ✅ **Require linear history**

- **Allow force pushes**: ❌ **Ensure unchecked**

- **Allow deletions**: ❌ **Ensure unchecked**

Click **Create** or **Save changes**

### Step 4: Set Default Branch

1. Still in **Settings** → **Branches**
2. Ensure **dev** is the default branch
3. If not, switch to **dev**

---

## Part 3: Trigger First CI Run

The status checks won't appear in the branch protection UI until they've run at least once.

### For heimdal-packages:

```bash
cd /path/to/heimdal-packages
git push origin main
git push origin dev
```

Wait for the CI workflow to complete, then go back and add the status checks.

### For heimdal:

```bash
cd /path/to/heimdal
git push origin dev
```

Wait for CI to complete, then add the status checks.

---

## Verification

### Test the Protection Rules

1. **Try to push directly to main** (should fail):
   ```bash
   git checkout main
   echo "test" >> README.md
   git commit -am "test"
   git push origin main
   # Should get: remote: error: GH006: Protected branch update failed
   ```

2. **Try to force push** (should fail):
   ```bash
   git push --force origin main
   # Should get: remote: error: GH006: Protected branch update failed
   ```

3. **Create a proper PR** (should work):
   ```bash
   git checkout dev
   git pull origin dev
   git checkout -b test/protection
   echo "test" >> README.md
   git commit -am "test: verify branch protection"
   git push origin test/protection
   # Create PR on GitHub - CI should run
   ```

---

## Troubleshooting

### "Status checks not showing up"

- Wait for CI to run at least once
- Refresh the branch protection settings page
- The check names are case-sensitive

### "Can't select administrators exclusion"

- This is intentional - admins should follow rules too
- If you need to bypass temporarily, you can edit the rule

### "CI failing on first run"

Check:
- `cargo run --bin validate` works locally
- `cargo run --bin compile` works locally
- Cargo.lock is committed
- All dependencies available

---

## Important Notes

- ✅ Branch protection is now active
- ✅ No one (including admins) can push directly to `main`
- ✅ No force pushes allowed
- ✅ All changes must go through PR with CI passing
- ✅ Linear history enforced (no merge commits)

## Next Steps

After setting up branch protection:

1. Test the workflow with a small PR
2. Verify CI runs on all PRs
3. Document any repo-specific requirements
4. Train team members on the workflow

---

**Remember**: These rules protect your code quality and ensure database integrity. Don't bypass them!
