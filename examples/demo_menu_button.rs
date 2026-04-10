use std::thread;
use std::time::Duration;
use talos::{
    LayoutBuilder, Talos,
    input::{Event, MouseButton, MouseEvent, MouseEventKind},
    layout::{Constraint, Direction, Rect},
    render::{Colour, Normal, Style},
    widgets::{
        Text,
        stateful::{Button, ButtonState, MenuButton},
        traits::Widget,
    },
};

fn main() -> Result<(), talos::TalosError> {
    let mut talos = Talos::builder().build()?;
    let mut running = true;

    let (_, _codex) = talos.render_ctx();

    // The main button state
    let mut menu_open = ButtonState { clicked: false };

    // Track which menu item was last clicked
    let mut last_action = String::from("None");

    let mut menu_rect = Rect::default();
    let mut item_rects: Vec<Rect> = Vec::new();

    while running {
        if let Some(events) = talos.poll_input()? {
            for event in events {
                match event {
                    Event::MouseEvent(MouseEvent {
                        kind: MouseEventKind::Down(MouseButton::Left),
                        column,
                        row,
                        ..
                    }) => {
                        // Check if the main button was clicked
                        if menu_rect.contains(*column, *row) {
                            menu_open.clicked = !menu_open.clicked;
                        }
                        // If menu is open, check if any menu items were clicked
                        else if menu_open.clicked {
                            for (i, rect) in item_rects.iter().enumerate() {
                                if rect.contains(*column, *row) {
                                    last_action = format!("Item {}", i + 1);
                                    menu_open.clicked = false;
                                }
                            }
                        }
                    }
                    Event::KeyEvent(ev) => {
                        if ev.code == talos::input::KeyCode::Char('q')
                            || ev.code == talos::input::KeyCode::Esc
                        {
                            running = false;
                        }
                    }
                    _ => {}
                }
            }
        }

        talos.begin_frame();
        let (canvas, codex) = talos.render_ctx();

        let chunks = LayoutBuilder::new()
            .direction(Direction::Vertical)
            .add_constraint(Constraint::Length(3)) // Header
            .add_constraint(Constraint::Length(3)) // Menu Button Row
            .add_constraint(Constraint::Min(0)) // Content
            .build()
            .split(canvas.size_rect());

        let mut header =
            Text::new("MenuButton Demo - Click to open, 'q' to quit", codex).align_center();
        header.render(canvas, chunks[0], codex);

        // Center the menu button horizontally
        let button_row = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Percentage(40))
            .add_constraint(Constraint::Length(20))
            .add_constraint(Constraint::Percentage(40))
            .build()
            .split(chunks[1]);

        menu_rect = button_row[1];

        // Define menu buttons
        let items = vec![
            Button::new("Save", codex),
            Button::new("Load", codex),
            Button::new("Exit", codex),
        ];

        // Calculate item rects for hit testing in the next frame
        // Note: In a real app, you might want to calculate these more robustly
        item_rects.clear();
        if menu_open.clicked {
            for i in 0..items.len() {
                item_rects.push(Rect {
                    x: menu_rect.x,
                    y: menu_rect
                        .bottom()
                        .saturating_add(i as u16 * menu_rect.height),
                    width: menu_rect.width,
                    height: menu_rect.height,
                });
            }
        }

        let main_style = Style::builder()
            .set_bg(Colour::Normal(Normal::Blue))
            .set_fg(Colour::Normal(Normal::White))
            .build();

        let menu_style = Style::builder()
            .set_bg(Colour::Normal(Normal::Cyan))
            .set_fg(Colour::Normal(Normal::Black))
            .build();

        let mut menu = MenuButton::new(
            Button::new("File", codex)
                .with_state(&mut menu_open)
                .with_style(main_style),
            items,
        )
        .with_style(menu_style);

        let mut footer = Text::new(format!("Last Action: {}", last_action), codex).align_center();
        footer.render(canvas, chunks[2], codex);

        // Rendering last to show the menu button overdrawing the footer
        menu.render(canvas, menu_rect, codex);

        talos.present()?;
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
