# RuCommander — Product Requirements Document

## Summary

RuCommander is an open-source, keyboard-first, two-pane terminal file manager inspired by Norton Commander, implemented in Rust. The first supported release targets macOS terminals. The design should keep filesystem and platform-specific behavior behind small interfaces so Linux and Windows can be supported in later releases.

Repository: **public** [github.com/rbialek/RuCommander](https://github.com/rbialek/RuCommander). License: MIT.

## Problem and audience

Developers and terminal users need a fast way to browse and manage files in two locations without repeatedly typing long shell commands. RuCommander is for users who prefer a visual, keyboard-driven workflow that still runs in an ordinary terminal.

## Goals

- Make it easy to navigate two independent directory panes and see the active location.
- Provide reliable keyboard-driven copy, move, rename, delete, and directory creation workflows in the first functional release.
- Preserve terminal state after normal exits and recoverable errors.
- Keep the app fast and usable on large local directories.
- Develop in a public GitHub repository with documented setup, issues, and contributions.

## Non-goals for the first functional release

- Pixel-perfect replication of Norton Commander or compatibility with its plugins.
- Remote filesystems (SFTP/FTP), archive browsing, and cloud storage.
- A built-in editor, mouse-first workflow, or background indexing.
- Official Linux or Windows support in the first macOS release.

## Users and core workflows

1. **Browse:** launch in the current directory; navigate either pane with arrows or Vim-style keys; switch panes with Tab; see directories before files.
2. **Transfer:** choose a source item in one pane and copy or move it to the directory shown in the other pane; confirm destination and conflicts before modifying data.
3. **Manage:** create a directory, rename an item, or delete selected items with a clear confirmation for destructive actions.
4. **Recover:** see useful errors for denied permissions, disappeared files, and conflicting paths without losing the session or terminal state.

## Functional requirements

| ID | Requirement | Priority | Status |
| --- | --- | --- | --- |
| F1 | Show two independent directory panes, active pane, current path, and selected entry. | P0 | Starter implemented |
| F2 | Navigate directories, parent directory, switch panes, refresh, and quit by keyboard. | P0 | Starter implemented |
| F3 | Sort directories first and show distinct file/directory/symlink indicators. | P0 | Starter implemented; symlinks are not traversed |
| F4 | Copy files and directories to the opposite pane with overwrite/skip/cancel conflict handling. | P0 | Planned |
| F5 | Move, rename, create directories, and delete with confirmation and meaningful errors. | P0 | Planned |
| F6 | Scroll long lists; support empty directories, narrow terminals, and non-UTF-8 names without misleading operations. | P0 | Planned |
| F7 | Show file details (size, modification time, permissions) and operation progress/status. | P1 | Planned |
| F8 | Search/filter entries, multi-select, configurable shortcuts, and hidden-file toggle. | P1 | Planned |

## UX and safety

- The active pane must remain obvious; key help should be visible in the UI.
- Before an operation changes data, show the source and destination and ask about collisions. Delete requires explicit confirmation; never silently overwrite files.
- Handle failures in the UI and keep both panes usable. Refresh affected panes after successful operations.
- File operations must use full filesystem paths rather than lossy display names; represent filenames with `OsString`/`PathBuf` internally when implementing mutations.
- Do not follow directory symlinks implicitly during recursive copy/delete; specify and test symlink behavior before enabling it.

## Technical approach

- Rust 2021, Ratatui for rendering, Crossterm for terminal input and raw/alternate-screen control.
- Split state/navigation (`src/app.rs`), rendering (`src/ui.rs`), and terminal lifecycle (`src/main.rs`). Introduce separate filesystem operation and platform modules as functionality grows.
- macOS is the initial development/test target; prefer portable standard-library filesystem APIs and isolate OS-specific features. Validate Linux and Windows builds and behavior before declaring support.
- Keep the Norton Commander–inspired shortcuts in an editable TOML keymap (`config/keymap.toml`) with optional per-user overrides; distinguish live navigation shortcuts from planned file-operation shortcuts until those operations are implemented.
- Add automated tests for navigation, sorting, conflicts, and recursive operations as those features are built; use CI for formatting, Clippy, and tests.

## Milestones and acceptance criteria

1. **Starter (current):** `cargo run` opens a responsive two-pane TUI on macOS; both panes can independently browse directories; quitting restores the terminal; README documents controls.
2. **macOS MVP:** F1–F6 work on local files; operations handle collisions and errors; tests cover critical file-management cases; install/build instructions are published; users can manage files without leaving the app.
3. **Cross-platform:** Linux and Windows builds run in CI and manual smoke tests pass in representative terminals; platform-specific behavior is documented and isolated.

## Success measures

- A user can navigate, copy, move, rename, create, and delete local files using only the keyboard in the macOS MVP.
- No silent overwrites; critical filesystem operation tests and CI checks pass.
- Terminal settings and screen are restored on normal quit and handled errors.

## Open decisions

- Final default shortcuts for file operations and multi-selection.
- Symlink policies for copy, move, and delete across platforms.
- Distribution channels (Homebrew, Cargo, binaries) after the macOS MVP.
