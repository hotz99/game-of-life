use rand::{thread_rng, Rng};
use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Rect, Alignment};
use tui::style::{Style, Color, Modifier};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

pub type Gen = Vec<Vec<Cell>>;

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Alive,
    Dead,
}

pub fn new_gen() -> Gen {
    let cells = vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Alive];
    let cols = 80;
    let rows = 45;
    let mut grid: Vec<Vec<Cell>> = Vec::new();

    for _ in 0..rows {
        let mut row: Vec<Cell> = Vec::new();
        for _ in 0..cols {
            let rand = thread_rng().gen_range(0..10);
            row.push(cells[rand % 5]);
        }
        grid.push(row);
    }

    grid
}

pub fn gen_to_spans(gen: &Gen) -> Vec<Spans> {
    let mut spans = Vec::new();
    // 🟥 🟦 🟨 🟪 🟧 🟩 🟫

    for row in 0..gen.len() {
        let mut txt = String::new();

        // gen[0] and not gen[row] because all rows have the same length
        for col in 0..gen[0].len() {
            match gen[row][col] {
                Cell::Alive => txt.push_str("⬜"),
                Cell::Dead => txt.push_str("  ")
            }
        }
        spans.push(Spans::from(txt));
    }

    spans
}

pub fn render_frame<B: Backend>(f: &mut Frame<B>, area: Rect, spans: &Vec<Spans>) {
    let create_block = |title| {
        Block::default()
        .title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::White).add_modifier(Modifier::BOLD))
        .title_alignment(Alignment::Center)
    };

    let paragraph = Paragraph::new(spans.clone())
        .style(Style::default().bg(Color::Black).fg(Color::Blue))
        .block(create_block("Jogo da Vida"))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}