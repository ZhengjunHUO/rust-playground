use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use test_macro::vec_string;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use tui_test::structs::Palais;

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

    let palais = Palais {
        id: 0,
        name: "命宫".to_string(),
        gz_name: "戊寅".to_string(),
        daxian: "5-14".to_string(),
        xiaoxian: String::new(),
        stars_a: vec_string!("武曲地", "天相庙"),
        stars_b: vec_string!("天刑", "蜚廉", "天厨"),
        stars_c: vec_string!("病", "飞廉", "白虎", "指背"),
        oppo: 0,
        tri: (0, 0),
    };

    //let text = vec![Spans::from("foo bar")];
    let mut text_1: Vec<_> = palais
        .stars_a
        .clone()
        .into_iter()
        .map(|s| Spans::from(Span::styled(s, Style::default().fg(Color::Red))))
        .collect();
    let text_2: Vec<_> = palais
        .stars_b
        .clone()
        .into_iter()
        .map(|s| Spans::from(Span::styled(s, Style::default().fg(Color::Yellow))))
        .collect();
    text_1.extend(text_2);

    let text_3: Vec<_> = palais
        .stars_c
        .clone()
        .into_iter()
        .map(|s| {
            Spans::from(Span::styled(
                format!("                                      {}", s),
                Style::default().fg(Color::White),
            ))
        })
        .collect();

    let build_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            //.style(Style::default().bg(Color::White).fg(Color::Blue))
            .style(Style::default().fg(Color::Blue))
            .title_alignment(Alignment::Center)
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let paragraph = Paragraph::new(text_1.clone())
        //.style(Style::default().bg(Color::White).fg(Color::Black))
        .block(build_block(&palais.name))
        .alignment(Alignment::Left);

    (0..4).for_each(|i| f.render_widget(block.clone(), top_horz[i]));

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

    (0..2).for_each(|i| f.render_widget(block.clone(), mid_left_parts[i]));

    // Center big block
    f.render_widget(block.clone(), mid_horz_parts[1]);

    // Middle left 2 blocks
    let mid_right_parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(mid_horz_parts[2]);
    (0..2).for_each(|i| f.render_widget(block.clone(), mid_right_parts[i]));

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

    let paragraph_1 = Paragraph::new(text_3.clone())
        //.style(Style::default().bg(Color::White).fg(Color::Black))
        .block(build_block(&palais.name))
        .alignment(Alignment::Left);
    f.render_widget(paragraph_1, bottom_horz[0]);

    f.render_widget(paragraph, bottom_horz[0]);

    (1..4).for_each(|i| f.render_widget(block.clone(), bottom_horz[i]));
}
