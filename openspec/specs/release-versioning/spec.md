# Release Versioning Spec

## Requirements

### Requirement: Single-source workspace version
The workspace SHALL define the package version once in the root `Cargo.toml` under `[workspace.package]`, and every member crate SHALL inherit it via `version.workspace = true`.

#### Scenario: Crates share the version
- **WHEN** `cargo metadata --no-deps` is run
- **THEN** `spoolman-types`, `spoolman-server` and `spoolman-client` all report the same version

#### Scenario: No literal versions in crates
- **WHEN** a member crate manifest is inspected
- **THEN** its `[package]` table contains no literal `version = "..."` line

### Requirement: SemVer-conformant bump script
`scripts/bump-version.sh` SHALL accept `major`, `minor`, `patch` or an explicit `X.Y.Z`, update only the workspace version, and refresh `Cargo.lock`.

#### Scenario: Patch bump
- **WHEN** the version is `1.10.3` and the script is run with `patch`
- **THEN** the version becomes `1.10.4`

#### Scenario: Minor bump resets patch
- **WHEN** the version is `1.10.3` and the script is run with `minor`
- **THEN** the version becomes `1.11.0`

#### Scenario: Major bump resets minor and patch
- **WHEN** the version is `1.10.3` and the script is run with `major`
- **THEN** the version becomes `2.0.0`

#### Scenario: Invalid or non-increasing explicit version
- **WHEN** the script is run with `1.10` or with a version not greater than the current one
- **THEN** it exits non-zero and changes no file

### Requirement: Tag matches workspace version
The release workflow SHALL fail before building or pushing images when the pushed `vX.Y.Z` tag differs from the workspace version.

#### Scenario: Matching tag
- **WHEN** tag `v1.10.4` is pushed and the workspace version is `1.10.4`
- **THEN** the check passes and image publishing proceeds

#### Scenario: Mismatched tag
- **WHEN** tag `v1.10.4` is pushed and the workspace version is `1.10.3`
- **THEN** the job fails and no image is pushed

### Requirement: Documented bump rules
RELEASE.md SHALL state when to bump major (breaking change), minor (backward-compatible feature) and patch (backward-compatible fix) per SemVer 2.0.0.

#### Scenario: Release instructions
- **WHEN** a maintainer reads RELEASE.md
- **THEN** they find the bump rules and the `scripts/bump-version.sh` invocation
