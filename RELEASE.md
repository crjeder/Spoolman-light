# Releasing

1. Make sure `CHANGELOG.md` has an entry for the new version under `## [X.Y.Z] - YYYY-MM-DD` (Keep a Changelog format).
2. Bump the version (single source: `[workspace.package]` in the root `Cargo.toml`) following [SemVer](https://semver.org/):
   - `major` - breaking change (API/data format incompatible; `cargo semver-checks` failing on `spoolman-types`)
   - `minor` - backward-compatible new feature
   - `patch` - backward-compatible fix

   ```bash
   scripts/bump-version.sh patch   # or minor | major | X.Y.Z
   ```
3. Commit and push to `master`:
   ```bash
   git add CHANGELOG.md Cargo.lock crates/*/Cargo.toml
   git commit -m "chore: release vX.Y.Z"
   git push
   ```
4. Tag and push the tag — this is what triggers the release:
   ```bash
   git tag vX.Y.Z
   git push origin vX.Y.Z
   ```

Pushing the `vX.Y.Z` tag triggers `.github/workflows/ci.yml`:
- `publish-images` first fails if the tag differs from the workspace version.
- `publish-images` builds and pushes `linux/amd64,arm64,arm/v7` images to `ghcr.io/crjeder/spoolman-light` and Docker Hub, tagged `X.Y.Z`, `X.Y`, and `X`.
- `publish-release` then creates a **draft** GitHub release named `vX.Y.Z` with auto-generated notes.

5. Go to the repo's Releases page, review/edit the draft, and publish it manually.
