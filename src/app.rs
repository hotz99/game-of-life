use crossterm::event::{Event, KeyCode, self};
use tui::{ backend::Backend, Terminal, text::{Spans, Span}, widgets::{Block, Borders, Paragraph}, style::{Style, Color, Modifier}, layout::Alignment};
use std::{io::Result, thread::sleep, time::Duration};

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

fn has_user_halted() -> bool {
    if (crossterm::event::poll(Duration::from_millis(1))).unwrap() {
        if let Event::Key(k) = event::read().unwrap() {
            match k.code {
                KeyCode::Char('q') => {
                    return true
                }, 
                _ => {}
            }
        }
    }

    false
}

pub fn init<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut curr_gen = init_gen();
    let init_spans = &gen_to_spans(&curr_gen);

    let res = render_spans(terminal, init_spans);

    if let Err(err) = res {
        println!("{:?}", err)
    }

    loop {
        let next_gen = next_gen(&curr_gen);
        let spans = &gen_to_spans(&next_gen);

        render_spans(terminal, spans)?;

        sleep(Duration::from_millis(32));
        
        if has_user_halted() { break }

        curr_gen = next_gen;
    }

    Ok(())
}