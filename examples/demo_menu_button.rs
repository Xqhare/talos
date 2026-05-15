use talos::{
    LayoutBuilder, Talos,
    input::{Event, KeyCode, KeyEvent, MouseButton, MouseEventKind},
    layout::{Constraint, Direction, Rect},
    render::{Colour, Normal, Style},
    widgets::{
        Block, Text,
        stateful::{BlockBox, Button, ButtonState, MenuButton, TextBox, TextBoxState},
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

    // --- State for the MenuButton Demo ---
    let mut menu_open = ButtonState { clicked: false };
    let mut sub_menu_open = ButtonState { clicked: false };

    let mut path_text_state = TextBoxState::default();
    path_text_state.text = Text::new("talos/examples/demo_menu_button.rs", talos.codex());

    let mut last_action = String::from("None");

    // Clickable areas for basic mouse hit testing (since this demo is simple)
    let mut menu_rect = Rect::default();
    let mut sub_menu_rect = Rect::default();

    while running {
        // 2. Handle Input
        let events = talos.poll_input().ok().flatten().map(|e| e.to_vec());
        if let Some(events) = events {
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
                    Event::MouseEvent(mouse_event) => {
                        if let MouseEventKind::Up(MouseButton::Left) = mouse_event.kind {
                            // Check if the main File menu was clicked
                            if menu_rect.contains(mouse_event.column, mouse_event.row) {
                                menu_open.clicked = !menu_open.clicked;
                                last_action = String::from("Toggled Main Menu");
                            } else if menu_open.clicked {
                                // Define the areas of the sub-items for hit testing
                                // Main menu is vertical, sub-items are below it
                                let item_rects = LayoutBuilder::new()
                                    .direction(Direction::Vertical)
                                    .add_constraint(Constraint::Length(3))
                                    .add_constraint(Constraint::Length(3))
                                    .add_constraint(Constraint::Length(3))
                                    .build()
                                    .split(Rect {
                                        x: menu_rect.x,
                                        y: menu_rect.bottom(),
                                        width: menu_rect.width,
                                        height: 9,
                                    });

                                if item_rects[0].contains(mouse_event.column, mouse_event.row) {
                                    last_action = String::from("Save Clicked");
                                    menu_open.clicked = false;
                                } else if item_rects[1].contains(mouse_event.column, mouse_event.row)
                                {
                                    sub_menu_open.clicked = !sub_menu_open.clicked;
                                    last_action = String::from("Toggled Load Sub-Menu");
                                } else if item_rects[2].contains(mouse_event.column, mouse_event.row)
                                {
                                    running = false;
                                } else if sub_menu_open.clicked
                                    && sub_menu_rect.contains(mouse_event.column, mouse_event.row)
                                {
                                    last_action = String::from("Sub-Menu path clicked");
                                    sub_menu_open.clicked = false;
                                    menu_open.clicked = false;
                                } else {
                                    menu_open.clicked = false;
                                    sub_menu_open.clicked = false;
                                }
                            }
                        }
                    }
                    Event::KeyEvent(key_event) => {
                        if sub_menu_open.clicked {
                            let path_text = path_text_state.text.get_content().to_string();
                            let mut new_path = path_text.clone();
                            match key_event.code {
                                KeyCode::Char(c) => new_path.push(c),
                                KeyCode::Backspace => {
                                    new_path.pop();
                                }
                                _ => {}
                            }
                            if new_path != path_text {
                                path_text_state.text.set_content(&new_path, talos.codex());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // 3. Render Frame
        talos.begin_frame();
        let mut ctx = talos.render_ctx();
        let size = ctx.canvas.size_rect();

        // Layout the screen
        let chunks = LayoutBuilder::new()
            .direction(Direction::Vertical)
            .add_constraint(Constraint::Length(1))
            .add_constraint(Constraint::Min(10))
            .add_constraint(Constraint::Length(1))
            .build()
            .split(size);

        menu_rect = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 1,
        };

        // Styles
        let main_style = Style::builder()
            .set_fg(Colour::Normal(Normal::White))
            .set_bg(Colour::Normal(Normal::Blue))
            .build();
        let menu_style = Style::builder()
            .set_fg(Colour::Normal(Normal::Black))
            .set_bg(Colour::Normal(Normal::White))
            .build();

        // 1. Create the sub-menu for the "Load" button
        let text_box = TextBox::new(&mut path_text_state).with_style(menu_style);
        let mut block = Block::new().with_style(menu_style).with_fat_border();
        block = block.title("enter a path", ctx.codex, true);
        let block_box = BlockBox::new(block, text_box).with_style(menu_style);

        let load_menu = MenuButton::new(
            Button::new("Load", &mut sub_menu_open, ctx.codex).with_style(menu_style),
        )
        .add(block_box)
        .with_horizontal_layout()
        .with_child_width(30)
        .with_child_height(3);

        // Define menu buttons for the main File menu
        let mut sub_menu_save = ButtonState { clicked: false };
        let mut sub_menu_exit = ButtonState { clicked: false };
        let save_btn = Button::new("Save", &mut sub_menu_save, ctx.codex).with_style(menu_style);
        let exit_btn = Button::new("Exit", &mut sub_menu_exit, ctx.codex).with_style(menu_style);

        let is_menu_open = menu_open.clicked;
        let mut menu = MenuButton::new(
            Button::new("File", &mut menu_open, ctx.codex).with_style(main_style),
        )
        .add(save_btn)
        .add(load_menu)
        .add(exit_btn);

        let mut footer = Text::new(format!("Last Action: {}", last_action), ctx.codex).align_center();
        footer.render(&mut ctx, chunks[2]);

        menu.render(&mut ctx, menu_rect);

        if is_menu_open {
            let item_rects = LayoutBuilder::new()
                .direction(Direction::Vertical)
                .add_constraint(Constraint::Length(3))
                .add_constraint(Constraint::Length(3))
                .add_constraint(Constraint::Length(3))
                .build()
                .split(Rect {
                    x: menu_rect.x,
                    y: menu_rect.bottom(),
                    width: menu_rect.width,
                    height: 9,
                });
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
