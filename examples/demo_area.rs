
use std::thread;
use std::time::Duration;
use talos::input::{Event, KeyCode, KeyEvent};
use talos::layout::Rect;
use talos::render::{Colour, Normal, Style};
use talos::widgets::traits::Widget;
use talos::widgets::{Area, Block};
use talos::Talos;

fn main() -> Result<(), talos::error::TalosError> {
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
                        code: KeyCode::Esc,
                        ..
                    }) => {
                        running = false;
                    }
                    _ => {}
                }
            }
        }

        // 3. Render Frame
        talos.begin_frame();
        let (canvas, codex) = talos.render_ctx();

        // Background area (fills entire screen)
        let screen_rect = Rect::new(0, 0, canvas.max_width() + 1, canvas.max_height() + 1);
        let mut bg_area = Area::new();
        bg_area.style(
            Style::builder()
                .set_bg(Colour::Normal(Normal::Blue))
                .build(),
        );
        bg_area.render(canvas, screen_rect, codex);

        // A centered red area
        let center_rect = Rect::new(
            (canvas.max_width() / 2).saturating_sub(15),
            (canvas.max_height() / 2).saturating_sub(5),
            30,
            10,
        );
        let mut red_area = Area::new();
        red_area.style(
            Style::builder()
                .set_bg(Colour::Normal(Normal::Red))
                .build(),
        );
        red_area.render(canvas, center_rect, codex);

        // A block on top to show contrast
        let block_rect = Rect::new(
            (canvas.max_width() / 2).saturating_sub(10),
            (canvas.max_height() / 2).saturating_sub(3),
            20,
            6,
        );
        let mut block = Block::new()
            .title("Area Demo", codex, true)
            .with_bg_fill();
        block.style(
            Style::builder()
                .set_fg(Colour::Normal(Normal::White))
                .set_bg(Colour::Normal(Normal::Green))
                .build(),
        );
        block.render(canvas, block_rect, codex);

        // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
