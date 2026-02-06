 use talos::{Talos, LayoutBuilder, render::{Colour, Normal, Style}, input::{Event, KeyEvent, KeyCode}, layout::{Direction, Constraint}, widgets::{Block, Text, stateful::{Table, TableState}, traits::Widget}};

// A simple helper to make the loop cleaner
use std::thread;
use std::time::Duration;

fn main() -> Result<(), talos::TalosError> {
    // 1. Initialize Talos
    let mut talos = Talos::builder()
        .build()?;

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
                    Event::KeyEvent(KeyEvent { code: KeyCode::Char('q'), .. }) |
                    Event::KeyEvent(KeyEvent { code: KeyCode::Esc, .. }) => {
                        running = false;
                    }
                    Event::KeyEvent(KeyEvent { code: KeyCode::Up, .. }) => {
                        table_state.y_offset = table_state.y_offset.saturating_sub(1);
                    }
                    Event::KeyEvent(KeyEvent { code: KeyCode::Down, .. }) => {
                        table_state.y_offset = table_state.y_offset.saturating_add(1);
                    }
                    Event::KeyEvent(KeyEvent { code: KeyCode::Left, .. }) => {
                        table_state.x_offset = table_state.x_offset.saturating_sub(1);
                    }
                    Event::KeyEvent(KeyEvent { code: KeyCode::Right, .. }) => {
                        table_state.x_offset = table_state.x_offset.saturating_add(1);
                    }
                    _ => {}
                }
            }
        }

        // 3. Render Frame
        talos.begin_frame();
        let (canvas, codex) = talos.render_ctx();
        let size = canvas.size_rect();


        let mut table_vec: Vec<Vec<Text>> = vec![
            vec![Text::new("Row 1, Column 1", &codex), Text::new("Column 2", &codex), Text::new("Column 3", &codex), Text::new("Column 4", &codex), Text::new("Column 5", &codex), Text::new("Column 6", &codex), Text::new("Column 7", &codex), Text::new("Column 8", &codex), Text::new("Column 9", &codex), Text::new("Column 10", &codex)],
            vec![Text::new("Row 2, Column 1", &codex), Text::new("Column 2", &codex), Text::new("Column 3", &codex), Text::new("Column 4", &codex), Text::new("Column 5", &codex), Text::new("Column 6", &codex), Text::new("Column 7", &codex), Text::new("Column 8", &codex), Text::new("Column 9", &codex), Text::new("Column 10", &codex)],
            vec![Text::new("Row 3, Column 1", &codex), Text::new("Column 2", &codex), Text::new("Column 3", &codex), Text::new("Column 4", &codex), Text::new("Column 5", &codex), Text::new("Column 6", &codex), Text::new("Column 7", &codex), Text::new("Column 8", &codex), Text::new("Column 9", &codex), Text::new("Column 10", &codex)],
            vec![Text::new("Row 4, Column 1", &codex), Text::new("Column 2", &codex), Text::new("Column 3", &codex), Text::new("Column 4", &codex), Text::new("Column 5", &codex), Text::new("Column 6", &codex), Text::new("Column 7", &codex), Text::new("Column 8", &codex), Text::new("Column 9", &codex), Text::new("Column 10", &codex)],
            vec![Text::new("Row 5, Column 1", &codex), Text::new("Column 2", &codex), Text::new("Column 3", &codex), Text::new("Column 4", &codex), Text::new("Column 5", &codex), Text::new("Column 6", &codex), Text::new("Column 7", &codex), Text::new("Column 8", &codex), Text::new("Column 9", &codex), Text::new("Column 10", &codex)],
            vec![Text::new("Row 6, Column 1", &codex), Text::new("Column 2", &codex), Text::new("Column 3", &codex), Text::new("Column 4", &codex), Text::new("Column 5", &codex), Text::new("Column 6", &codex), Text::new("Column 7", &codex), Text::new("Column 8", &codex), Text::new("Column 9", &codex), Text::new("Column 10", &codex)],
            vec![Text::new("Row 7, Column 1", &codex), Text::new("Column 2", &codex), Text::new("Column 3", &codex), Text::new("Column 4", &codex), Text::new("Column 5", &codex), Text::new("Column 6", &codex), Text::new("Column 7", &codex), Text::new("Column 8", &codex), Text::new("Column 9", &codex), Text::new("Column 10", &codex)],
            vec![Text::new("Row 8, Column 1", &codex), Text::new("Column 2", &codex), Text::new("Column 3", &codex), Text::new("Column 4", &codex), Text::new("Column 5", &codex), Text::new("Column 6", &codex), Text::new("Column 7", &codex), Text::new("Column 8", &codex), Text::new("Column 9", &codex), Text::new("Column 10", &codex)],
            vec![Text::new("Row 9, Column 1", &codex), Text::new("Column 2", &codex), Text::new("Column 3", &codex), Text::new("Column 4", &codex), Text::new("Column 5", &codex), Text::new("Column 6", &codex), Text::new("Column 7", &codex), Text::new("Column 8", &codex), Text::new("Column 9", &codex), Text::new("Column 10", &codex)],
            vec![Text::new("Row 10, Column 1", &codex), Text::new("Column 2", &codex), Text::new("Column 3", &codex), Text::new("Column 4", &codex), Text::new("Column 5", &codex), Text::new("Column 6", &codex), Text::new("Column 7", &codex), Text::new("Column 8", &codex), Text::new("Column 9", &codex), Text::new("Column 10", &codex)],
        ];
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
        let mut head = Block::new().title("Header", codex, false);
            head.render(canvas, chunks[0], codex);

        let head_inner = head.inner(chunks[0]);
        
        let _header_text = Text::new("To move the table, use the arrow keys!", &codex).render(canvas, head_inner, codex);

        // Content
        let mut content_block = Block::new().title("Content", codex, true);

        content_block.render(canvas, chunks[1], codex);

        let content_size = content_block.inner(chunks[1]);

        let table_style = Style::builder()
             .set_fg(Colour::Normal(Normal::Yellow))
             .set_bg(Colour::Normal(Normal::Red))
             .build();

        let table_alternate_style = Style::builder()
             .set_fg(Colour::Normal(Normal::White))
             .set_bg(Colour::Normal(Normal::Blue))
             .build();

        let mut table = Table::new()
            .with_state(&mut table_state)
            .with_rows(table_vec.iter_mut())
            .with_alternate_style(table_alternate_style)
            .alternate_colour_vertically()
            .alternate_colour_horizontally();

        table.style(table_style);
        table.render(canvas, content_size, codex);

        // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
