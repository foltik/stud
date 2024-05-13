#!/usr/bin/env bash
# Usage: bump.sh [major|minor|patch]
BUMP=${1:-patch}
cargo set-version --workspace --bump $BUMP
