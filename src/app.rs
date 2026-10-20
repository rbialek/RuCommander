use std::{
    cmp::Ordering,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::keymap::Action;

pub struct Entry {
    pub name: String,
    pub is_dir: bool,
    pub is_symlink: bool,
}

pub struct Pane {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
    pub selected: usize,
}

impl Pane {
    fn open(path: PathBuf) -> Result<Self> {
        let mut pane = Self {
            path,
            entries: Vec::new(),
            selected: 0,
        };
        pane.refresh()?;
        Ok(pane)
    }

    fn refresh(&mut self) -> Result<()> {
        let mut entries = Vec::new();
        for item in fs::read_dir(&self.path)
            .with_context(|| format!("Cannot read {}", self.path.display()))?
        {
            let item = item?;
            let kind = item.file_type()?;
            entries.push(Entry {
                name: item.file_name().to_string_lossy().into_owned(),
                is_dir: kind.is_dir(),
                is_symlink: kind.is_symlink(),
            });
        }

        entries.sort_by(|a, b| match b.is_dir.cmp(&a.is_dir) {
            Ordering::Equal => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            ordering => ordering,
        });
        if self.path.parent().is_some() {
            entries.insert(
                0,
                Entry {
                    name: "..".into(),
                    is_dir: true,
                    is_symlink: false,
                },
            );
        }

        self.entries = entries;
        self.selected = self.selected.min(self.entries.len().saturating_sub(1));
        Ok(())
    }

    fn change_dir(&mut self, path: &Path) -> Result<()> {
        let next = Self::open(path.to_path_buf())?;
        *self = next;
        Ok(())
    }

    fn parent(&mut self) -> Result<()> {
        if let Some(parent) = self.path.parent().map(Path::to_path_buf) {
            self.change_dir(&parent)?;
        }
        Ok(())
    }

    fn enter(&mut self) -> Result<()> {
        if let Some(entry) = self.entries.get(self.selected) {
            if entry.name == ".." {
                return self.parent();
            }
            if entry.is_dir {
                let path = self.path.join(&entry.name);
                return self.change_dir(&path);
            }
        }
        Ok(())
    }
}

pub struct App {
    pub panes: [Pane; 2],
    pub active: usize,
}

impl App {
    pub fn new() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        Ok(Self {
            panes: [Pane::open(cwd.clone())?, Pane::open(cwd)?],
            active: 0,
        })
    }

    pub fn handle_action(&mut self, action: Action) -> Result<bool> {
        let pane = &mut self.panes[self.active];
        match action {
            Action::Quit => return Ok(true),
            Action::SwitchPane => self.active = 1 - self.active,
            Action::Up => {
                pane.selected = pane.selected.saturating_sub(1);
            }
            Action::Down => {
                pane.selected = (pane.selected + 1).min(pane.entries.len().saturating_sub(1));
            }
            Action::Open => pane.enter()?,
            Action::Parent => pane.parent()?,
            Action::Refresh => pane.refresh()?,
        }
        Ok(false)
    }
}
