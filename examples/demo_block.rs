use talos::{Talos, input::{Event, KeyEvent, KeyCode}, render::{Colour, Normal, Style, traits::Widget}, layout::Rect, widgets::Block};

// A simple helper to make the loop cleaner
use std::thread;
use std::time::Duration;

fn main() -> Result<(), talos::TalosError> {
    // 1. Initialize Talos
    let mut talos = Talos::builder()
        .build()?;

    let mut running = true;

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
                    _ => {}
                }
            }
        }

        // 3. Render Frame
        talos.begin_frame();
        let (canvas, codex) = talos.render_ctx();

        // Let's draw a red block in the middle
        let area = Rect::new(15, 15, 30, 10);
        
        let style = Style::builder()
            .set_fg(Colour::Normal(Normal::Red))
            .build();

        Block::new()
            .title(" Hello Talos ")
            .style(style)
            .render(canvas, area, codex);

        // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
