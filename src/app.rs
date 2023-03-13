use crossterm::event::{Event, KeyCode, self};
use rand::Rng;
use tui::{ backend::Backend, Terminal, text::{Spans, Span}, widgets::{Block, Borders, Paragraph}, style::{Style, Color, Modifier}, layout::Alignment};
use std::{io::{Result}, thread::sleep, time::Duration, fs};

use crate::generation::*;

fn render_spans<B: Backend>(terminal: &mut Terminal<B>, spans: &Vec<Spans>) -> Result<()> {
    let create_block = |title| {
        Block::default()
        .title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title_alignment(Alignment::Center)
    };

    let paragraph = Paragraph::new(spans.clone())
        .style(Style::default())
        .block(create_block("  Jogo da Vida  "))
        .alignment(Alignment::Center);

    let res = terminal.draw(|f| {
        f.render_widget(paragraph, f.size())
    });

    match res {
        Err(e) => return Err(e),
        Ok(_) => Ok(())
    }
}

enum Input {
    Quit,
    NewPattern,
    None
}

fn read_input() -> Input {
    if (crossterm::event::poll(Duration::from_millis(1))).unwrap() {
        if let Event::Key(k) = event::read().unwrap() {
            match k.code {
                KeyCode::Char('q') => return Input::Quit,
                KeyCode::Char('n') => return Input::NewPattern,
                _ => Input::None
            };
        }
    }

    Input::None
}

fn rand_pattern() -> Result<String> {
    let i = rand::thread_rng().gen_range(1..=513);

    let file = format!("presets/pattern{}.txt", i);
    fs::read_to_string(file)
}

pub fn init<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut curr_gen = gen_from_file(&rand_pattern()?);
    let init_spans = &gen_to_spans(&curr_gen);

    let res = render_spans(terminal, init_spans);

    if let Err(err) = res {
        println!("{:?}", err)
    }

    loop {
        let mut next_gen = next_gen(&curr_gen);
        let spans = &gen_to_spans(&next_gen);

        render_spans(terminal, spans)?;

        sleep(Duration::from_millis(32));
        
        match read_input() {
            Input::Quit => break,
            Input::NewPattern => {
                next_gen = gen_from_file(&rand_pattern()?)
            },
            _ => {}
        }

        curr_gen = next_gen;
    }

    Ok(())
}