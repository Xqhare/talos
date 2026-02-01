use talos::{Talos, input::{Event, KeyEvent, KeyCode}, render::{Colour, Normal, Style}, layout::Rect, widgets::{Block, Text, traits::Widget}};

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

        let big_area = Rect::new(1, 1, canvas.max_width(), canvas.max_height());

        let style = Style::builder()
            .set_fg(Colour::Normal(Normal::Yellow))
            .set_bg(Colour::Normal(Normal::Blue))
            .build();

        let large_block: Block = Block::new()
            .title("")
            .style(style)
            .with_bg_fill();

        large_block.render(canvas, big_area, codex);

        let right_area = Rect::new(canvas.max_width().saturating_sub(60), 5, 30, 5);

        let style = Style::builder()
            .set_fg(Colour::Normal(Normal::White))
            .set_bg(Colour::Normal(Normal::Red))
            .build();

        let right_block: Block = Block::new()
            .title("Right")
            .style(style)
            .with_bg_fill();

        right_block.render(canvas, right_area, codex);

        let drawing_over_right = Rect::new(canvas.max_width().saturating_sub(40), 8, 30, 5);

        let style = Style::builder()
            .set_fg(Colour::Normal(Normal::White))
            .set_bg(Colour::Normal(Normal::Green))
            .build();

        let next_right_block: Block = Block::new()
            .title("Over Right")
            .style(style)
            .with_bg_fill();

        next_right_block.render(canvas, drawing_over_right, codex);

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

        let _text = Text::new("Look mom! Text inside a block!", codex)
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
