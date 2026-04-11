use std::thread;
use std::time::Duration;
use talos::{
    LayoutBuilder, Talos,
    input::{Event, KeyCode, MouseButton, MouseEvent, MouseEventKind},
    layout::{Constraint, Direction, Rect},
    render::{Colour, Normal, Style},
    widgets::{
        Block, Text,
        stateful::{BlockBox, Button, ButtonState, MenuButton, TextBox, TextBoxState},
        traits::Widget,
    },
};

fn main() -> Result<(), talos::TalosError> {
    let mut talos = Talos::builder().build()?;
    let mut running = true;

    let codex = talos.codex();

    // The main button state
    let mut menu_open = ButtonState { clicked: false };
    // The sub-menu state (for "Load")
    let mut sub_menu_open = ButtonState { clicked: false };

    // TextBox state
    let mut path_text = String::from("some/path");
    let mut path_text_state = TextBoxState {
        active: false,
        cursor: Some(0),
        text: Text::new(&path_text, codex),
    };

    // Track which menu item was last clicked
    let mut last_action = String::from("None");

    let mut menu_rect = Rect::default();
    let mut item_rects: Vec<Rect> = Vec::new();
    let mut sub_menu_rect = Rect::default();

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
                            sub_menu_open.clicked = false;
                            path_text_state.active = false;
                        }
                        // If menu is open, check if any menu items were clicked
                        else if menu_open.clicked {
                            let mut handled = false;
                            for (i, rect) in item_rects.iter().enumerate() {
                                if rect.contains(*column, *row) {
                                    if i == 1 {
                                        // "Load"
                                        sub_menu_open.clicked = !sub_menu_open.clicked;
                                        path_text_state.active = sub_menu_open.clicked;
                                        path_text_state.cursor = Some(path_text.len());
                                    } else {
                                        last_action = format!("Item {}", i + 1);
                                        menu_open.clicked = false;
                                        sub_menu_open.clicked = false;
                                        path_text_state.active = false;
                                    }
                                    handled = true;
                                    break;
                                }
                            }

                            if !handled
                                && sub_menu_open.clicked
                                && sub_menu_rect.contains(*column, *row)
                            {
                                path_text_state.active = true;
                                path_text_state.cursor = Some(path_text.len());
                            } else if !handled {
                                // Clicked outside the menu and sub-menu
                                // (Simplification for the demo)
                            }
                        }
                    }
                    Event::KeyEvent(ev) => {
                        if path_text_state.active {
                            match ev.code {
                                KeyCode::Char(c) => {
                                    path_text.push(c);
                                }
                                KeyCode::Backspace => {
                                    path_text.pop();
                                }
                                KeyCode::Enter => {
                                    last_action = format!("Loaded: {}", path_text);
                                    path_text.clear();
                                    path_text_state.active = false;
                                    sub_menu_open.clicked = false;
                                    menu_open.clicked = false;
                                }
                                KeyCode::Esc => {
                                    path_text_state.active = false;
                                    sub_menu_open.clicked = false;
                                }
                                _ => {}
                            }
                        } else if ev.code == KeyCode::Char('q') || ev.code == KeyCode::Esc {
                            running = false;
                        }
                    }
                    _ => {}
                }
            }
        }

        talos.begin_frame();
        let (canvas, codex) = talos.render_ctx();

        // Update TextBox text
        path_text_state.text.set_content(&path_text, codex);
        path_text_state.cursor = if path_text_state.active {
            Some(path_text.len())
        } else {
            None
        };

        let chunks = LayoutBuilder::new()
            .direction(Direction::Vertical)
            .add_constraint(Constraint::Length(3)) // Header
            .add_constraint(Constraint::Length(3)) // Menu Button Row
            .add_constraint(Constraint::Min(0)) // Content
            .build()
            .split(canvas.size_rect());

        let mut header =
            Text::new("MenuButton Demo - Click File -> Load for sub-menu", codex).align_center();
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

        // Calculate item rects for hit testing in the next frame
        item_rects.clear();
        if menu_open.clicked {
            for i in 0..3 {
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

        let sub_menu_style = Style::builder()
            .set_bg(Colour::Normal(Normal::Green))
            .set_fg(Colour::Normal(Normal::Black))
            .build();

        let highlight_style = Style::builder()
            .set_bg(Colour::Normal(Normal::White))
            .set_fg(Colour::Normal(Normal::Black))
            .build();

        // Create the nested menu for "Load"
        let mut path_block = Block::new()
            .title("enter a path", codex, true)
            .with_bg_fill();
        let mut path_text_box =
            TextBox::new(&mut path_text_state).with_highlight_style(highlight_style);

        let mut block_box = BlockBox::new(&mut path_block, &mut path_text_box);
        block_box.style(sub_menu_style);

        let mut load_items: Vec<&mut dyn Widget> = vec![&mut block_box];
        let mut load_menu = MenuButton::new(
            Button::new("Load", codex)
                .with_state(&mut sub_menu_open)
                .with_style(menu_style),
            load_items.iter_mut(),
        )
        .with_horizontal_layout()
        .with_child_width(30)
        .with_child_height(3);

        // Define menu buttons for the main File menu
        let mut save_btn = Button::new("Save", codex).with_style(menu_style);
        let mut exit_btn = Button::new("Exit", codex).with_style(menu_style);

        let mut file_items: Vec<&mut dyn Widget> =
            vec![&mut save_btn, &mut load_menu, &mut exit_btn];

        let mut menu = MenuButton::new(
            Button::new("File", codex)
                .with_state(&mut menu_open)
                .with_style(main_style),
            file_items.iter_mut(),
        );

        let mut footer = Text::new(format!("Last Action: {}", last_action), codex).align_center();
        footer.render(canvas, chunks[2], codex);

        // Rendering last to show the menu button overdrawing the footer
        menu.render(canvas, menu_rect, codex);

        // Position of the sub-menu for hit testing
        if menu_open.clicked {
            sub_menu_rect = Rect {
                x: item_rects[1].right(),
                y: item_rects[1].y,
                width: 30,
                height: 3,
            };
        }

        talos.present()?;
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
