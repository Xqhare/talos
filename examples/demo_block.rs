use talos::{Talos, input::{Event, KeyEvent, KeyCode}, render::{Colour, Normal, Style, traits::Widget}, layout::Rect, widgets::{Block, Text}};

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

        // Let's draw a white & black block in the middle
        let area = Rect::new(15, 15, 50, 10);
        
        let style = Style::builder()
            .set_fg(Colour::Normal(Normal::Black))
            .set_bg(Colour::Normal(Normal::White))
            .build();

        let block: Block = Block::new()
            .title(" Hello Talos ")
            .style(style)
            .with_bg_fill();

        block.render(canvas, area, codex);

        // Lets add some styled text to the block
        let block_inner = block.inner(area);

        let text_style = Style::builder()
            .set_bg(Colour::Normal(Normal::White))
            .set_fg(Colour::Normal(Normal::Blue))
            .set_bold(true)
            .build();

        let _text = Text::new("Look mom! Text inside a block!")
            .style(text_style)
            .align_center()
            .align_vertically()
            .render(canvas, block_inner, codex);

        // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
