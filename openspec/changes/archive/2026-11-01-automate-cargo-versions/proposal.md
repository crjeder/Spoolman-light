## Why

The version `1.10.3` is hand-copied into three crate manifests on every release (RELEASE.md step 2, via `sed`). Easy to forget one, nothing checks that the git tag matches the manifests, and nothing ties the bump size to SemVer.

## What Changes

- Define the version once in the root `Cargo.toml` under `[workspace.package]`; the three crates inherit it with `version.workspace = true`.
- Add a CI guard on `v*` tags: fail before publishing images if the tag does not equal the workspace version.
- Add `scripts/bump-version.sh <major|minor|patch|X.Y.Z>`: computes the next SemVer from the current workspace version, edits the single line, refreshes `Cargo.lock`.
- Update RELEASE.md to use the script and document the SemVer bump rules (breaking = major, feature = minor, fix = patch; 0.x not used, project is >= 1.0).

## Capabilities

### New Capabilities
- `release-versioning`: single-source workspace version, SemVer-conformant bumping, and tag/version consistency check.

### Modified Capabilities

## Impact

- `Cargo.toml`, `crates/*/Cargo.toml` (version lines), `Cargo.lock` (unchanged content, version stays 1.10.3).
- `.github/workflows/ci.yml` (new step in `publish-images` path).
- New `scripts/bump-version.sh`; `RELEASE.md` rewritten steps 2 and 4.
- No runtime or API change.
