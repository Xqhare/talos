use talos::{
    LayoutBuilder, Talos,
    input::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind, MouseButton},
    layout::{Constraint, Direction, Rect},
    render::{Colour, Normal, Style},
    widgets::{
        Block, Text,
        stateful::{Button, ButtonState, TextBox, TextBoxState},
        traits::Widget,
    },
};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), talos::TalosError> {
    let mut talos = Talos::builder().build()?;
    let mut running = true;

    let (_, codex) = talos.render_ctx();

    let mut button_state = ButtonState { clicked: false };
    let mut text_box_state = TextBoxState {
        active: false,
        cursor: Some(0),
        text: Text::new("", codex),
    };
    let mut input_text = String::new();

    let mut button_rect = Rect::default();

    while running {
        let mut text_changed = false;
        if let Some(events) = talos.poll_input()? {
            for event in events {
                match event {
                    Event::KeyEvent(KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    })
                    | Event::KeyEvent(KeyEvent {
                        code: KeyCode::Esc, ..
                    }) if !text_box_state.active => {
                        running = false;
                    }
                    Event::MouseEvent(MouseEvent {
                        kind: MouseEventKind::Down(MouseButton::Left),
                        column,
                        row,
                        ..
                    }) => {
                        if button_rect.contains(*column, *row) {
                            text_box_state.active = !text_box_state.active;
                            button_state.clicked = text_box_state.active;
                        }
                    }
                    Event::KeyEvent(KeyEvent { code, .. }) if text_box_state.active => {
                        match code {
                            KeyCode::Char(c) => {
                                input_text.push(*c);
                                text_changed = true;
                            }
                            KeyCode::Backspace => {
                                input_text.pop();
                                text_changed = true;
                            }
                            KeyCode::Enter => {
                                text_box_state.active = false;
                                button_state.clicked = false;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        if text_changed {
            text_box_state.text.set_content(&input_text, talos.codex());
            text_box_state.cursor = Some(input_text.len());
        }

        talos.begin_frame();
        let (canvas, codex) = talos.render_ctx();

        let root_rect = canvas.size_rect();
        
        let chunks = LayoutBuilder::new()
            .direction(Direction::Vertical)
            .add_constraint(Constraint::Length(3)) // Button
            .add_constraint(Constraint::Length(1)) // Spacer
            .add_constraint(Constraint::Length(3)) // Text box
            .add_constraint(Constraint::Min(0))    // Rest
            .build()
            .split(root_rect);

        button_rect = chunks[0];

        let button_style = if text_box_state.active {
            Style::builder()
                .set_bg(Colour::Normal(Normal::Green))
                .set_fg(Colour::Normal(Normal::Black))
                .build()
        } else {
            Style::builder()
                .set_bg(Colour::Normal(Normal::Blue))
                .set_fg(Colour::Normal(Normal::White))
                .build()
        };

        let mut button = Button::new(
            if text_box_state.active { "UNFOCUS" } else { "FOCUS TEXT BOX" },
            codex
        )
        .with_state(&mut button_state)
        .with_style(button_style);
        
        button.render(canvas, button_rect, codex);

        let mut text_box_block = Block::new()
            .title("Input", codex, false)
            .with_beautify_border_breaks()
            .with_bg_fill();
        
        let text_box_style = if text_box_state.active {
            Style::builder()
                .set_fg(Colour::Normal(Normal::Yellow))
                .build()
        } else {
            Style::default()
        };
        text_box_block.style(text_box_style);
        text_box_block.render(canvas, chunks[2], codex);

        let inner_text_box = text_box_block.inner(chunks[2]);
        let mut text_box = TextBox::new(&mut text_box_state);
        text_box.render(canvas, inner_text_box, codex);

        let mut help_text = Text::new(
            if text_box_state.active {
                "Type to enter text. Press Enter to unfocus."
            } else {
                "Click the button to focus. Press 'q' or Esc to quit."
            },
            codex
        );
        help_text.render(canvas, chunks[3], codex);

        talos.present()?;
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
