use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState},
    Frame,
};

use crate::app::{App, CurrentScreen, Feedback};

pub fn ui(f: &mut Frame, app: &mut App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(f.area()); // Changed size() to area() for newer ratatui

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Line::from(vec![
        Span::styled("Kana TUI Practice", Style::default().fg(Color::Cyan).bold()),
    ]))
    .block(title_block)
    .alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);

    match app.current_screen {
        CurrentScreen::Menu => render_menu(f, app, chunks[1]),
        CurrentScreen::Quiz => render_quiz(f, app, chunks[1]),
    }

    let footer_text = match app.current_screen {
        CurrentScreen::Menu => "Use Arrow keys to select, Enter to confirm, q/Esc to exit",
        CurrentScreen::Quiz => "Type answer + Enter. Esc/q to return to menu.",
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, chunks[2]);
}

fn render_menu(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let constraints = [
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ];
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    let center_area = chunks[1];

    // Vertical center
    let v_constraints = [
        Constraint::Percentage(30),
        Constraint::Length(10), // Menu height
        Constraint::Percentage(30),
    ];
     let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(v_constraints)
        .split(center_area);

    let menu_area = v_chunks[1];

    let items: Vec<ListItem> = app
        .menu_state
        .items
        .iter()
        .map(|mode| ListItem::new(format!("  {}  ", mode)))
        .collect();

    // We need to maintain a ListState to show selection
    let mut state = ListState::default();
    state.select(Some(app.menu_state.selected_index));

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Select Mode"))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, menu_area, &mut state);
}

fn render_quiz(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20), // Score/Stats
            Constraint::Percentage(40), // Question
            Constraint::Percentage(20), // Input
            Constraint::Percentage(20), // Feedback
        ])
        .split(area);

    // Stats
    let stats_text = vec![
        Line::from(vec![
            Span::raw("Score: "),
            Span::styled(format!("{}/{}", app.quiz_state.score, app.quiz_state.total_attempts), Style::default().fg(Color::Yellow)),
            Span::raw(" | Streak: "),
            Span::styled(format!("{}", app.quiz_state.streak), Style::default().fg(Color::Green)),
        ]),
    ];
    let stats = Paragraph::new(stats_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(stats, chunks[0]);

    // Question
    if let Some(kana) = &app.quiz_state.current_kana {
        let question_text = format!("{}", kana.character);
        let question = Paragraph::new(question_text)
            .style(Style::default().fg(Color::White).add_modifier(ratatui::style::Modifier::BOLD))
            .alignment(Alignment::Center)
            // Make it big? Ratatui doesn't scale fonts, but we can center it.
            .block(Block::default().borders(Borders::ALL).title("Kana"));
        f.render_widget(question, chunks[1]);
    }

    // Input
    let input_text = format!("Answer: {}", app.quiz_state.user_input);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Type Romaji"));
    f.render_widget(input, chunks[2]);

    // Feedback
    if let Some(feedback) = &app.quiz_state.feedback {
        let (text, color) = match feedback {
            Feedback::Correct => ("Correct!".to_string(), Color::Green),
            Feedback::Incorrect(ans) => (format!("Incorrect! Answer was: {}", ans), Color::Red),
        };

        let feedback_widget = Paragraph::new(text)
            .style(Style::default().fg(color).add_modifier(ratatui::style::Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(feedback_widget, chunks[3]);
    }
}
