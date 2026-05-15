use talos::{
    LayoutBuilder, Talos,
    input::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Direction},
    render::{Colour, Normal, Style},
    widgets::{
        Block, Text,
        stateful::{InnerBorder, Table, TableState},
        traits::Widget,
    },
};

// A simple helper to make the loop cleaner
use std::thread;
use std::time::Duration;

fn main() -> Result<(), talos::TalosError> {
    // 1. Initialize Talos
    let mut talos = Talos::builder().build()?;

    let mut running = true;

    let mut table_state: TableState = TableState {
        x_offset: 0,
        y_offset: 0,
        max_rows: Some(5),
        max_columns: Some(5),
    };

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
                    Event::KeyEvent(KeyEvent {
                        code: KeyCode::Up, ..
                    }) => {
                        table_state.y_offset = table_state.y_offset.saturating_sub(1);
                    }
                    Event::KeyEvent(KeyEvent {
                        code: KeyCode::Down,
                        ..
                    }) => {
                        table_state.y_offset = table_state.y_offset.saturating_add(1);
                    }
                    Event::KeyEvent(KeyEvent {
                        code: KeyCode::Left,
                        ..
                    }) => {
                        table_state.x_offset = table_state.x_offset.saturating_sub(1);
                    }
                    Event::KeyEvent(KeyEvent {
                        code: KeyCode::Right,
                        ..
                    }) => {
                        table_state.x_offset = table_state.x_offset.saturating_add(1);
                    }
                    _ => {}
                }
            }
        }

        // 3. Render Frame
        talos.begin_frame();
        let mut ctx = talos.render_ctx();
        let size = ctx.canvas.size_rect();

        let mut rows = Vec::new();
        for r in 1..=10 {
            let mut row = Vec::new();
            for c in 1..=10 {
                row.push(Box::new(Text::new(format!("R{} C{}", r, c), ctx.codex)) as Box<dyn Widget>);
            }
            rows.push(row);
        }

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
        let mut head = Block::new().title("Header", ctx.codex, false);
        head.render(&mut ctx, chunks[0]);

        let head_inner = head.inner(chunks[0]);

        Text::new("To move the table, use the arrow keys!", ctx.codex)
            .render(&mut ctx, head_inner);

        // Content
        let mut content_block = Block::new().title("Content", ctx.codex, true);

        content_block.render(&mut ctx, chunks[1]);

        let content_size = content_block.inner(chunks[1]);

        let table_style = Style::builder()
            .set_fg(Colour::Normal(Normal::Yellow))
            .set_bg(Colour::Normal(Normal::Red))
            .build();

        let table_alternate_style = Style::builder()
            .set_fg(Colour::Normal(Normal::White))
            .set_bg(Colour::Normal(Normal::Blue))
            .build();

        let mut table = Table::new(&mut table_state)
            .with_rows(rows)
            .with_alternate_style(table_alternate_style)
            .alternate_colour_vertically()
            .alternate_colour_horizontally()
            .draw_inner_border(InnerBorder::All)
            .draw_outer_border();

        table.style(table_style);
        table.render(&mut ctx, content_size);

        // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
