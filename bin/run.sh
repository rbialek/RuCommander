#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

if command -v rustup >/dev/null 2>&1; then
    export RUSTC="$(rustup which rustc --toolchain stable)"
    rustup run stable cargo run -- "$@"
else
    cargo run -- "$@"
fi
