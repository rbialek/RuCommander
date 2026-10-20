# RuCommander

A Rust terminal file manager inspired by Norton Commander. This is an early, macOS-first, two-pane starter; the architecture and roadmap target cross-platform support.

## Run

Install a current stable Rust toolchain, then run in an interactive terminal:

```sh
cargo run
```

Both panes start in the current working directory. Navigation does not modify files.

| Key | Action |
| --- | --- |
| `Tab` | Switch active pane |
| `↑` / `↓` or `k` / `j` | Move selection |
| `Enter` / `→` / `l` | Enter selected directory |
| `Backspace` / `←` / `h` | Go to parent directory |
| `r` | Refresh active pane |
| `q` / `Esc` | Quit |

Directories appear before files; symlinks are displayed with `@` and are not traversed yet. For scope, milestones, and platform plans, see [the PRD](spec/prd.md).

## Development

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

Contributions and issues are welcome. Licensed under MIT.
