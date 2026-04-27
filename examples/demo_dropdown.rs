use std::thread;
use std::time::Duration;
use talos::{
    LayoutBuilder, Talos,
    input::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind},
    layout::{Constraint, Direction, Rect},
    render::{Colour, Normal, Style},
    widgets::{
        Text,
        stateful::{Dropdown, DropdownState},
        traits::Widget,
    },
};

fn main() -> Result<(), talos::TalosError> {
    let mut talos = Talos::builder().build()?;
    let mut running = true;

    let mut dropdown_state = DropdownState::default();
    let options = vec![
        "Option 1".to_string(),
        "Option 2".to_string(),
        "Option 3".to_string(),
        "Option 4".to_string(),
        "Option 5".to_string(),
    ];

    let mut dropdown_rect = Rect::default();

    while running {
        if let Some(events) = talos.poll_input()? {
            for event in events {
                match event {
                    Event::KeyEvent(KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    })
                    | Event::KeyEvent(KeyEvent {
                        code: KeyCode::Esc, ..
                    }) => {
                        running = false;
                    }
                    Event::MouseEvent(MouseEvent {
                        kind: MouseEventKind::Down(MouseButton::Left),
                        column,
                        row,
                        ..
                    }) => {
                        // Hit testing for the main button
                        if dropdown_rect.contains(*column, *row) {
                            dropdown_state.expanded = !dropdown_state.expanded;
                        } else if dropdown_state.expanded {
                            // Hit testing for the list
                            let list_height = options.len() as u16;
                            let list_rect = Rect::new(
                                dropdown_rect.x,
                                dropdown_rect.bottom(),
                                dropdown_rect.width,
                                list_height,
                            );

                            if list_rect.contains(*column, *row) {
                                let index = (row - list_rect.y) as usize;
                                if index < options.len() {
                                    dropdown_state.list_state.selected = Some(index);
                                    dropdown_state.expanded = false;
                                }
                            } else {
                                // Clicked outside, collapse
                                dropdown_state.expanded = false;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        talos.begin_frame();
        let (canvas, codex) = talos.render_ctx();

        let root_rect = canvas.size_rect();

        let chunks = LayoutBuilder::new()
            .direction(Direction::Vertical)
            .add_constraint(Constraint::Length(3)) // Dropdown
            .add_constraint(Constraint::Min(0)) // Rest
            .build()
            .split(root_rect);

        dropdown_rect = chunks[0];

        let mut items: Vec<Text> = options
            .iter()
            .map(|opt| Text::new(opt, codex))
            .collect();

        let selected_label = dropdown_state
            .list_state
            .selected
            .map(|i| options[i].clone());

        let mut dropdown = Dropdown::new(&mut dropdown_state, items.iter_mut())
            .with_placeholder("Pick an option...")
            .with_style(
                Style::builder()
                    .set_bg(Colour::Normal(Normal::Blue))
                    .set_fg(Colour::Normal(Normal::White))
                    .build(),
            );

        if let Some(label) = selected_label {
            dropdown = dropdown.with_label(label);
        }

        dropdown.render(canvas, dropdown_rect, codex);

        let mut help_text = Text::new(
            "Click the dropdown to expand. Click an option to select. 'q' to quit.",
            codex,
        );
        help_text.render(canvas, chunks[1], codex);

        talos.present()?;
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
