mod app;
mod keymap;
mod ui;

use std::{
    io::{self, Stdout},
    path::PathBuf,
};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;
use keymap::Keymap;

struct TerminalSession {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalSession {
    fn start() -> Result<Self> {
        enable_raw_mode()?;
        if let Err(error) = execute!(io::stdout(), EnterAlternateScreen) {
            disable_raw_mode()?;
            return Err(error.into());
        }

        match Terminal::new(CrosstermBackend::new(io::stdout())) {
            Ok(terminal) => Ok(Self { terminal }),
            Err(error) => {
                execute!(io::stdout(), LeaveAlternateScreen)?;
                disable_raw_mode()?;
                Err(error.into())
            }
        }
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let config_path = match args.next() {
        Some(flag) if flag == "--keymap" => {
            Some(PathBuf::from(args.next().ok_or_else(|| {
                anyhow::anyhow!("--keymap requires a path")
            })?))
        }
        Some(_) => anyhow::bail!("usage: rucommander [--keymap PATH]"),
        None => None,
    };
    if args.next().is_some() {
        anyhow::bail!("usage: rucommander [--keymap PATH]");
    }
    let keymap = Keymap::load(config_path.as_deref())?;
    let mut app = App::new()?;
    let mut session = TerminalSession::start()?;

    loop {
        session
            .terminal
            .draw(|frame| ui::render(frame, &mut app, &keymap))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if let Some(action) = keymap.action(key) {
                    if app.handle_action(action)? {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
