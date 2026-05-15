use talos::{
    LayoutBuilder, Talos,
    input::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Direction},
    render::{Colour, Normal, Style},
    widgets::{
        Block, Text,
        stateful::{List, ListState},
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

    let mut vertical_list_state: ListState = ListState {
        selected: Some(0),
        scroll_offset: 0,
    };
    let mut horizontal_list_state: ListState = ListState {
        selected: Some(0),
        scroll_offset: 0,
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
                        vertical_list_state.selected = vertical_list_state
                            .selected
                            .as_ref()
                            .and_then(|s| Some(s.saturating_sub(1)))
                    }
                    Event::KeyEvent(KeyEvent {
                        code: KeyCode::Down,
                        ..
                    }) => {
                        vertical_list_state.selected = vertical_list_state
                            .selected
                            .as_ref()
                            .and_then(|s| Some(s.saturating_add(1)))
                    }
                    Event::KeyEvent(KeyEvent {
                        code: KeyCode::Left,
                        ..
                    }) => {
                        horizontal_list_state.selected = horizontal_list_state
                            .selected
                            .as_ref()
                            .and_then(|s| Some(s.saturating_sub(1)))
                    }
                    Event::KeyEvent(KeyEvent {
                        code: KeyCode::Right,
                        ..
                    }) => {
                        horizontal_list_state.selected = horizontal_list_state
                            .selected
                            .as_ref()
                            .and_then(|s| Some(s.saturating_add(1)))
                    }
                    _ => {}
                }
            }
        }

        // 3. Render Frame
        talos.begin_frame();
        let mut ctx = talos.render_ctx();
        let size = ctx.canvas.size_rect();

        let mut large_list = Vec::new();
        for i in 1..=100 {
            large_list.push(Text::new(format!("Item {}", i), ctx.codex));
        }
        let mut large_list2 = Vec::new();
        for i in 1..=100 {
            large_list2.push(Text::new(format!("Item {}", i), ctx.codex));
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

        Text::new("To move the lists, use the arrow keys!", ctx.codex)
            .render(&mut ctx, head_inner);

        // Content
        let mut content_block = Block::new().title("Content", ctx.codex, true);

        content_block.render(&mut ctx, chunks[1]);

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

        let mut list = List::new(&mut horizontal_list_state)
            .with_selected_style(selected_style)
            .with_selected_symbol('→', ctx.codex)
            .horizontal();
        for item in large_list {
            list = list.add(item);
        }
        list.render(&mut ctx, inner_chunks[0]);

        let mut list2 = List::new(&mut vertical_list_state)
            .with_selected_style(selected_style)
            .with_selected_symbol('→', ctx.codex);
        for item in large_list2 {
            list2 = list2.add(item);
        }
        list2.render(&mut ctx, inner_chunks[1]);
        
        // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
