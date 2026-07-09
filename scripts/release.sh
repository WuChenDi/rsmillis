#!/usr/bin/env bash
# Release millis to crates.io: quality gate, dry-run package check, then publish.
# Usage: scripts/release.sh [-y]   (-y skips the confirmation prompt)
set -euo pipefail

cd "$(dirname "$0")/.."

VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)

echo "==> Quality gate"
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test

echo "==> Dry-run package check"
cargo publish --dry-run

if [[ "${1:-}" != "-y" ]]; then
    read -r -p "Publish millis ${VERSION} to crates.io? [y/N] " ans
    [[ "${ans}" == [yY]* ]] || { echo "Aborted."; exit 1; }
fi

echo "==> Publishing millis ${VERSION}"
cargo publish
echo "==> Done: https://crates.io/crates/millis/${VERSION}"
