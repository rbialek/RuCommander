#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

if [[ -n "$(git status --porcelain)" ]]; then
    echo "Commit or discard local changes before publishing." >&2
    exit 1
fi

"$repo_root/bin/build.sh"

if command -v rustup >/dev/null 2>&1; then
    export RUSTC="$(rustup which rustc --toolchain stable)"
    rustup run stable cargo test
else
    cargo test
fi

# Do not publish a different tree from the one that passed the checks.
if [[ -n "$(git status --porcelain)" ]]; then
    echo "Working tree changed during verification; commit and retry." >&2
    exit 1
fi

git push
