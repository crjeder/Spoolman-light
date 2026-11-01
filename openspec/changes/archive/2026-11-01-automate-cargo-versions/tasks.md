## 1. Workspace version

- [x] 1.1 Confirm `rust-toolchain.toml` is >= 1.64 and nothing parses `version` from crate manifests (grep Dockerfile, scripts, CI)
- [x] 1.2 Add `[workspace.package] version = "1.10.3"` to root `Cargo.toml`
- [x] 1.3 Replace the literal version in the three crate manifests with `version.workspace = true`
- [x] 1.4 `cargo check -p spoolman-types -p spoolman-server`; confirm `Cargo.lock` unchanged

## 2. Bump script

- [x] 2.1 Write `scripts/bump-version.sh` (major/minor/patch/explicit, SemVer regex, must increase, edits the workspace line, `cargo update -w`)
- [x] 2.2 Verify scenarios: patch, minor, major, invalid, non-increasing (run on a copy / revert afterwards)

## 3. CI guard

- [x] 3.1 Add tag-vs-workspace-version check as first step of `publish-images` in `.github/workflows/ci.yml`

## 4. Docs

- [x] 4.1 Rewrite RELEASE.md steps 2 and 4 to use the script and add SemVer bump rules
- [x] 4.2 Add CHANGELOG entry and TODO.md update per project convention
