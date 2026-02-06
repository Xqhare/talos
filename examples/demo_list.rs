use talos::{Talos, LayoutBuilder, render::{Colour, Normal, Style}, input::{Event, KeyEvent, KeyCode}, layout::{Direction, Constraint}, widgets::{Block, Text, stateful::{List, ListState}, traits::Widget}};

// A simple helper to make the loop cleaner
use std::thread;
use std::time::Duration;

fn main() -> Result<(), talos::TalosError> {
    // 1. Initialize Talos
    let mut talos = Talos::builder()
        .build()?;

    let mut running = true;

    let mut vertical_list_state: ListState = ListState {
        selected: Some(0),
        scroll_offset: 0
    };
    let mut horizontal_list_state: ListState = ListState {
        selected: Some(0),
        scroll_offset: 0
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
                        vertical_list_state.selected = vertical_list_state.selected.as_ref().and_then(|s| Some(s.saturating_sub(1)))
                    }
                    Event::KeyEvent(KeyEvent { code: KeyCode::Down, .. }) => {
                        vertical_list_state.selected = vertical_list_state.selected.as_ref().and_then(|s| Some(s.saturating_add(1)))
                    }
                    Event::KeyEvent(KeyEvent { code: KeyCode::Left, .. }) => {
                        horizontal_list_state.selected = horizontal_list_state.selected.as_ref().and_then(|s| Some(s.saturating_sub(1)))
                    }
                    Event::KeyEvent(KeyEvent { code: KeyCode::Right, .. }) => {
                        horizontal_list_state.selected = horizontal_list_state.selected.as_ref().and_then(|s| Some(s.saturating_add(1)))
                    }
                    _ => {}
                }
            }
        }

        // 3. Render Frame
        talos.begin_frame();
        let (canvas, codex) = talos.render_ctx();
        let size = canvas.size_rect();

        let mut large_list: Vec<Text> = vec![
            Text::new("Item 1", &codex),
            Text::new("Item 2", &codex),
            Text::new("Item 3", &codex),
            Text::new("Item 4", &codex),
            Text::new("Item 5", &codex),
            Text::new("Item 6", &codex),
            Text::new("Item 7", &codex),
            Text::new("Item 8", &codex),
            Text::new("Item 9", &codex),
            Text::new("Item 10", &codex),
            Text::new("Item 11", &codex),
            Text::new("Item 12", &codex),
            Text::new("Item 13", &codex),
            Text::new("Item 14", &codex),
            Text::new("Item 15", &codex),
            Text::new("Item 16", &codex),
            Text::new("Item 17", &codex),
            Text::new("Item 18", &codex),
            Text::new("Item 19", &codex),
            Text::new("Item 20", &codex),
            Text::new("Item 31", &codex),
            Text::new("Item 32", &codex),
            Text::new("Item 33", &codex),
            Text::new("Item 34", &codex),
            Text::new("Item 35", &codex),
            Text::new("Item 36", &codex),
            Text::new("Item 37", &codex),
            Text::new("Item 38", &codex),
            Text::new("Item 39", &codex),
            Text::new("Item 40", &codex),
            Text::new("Item 51", &codex),
            Text::new("Item 52", &codex),
            Text::new("Item 53", &codex),
            Text::new("Item 54", &codex),
            Text::new("Item 55", &codex),
            Text::new("Item 56", &codex),
            Text::new("Item 57", &codex),
            Text::new("Item 58", &codex),
            Text::new("Item 59", &codex),
            Text::new("Item 60", &codex),
            Text::new("Item 70", &codex),
            Text::new("Item 71", &codex),
            Text::new("Item 72", &codex),
            Text::new("Item 73", &codex),
            Text::new("Item 74", &codex),
            Text::new("Item 75", &codex),
            Text::new("Item 76", &codex),
            Text::new("Item 77", &codex),
            Text::new("Item 78", &codex),
            Text::new("Item 79", &codex),
            Text::new("Item 80", &codex),
            Text::new("Item 81", &codex),
            Text::new("Item 82", &codex),
            Text::new("Item 83", &codex),
            Text::new("Item 84", &codex),
            Text::new("Item 85", &codex),
            Text::new("Item 86", &codex),
            Text::new("Item 87", &codex),
            Text::new("Item 88", &codex),
            Text::new("Item 89", &codex),
            Text::new("Item 90", &codex),
            Text::new("Item 91", &codex),
            Text::new("Item 92", &codex),
            Text::new("Item 93", &codex),
            Text::new("Item 94", &codex),
            Text::new("Item 95", &codex),
            Text::new("Item 96", &codex),
            Text::new("Item 97", &codex),
            Text::new("Item 98", &codex),
            Text::new("Item 99", &codex),
            Text::new("Item 100", &codex),
        ];
        let mut large_list2: Vec<Text> = vec![
            Text::new("Item 1", &codex),
            Text::new("Item 2", &codex),
            Text::new("Item 3", &codex),
            Text::new("Item 4", &codex),
            Text::new("Item 5", &codex),
            Text::new("Item 6", &codex),
            Text::new("Item 7", &codex),
            Text::new("Item 8", &codex),
            Text::new("Item 9", &codex),
            Text::new("Item 10", &codex),
            Text::new("Item 11", &codex),
            Text::new("Item 12", &codex),
            Text::new("Item 13", &codex),
            Text::new("Item 14", &codex),
            Text::new("Item 15", &codex),
            Text::new("Item 16", &codex),
            Text::new("Item 17", &codex),
            Text::new("Item 18", &codex),
            Text::new("Item 19", &codex),
            Text::new("Item 20", &codex),
            Text::new("Item 31", &codex),
            Text::new("Item 32", &codex),
            Text::new("Item 33", &codex),
            Text::new("Item 34", &codex),
            Text::new("Item 35", &codex),
            Text::new("Item 36", &codex),
            Text::new("Item 37", &codex),
            Text::new("Item 38", &codex),
            Text::new("Item 39", &codex),
            Text::new("Item 40", &codex),
            Text::new("Item 51", &codex),
            Text::new("Item 52", &codex),
            Text::new("Item 53", &codex),
            Text::new("Item 54", &codex),
            Text::new("Item 55", &codex),
            Text::new("Item 56", &codex),
            Text::new("Item 57", &codex),
            Text::new("Item 58", &codex),
            Text::new("Item 59", &codex),
            Text::new("Item 60", &codex),
            Text::new("Item 70", &codex),
            Text::new("Item 71", &codex),
            Text::new("Item 72", &codex),
            Text::new("Item 73", &codex),
            Text::new("Item 74", &codex),
            Text::new("Item 75", &codex),
            Text::new("Item 76", &codex),
            Text::new("Item 77", &codex),
            Text::new("Item 78", &codex),
            Text::new("Item 79", &codex),
            Text::new("Item 80", &codex),
            Text::new("Item 81", &codex),
            Text::new("Item 82", &codex),
            Text::new("Item 83", &codex),
            Text::new("Item 84", &codex),
            Text::new("Item 85", &codex),
            Text::new("Item 86", &codex),
            Text::new("Item 87", &codex),
            Text::new("Item 88", &codex),
            Text::new("Item 89", &codex),
            Text::new("Item 90", &codex),
            Text::new("Item 91", &codex),
            Text::new("Item 92", &codex),
            Text::new("Item 93", &codex),
            Text::new("Item 94", &codex),
            Text::new("Item 95", &codex),
            Text::new("Item 96", &codex),
            Text::new("Item 97", &codex),
            Text::new("Item 98", &codex),
            Text::new("Item 99", &codex),
            Text::new("Item 100", &codex),
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
        
        let _header_text = Text::new("To move the lists, use the arrow keys!", &codex).render(canvas, head_inner, codex);

        // Content
        let mut content_block = Block::new().title("Content", codex, true);

        content_block.render(canvas, chunks[1], codex);


        let content_size = content_block.inner(chunks[1]);

        let mut inner_chunks = LayoutBuilder::new()
            .direction(Direction::Vertical)
            .add_constraint(Constraint::Percentage(50))
            .add_constraint(Constraint::Percentage(50))
            .build()
            .split(content_size);

        inner_chunks[0].height = 1;

        let selected_style = Style::builder()
            .set_fg(Colour::Normal(Normal::Black))
            .set_bg(Colour::Normal(Normal::White))
            .build();

        let _list = List::new()
            .with_items(large_list.iter_mut())
            .with_selected_style(selected_style)
            .with_selected_symbol('→', codex)
            .with_state(&mut horizontal_list_state)
            .horizontal()
            .render(canvas, inner_chunks[0], codex);

        let _list2 = List::new()
            .with_items(large_list2.iter_mut())
            .with_selected_style(selected_style)
            .with_selected_symbol('→', codex)
            .with_state(&mut vertical_list_state)
            .render(canvas, inner_chunks[1], codex);
        // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
