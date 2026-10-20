#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

# Prefer the rustup-managed stable toolchain (which may not be first on PATH).
if command -v rustup >/dev/null 2>&1; then
    export RUSTC="$(rustup which rustc --toolchain stable)"
    rustup run stable cargo build "$@"
else
    cargo build "$@"
fi
