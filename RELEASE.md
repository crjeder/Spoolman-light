# Releasing

1. Make sure `CHANGELOG.md` has an entry for the new version under `## [X.Y.Z] - YYYY-MM-DD` (Keep a Changelog format).
2. Bump the version in all three crate manifests to match:
   ```bash
   for f in crates/spoolman-types/Cargo.toml crates/spoolman-server/Cargo.toml crates/spoolman-client/Cargo.toml; do
     sed -i 's/^version = ".*"/version = "X.Y.Z"/' "$f"
   done
   cargo check -p spoolman-types -p spoolman-server  # refreshes Cargo.lock
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
- `publish-images` builds and pushes `linux/amd64,arm64,arm/v7` images to `ghcr.io/crjeder/spoolman-light` and Docker Hub, tagged `X.Y.Z`, `X.Y`, and `X`.
- `publish-release` then creates a **draft** GitHub release named `vX.Y.Z` with auto-generated notes.

5. Go to the repo's Releases page, review/edit the draft, and publish it manually.
