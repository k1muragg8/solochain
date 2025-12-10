mod app;
mod data;
mod ui;

use std::io;
use color_eyre::eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use crate::app::{App, CurrentScreen};

fn main() -> Result<()> {
    color_eyre::install()?;
    // Use the RAII guard to ensure restoration on panic or return
    let mut tui = Tui::new()?;

    let mut app = App::new();
    run_app(&mut tui.terminal, &mut app)?;

    Ok(())
}

struct Tui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Tui {
    fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                // Global exit keys
                match key.code {
                     KeyCode::Esc => {
                        // Logic to go back or exit
                        match app.current_screen {
                            CurrentScreen::Menu => {
                                app.should_exit = true;
                            }
                            CurrentScreen::Quiz => {
                                app.current_screen = CurrentScreen::Menu;
                                app.quiz_mode = None;
                            }
                        }
                    }
                    KeyCode::Char('q') => {
                         if app.current_screen == CurrentScreen::Menu {
                             app.should_exit = true;
                         } else {
                             // In quiz, q is just a character input
                             if let CurrentScreen::Quiz = app.current_screen {
                                 app.quiz_state.user_input.push('q');
                             }
                         }
                    }
                    _ => {}
                }

                if app.should_exit {
                    return Ok(());
                }

                match app.current_screen {
                    CurrentScreen::Menu => {
                         match key.code {
                            KeyCode::Up => app.menu_state.previous(),
                            KeyCode::Down => app.menu_state.next(),
                            KeyCode::Enter => {
                                let mode = app.menu_state.current_selection();
                                app.start_quiz(mode);
                            }
                             _ => {}
                         }
                    }
                    CurrentScreen::Quiz => {
                        // If showing feedback, any key goes to next question
                        if app.quiz_state.feedback.is_some() {
                             match key.code {
                                 _ => app.next_question(), // Any key for next question
                             }
                        } else {
                            match key.code {
                                KeyCode::Char(c) => {
                                    app.quiz_state.user_input.push(c);
                                }
                                KeyCode::Backspace => {
                                    app.quiz_state.user_input.pop();
                                }
                                KeyCode::Enter => {
                                    app.submit_answer();
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}
