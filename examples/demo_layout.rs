use talos::{Talos, LayoutBuilder, input::{Event, KeyEvent, KeyCode}, layout::{Direction, Constraint}, widgets::{Block, traits::Widget}};

// A simple helper to make the loop cleaner
use std::thread;
use std::time::Duration;

fn main() -> Result<(), talos::TalosError> {
    // 1. Initialize Talos
    let mut talos = Talos::builder()
        .build()?;

    let mut running = true;

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
                    _ => {}
                }
            }
        }

        // 3. Render Frame
        talos.begin_frame();
        let (canvas, codex) = talos.render_ctx();
        let size = canvas.size_rect();

        // 1. Create a Layout
        // "Split the screen vertically. Top 10% for header, rest (Min 0) for content"
        let chunks = LayoutBuilder::new()
            .direction(Direction::Vertical)
            .add_constraint(Constraint::Percentage(10))
            .add_constraint(Constraint::Min(0))
            .build()
            .split(size); // You'll need a helper to get full screen rect

        // 2. Draw
        // Header
        Block::new().title("Header").render(canvas, chunks[0], codex);

        // Content
        Block::new().title("Content").render(canvas, chunks[1], codex);
        // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
