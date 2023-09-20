use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // 1) prepare
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2) run
    let res = run(&mut terminal);

    // 3) quit
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    // Background
    let background = Block::default()
        .title("foo")
        .title_alignment(Alignment::Center);
    f.render_widget(background, size);

    // 3 principle vertical parts
    let vert_parts = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Top 4 blocks
    let top_horz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(vert_parts[0]);

    let block = Block::default()
        /*
        .title(vec![
            Span::styled("bar", Style::default().fg(Color::Yellow)),
        ])
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::Green))
        */
        .borders(Borders::ALL);

    let text = vec![Spans::from("foo bar")];
    let build_block = |_title| {
        Block::default().borders(Borders::ALL)
        /*
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
        */
    };

    let paragraph = Paragraph::new(text.clone())
        //.style(Style::default().bg(Color::White).fg(Color::Black))
        .block(build_block("baz"))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, top_horz[0]);

    (1..4)
        .into_iter()
        .for_each(|i| f.render_widget(block.clone(), top_horz[i]));

    // Middle 3 horizontal subparts
    let mid_horz_parts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(vert_parts[1]);

    // Middle left 2 blocks
    let mid_left_parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(mid_horz_parts[0]);

    (0..2)
        .into_iter()
        .for_each(|i| f.render_widget(block.clone(), mid_left_parts[i]));

    // Center big block
    f.render_widget(block.clone(), mid_horz_parts[1]);

    // Middle left 2 blocks
    let mid_right_parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(mid_horz_parts[2]);
    (0..2)
        .into_iter()
        .for_each(|i| f.render_widget(block.clone(), mid_right_parts[i]));

    // Botton 4 blocks
    let bottom_horz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(vert_parts[2]);

    (0..4)
        .into_iter()
        .for_each(|i| f.render_widget(block.clone(), bottom_horz[i]));
}
