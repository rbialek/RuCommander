# RuCommander

A Rust terminal file manager inspired by Norton Commander. This is an early, macOS-first, two-pane starter; the architecture and roadmap target cross-platform support.

## Run

Install a current stable Rust toolchain, then run in an interactive terminal:

```sh
cargo run
```

From the repository root, you can also use these scripts (they can be invoked by absolute path from elsewhere):

```sh
./bin/build.sh               # build (extra Cargo build flags are forwarded)
./bin/run.sh                 # run the TUI (app arguments are forwarded)
./bin/run.sh --keymap /path/to/keymap.toml
```

Both panes start in the current working directory. Navigation does not modify files.

| Key | Action |
| --- | --- |
| `Tab` | Switch active pane |
| `↑` / `↓` or `k` / `j` | Move selection |
| `Enter` / `→` / `l` | Enter selected directory |
| `Backspace` / `←` / `h` | Go to parent directory |
| `r` | Refresh active pane |
| `F10` / `q` / `Esc` | Quit |

Directories appear before files; symlinks are displayed with `@` and are not traversed yet. For scope, milestones, and platform plans, see [the PRD](spec/prd.md).

## Keymap

The editable [Norton Commander–inspired keymap](config/keymap.toml) is bundled with the app (edits to the bundled file require rebuilding). To customize an installed binary without rebuilding, copy it to `~/.config/rucommander/keymap.toml` (create the directory first) and change the arrays under `[bindings]`. The app loads that file on startup, replacing only the actions you specify; there is no automatic write to your home directory. Alternatively, use `cargo run -- --keymap /path/to/keymap.toml` to load an explicit file. An explicit missing or malformed file produces an error before the TUI opens. Duplicate active shortcuts are rejected.

The `[planned]` section records the classic F1–F9 actions (Help, User menu, View, Edit, Copy, Move, Make directory, Delete, Menu) plus selection and panel-drive keys; those commands are **not implemented yet** and the entries do not bind any keys. `F10` quits now. Terminals on macOS may intercept function keys or Alt combinations; configure the terminal to pass them through or customize the binding.

References: [Norton Commander history and original function-key layout](https://www.softpanorama.org/OFM/Paradigm/Ch03/norton_commander.shtml), [orthodox file-manager function keys](https://en.wikipedia.org/wiki/File_manager#Orthodox_file_managers), and [Norton Commander F3/F4 viewer/editor](https://en.wikipedia.org/wiki/Norton_Commander). This is a practical terminal adaptation, not an exact transcription of every DOS-version shortcut.

## Development

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

To publish committed changes, run `./bin/publish.sh`. It requires a clean working tree, builds and tests with the stable Rust toolchain, then pushes the current branch to its configured upstream. It stops without pushing if the build or tests fail. Commit any changes (including new files) before running it; the script does not stage or create commits.

Contributions and issues are welcome. Licensed under MIT.
