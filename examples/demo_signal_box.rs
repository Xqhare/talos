use talos::{Talos, LayoutBuilder, input::{Event, KeyEvent, KeyCode}, render::{Colour, Normal, Style}, layout::{Rect, Direction, Constraint}, widgets::{Block, Text, SignalBox, SignalBoxState, traits::Widget}};

// A simple helper to make the loop cleaner
use std::thread;
use std::time::Duration;

fn main() -> Result<(), talos::TalosError> {
    // 1. Initialize Talos
    let mut talos = Talos::builder()
        .build()?;

    let mut running = true;

    let mut signal_box_state = SignalBoxState {
        signal: true
    };

    while running {
        // 2. Handle Input
        if let Some(events) = talos.poll_input()? {
            for event in events {
                match event {
                    // Quit on 'q' or Esc
                    Event::KeyEvent(KeyEvent { code: KeyCode::Char('q'), .. }) |
                    Event::KeyEvent(KeyEvent { code: KeyCode::Esc, .. }) => {
                        running = false;
                    }
                    Event::KeyEvent(KeyEvent { code: KeyCode::Char('c'), .. }) => {
                        signal_box_state.signal = !signal_box_state.signal;
                    }
                    _ => {}
                }
            }
        }

        // 3. Render Frame
        talos.begin_frame();
        let (canvas, codex) = talos.render_ctx();

        let chunks = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Percentage(50))
            .add_constraint(Constraint::Percentage(50))
            .build()
            .split(canvas.size_rect());

        let left_style = Style::builder()
            .set_bg(Colour::Normal(Normal::White))
            .set_fg(Colour::Normal(Normal::Blue))
            .set_bold(true)
            .build();

        let right_style = Style::builder()
            .set_bg(Colour::Normal(Normal::Blue))
            .set_fg(Colour::Normal(Normal::White))
            .build();

        let mut left_block: Block = Block::new()
            .title("Left Block", codex, false)
            .with_beautify_border_breaks()
            .with_fat_border()
            .with_bg_fill();

        left_block.style(left_style);
        left_block.render(canvas, chunks[0], codex);
        let inner_left = left_block.inner(chunks[0]);

        let left_chunks = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Percentage(50))
            .add_constraint(Constraint::Percentage(50))
            .build()
            .split(inner_left);

        let mut signal_box = SignalBox::new().with_state(&mut signal_box_state);
        signal_box.style(left_style);
        signal_box.render(canvas, left_chunks[1], codex);

        let mut text = Text::new("Press 'c' to toggle signal!", codex);
        text.style(left_style);
        text.render(canvas, left_chunks[0], codex);


        let mut right_block: Block = Block::new()
            .title("Right Block", codex, false)
            .with_beautify_border_breaks()
            .with_fat_border()
            .with_bg_fill();

        right_block.style(right_style);
        right_block.render(canvas, chunks[1], codex);

        // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
