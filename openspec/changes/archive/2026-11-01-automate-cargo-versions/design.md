## Context

Three crates (`spoolman-types`, `spoolman-server`, `spoolman-client`) are versioned in lockstep and none is published to crates.io. Docker image tags come from the git tag (`docker/metadata-action` semver patterns), not from Cargo. Release is manual per RELEASE.md. `cargo-semver-checks` already runs on PRs for `spoolman-types`.

## Goals / Non-Goals

**Goals:**
- One place holds the version.
- Bump level follows SemVer 2.0.0.
- A tag that disagrees with Cargo.toml cannot publish.

**Non-Goals:**
- Auto-deriving the bump from commits (release-plz / conventional commits); the repo does not use conventional-commit discipline consistently.
- Publishing to crates.io.
- Independent per-crate versions.

## Decisions

- **`[workspace.package] version` + `version.workspace = true`** (stable since Rust 1.64) over keeping three literals plus a sync script: removes the drift class entirely. `rust-toolchain.toml` must be >= 1.64; verify during implementation.
- **Bump script in plain bash, no new tool.** Parses `X.Y.Z` from the workspace line, applies major/minor/patch (resetting lower parts) or accepts an explicit version validated against the SemVer regex, rewrites the one line, runs `cargo update -w` to refresh the lockfile. Alternative `cargo-edit set-version` / `cargo-release` / `release-plz`: extra installed tool for a one-line edit; rejected. Check crates.io before implementing in case a lightweight option is preferable.
- **Bump level stays a human decision**, guided by RELEASE.md rules and the existing `semver-checks` PR job (a failing check on `spoolman-types` means major). The script rejects a non-increasing version.
- **Tag guard as first step of `publish-images`**: compare `${GITHUB_REF_NAME#v}` with the version read via `cargo metadata --no-deps` (jq on `.workspace_members`/packages), fail on mismatch. Placed before image build so no mislabelled image is pushed.

## Risks / Trade-offs

- cargo-leptos or Dockerfile reading `version` from a crate manifest with a regex -> grep the repo for such use during implementation; `cargo metadata` resolves inheritance anyway.
- Human picks the wrong bump level -> semver-checks covers the types crate only; accepted.
