use tui::{layout::Rect, Frame, backend::Backend};
use std::io::Result;

use crate::generation::*;

pub fn init<B: Backend>(frame: &mut Frame<B>) -> Result<()>{
    let area = Rect::new(0, 0, 10, 10);
    let init_gen = &new_gen();
    let spans = &gen_to_spans(&init_gen);

    render_frame(frame, area, spans);

    Ok(())
}