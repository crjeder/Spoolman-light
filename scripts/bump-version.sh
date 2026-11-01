#!/usr/bin/env bash
# Usage: scripts/bump-version.sh <major|minor|patch|X.Y.Z>
# Bumps [workspace.package] version in Cargo.toml (SemVer) and refreshes Cargo.lock.
set -euo pipefail
cd "$(dirname "$0")/.."

cur=$(sed -n '/^\[workspace.package\]/,/^\[/s/^version = "\(.*\)"/\1/p' Cargo.toml)
[[ $cur =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]] || { echo "cannot parse current version: $cur" >&2; exit 1; }
maj=${BASH_REMATCH[1]} min=${BASH_REMATCH[2]} pat=${BASH_REMATCH[3]}

case "${1:-}" in
  major) new="$((maj + 1)).0.0" ;;
  minor) new="$maj.$((min + 1)).0" ;;
  patch) new="$maj.$min.$((pat + 1))" ;;
  *)
    [[ ${1:-} =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]] || { echo "usage: $0 <major|minor|patch|X.Y.Z>" >&2; exit 1; }
    new=$1
    [[ $(printf '%s\n%s\n' "$cur" "$new" | sort -V | tail -1) == "$new" && $new != "$cur" ]] \
      || { echo "$new is not greater than $cur" >&2; exit 1; }
    ;;
esac

sed -i "/^\[workspace.package\]/,/^\[/s/^version = \".*\"/version = \"$new\"/" Cargo.toml
cargo update -w
echo "$cur -> $new"
