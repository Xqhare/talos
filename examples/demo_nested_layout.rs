use talos::{
    LayoutBuilder, Talos,
    input::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Direction},
    widgets::{Block, traits::Widget},
};

// A simple helper to make the loop cleaner
use std::thread;
use std::time::Duration;

fn main() -> Result<(), talos::TalosError> {
    // 1. Initialize Talos
    let mut talos = Talos::builder().build()?;

    let mut running = true;

    while running {
        // 2. Handle Input
        if let Some(events) = talos.poll_input()? {
            for event in events {
                match event {
                    // Quit on 'q' or Esc
                    Event::KeyEvent(KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    })
                    | Event::KeyEvent(KeyEvent {
                        code: KeyCode::Esc, ..
                    }) => {
                        running = false;
                    }
                    _ => {}
                }
            }
        }

        // 3. Render Frame
        talos.begin_frame();
        let (canvas, codex) = talos.render_ctx();
        let size = canvas.size_rect();

        // 1. Create a Layout
        // "Split the screen vertically. Top 16% for header, rest (Min 0) for content"
        let chunks = LayoutBuilder::new()
            .direction(Direction::Vertical)
            .add_constraint(Constraint::Percentage(16))
            .add_constraint(Constraint::Min(0))
            .add_constraint(Constraint::Percentage(8))
            .build()
            .split(size);

        let sub_chunks = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Percentage(50))
            .add_constraint(Constraint::Percentage(50))
            .build()
            .split(chunks[1]);

        // 2. Draw
        // Header
        Block::new()
            .title("Header", codex, true)
            .with_beautify_border_breaks()
            .render(canvas, chunks[0], codex);

        // Content
        Block::new()
            .title("Left Content Chunk", codex, false)
            .with_beautify_border_breaks()
            .render(canvas, sub_chunks[0], codex);
        Block::new()
            .title("Right Content Chunk", codex, false)
            .with_beautify_border_breaks()
            .render(canvas, sub_chunks[1], codex);

        // Footer
        Block::new()
            .title("Footer", codex, true)
            .render(canvas, chunks[2], codex);

        // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
