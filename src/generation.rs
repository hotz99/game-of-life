use rand::{thread_rng, Rng};
use tui::text::Spans;

pub type Gen = Vec<Vec<Cell>>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

fn get_alive(row: usize, col: usize, gen: &Gen) -> usize {
    let mut count = 0;
    
    // gets adjacent rows/cols while handling edge cases
    for adj_row in (row.saturating_sub(1))..=(row + 1).min(gen.len() - 1) {
        for adj_col in (col.saturating_sub(1))..=(col + 1).min(gen[0].len() - 1) {
            if !(adj_row == row && adj_col == col) && gen[adj_row][adj_col] == Cell::Alive {
                count += 1;
            }
        }
    }
    
    count
}

pub fn init_gen() -> Gen {
    let cells = vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Alive];
    let rows = 8;
    let cols = 15;
    let mut gen = Gen::new();

    for _ in 0..rows {
        let mut row: Vec<Cell> = Vec::new();
        for _ in 0..cols {
            let rand = thread_rng().gen_range(0..10);
            row.push(cells[rand % 5]);
        }
        gen.push(row);
    }

    gen
}

pub fn next_gen(gen: &Gen) -> Gen {
    let rows = gen.len();
    let cols = gen[0].len();
    let mut next_gen = Gen::new();

    // all dead grid
    for _ in 0..rows {
        let mut col = Vec::new();
        
        for _ in 0..cols {
            col.push(Cell::Dead);
        }

        next_gen.push(col);
    }

    for row in 0..rows {
        for col in 0..cols {
            let alive = get_alive(row, col, &gen);

            match gen[row][col] {
                Cell::Alive => {
                    if alive == 2 || alive == 3 {
                        next_gen[row][col] = Cell::Alive;
                    } else {
                        next_gen[row][col] = Cell::Dead;
                    }
                }
                Cell::Dead => {
                    if alive == 3 {
                        next_gen[row][col] = Cell::Alive;
                    }
                }
            }
        }
    }

    next_gen
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

pub fn gen_from_file(s: &String) -> Gen {
    let mut gen = Gen::new();

    for line in s.lines() {
        let mut row = Vec::new();

        for ch in line.chars() {
            if ch == '.' {
                row.push(Cell::Dead);
            } else {
                row.push(Cell::Alive);
            }
        }
        gen.push(row);
    }

    gen
}