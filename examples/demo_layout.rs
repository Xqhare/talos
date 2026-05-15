use talos::{
    LayoutBuilder, Talos,
    input::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Direction},
    widgets::{Block, Number, traits::Widget},
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
        let mut ctx = talos.render_ctx();
        let size = ctx.canvas.size_rect();

        // 1. Create a Layout
        // "Split the screen vertically. Top 16% for header, rest (Min 0) for content"
        let chunks = LayoutBuilder::new()
            .direction(Direction::Vertical)
            .add_constraint(Constraint::Percentage(16))
            .add_constraint(Constraint::Min(20))
            .build()
            .split(size);

        // 2. Draw
        // Header
        Block::new()
            .title("Header", ctx.codex, false)
            .render(&mut ctx, chunks[0]);

        // Content
        let mut content_block = Block::new().title("Content", ctx.codex, true);
        content_block.render(&mut ctx, chunks[1]);
        let inner_content = content_block.inner(chunks[1]);

        let inner_chunks = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Percentage(33))
            .add_constraint(Constraint::Percentage(33))
            .add_constraint(Constraint::Percentage(33))
            .build()
            .split(inner_content);

        let u8 = 1;
        let i8 = -2;
        let float = 3.5;
        Number::new(&u8, ctx.codex).render(&mut ctx, inner_chunks[0]);
        Number::new(&i8, ctx.codex).render(&mut ctx, inner_chunks[1]);
        Number::new(&float, ctx.codex).render(&mut ctx, inner_chunks[2]);
        // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
