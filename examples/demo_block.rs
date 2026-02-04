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

        // Let's draw a white & black block in the middle
        let area = Rect::new(15, 15, 90, 10);
        
        let style = Style::builder()
            .set_fg(Colour::Normal(Normal::Black))
            .set_bg(Colour::Normal(Normal::White))
            .build();

        let mut block: Block = Block::new()
            .title(" Hello Talos ", codex, false)
            .top_subtitle("Top Subtitle", codex)
            .bottom_left_subtitle("Bottom Left Subtitle", codex)
            .bottom_right_subtitle("Bottom Right Subtitle", codex)
            .bottom_center_subtitle("Bottom Subtitle", codex)
            .with_beautify_border_breaks()
            .with_bg_fill();
        block.style(style);

        block.render(canvas, area, codex);
        let area2 = Rect::new(2, 2, 100, 10);
        
        let style2 = Style::builder()
            .set_fg(Colour::Normal(Normal::White))
            .set_bg(Colour::Normal(Normal::Magenta))
            .build();

        let mut block2: Block = Block::new()
            .title(" Hello Talos ", codex, false)
            .top_subtitle("Top Subtitle", codex)
            .bottom_left_subtitle("Bottom Left Subtitle", codex)
            .bottom_right_subtitle("Bottom Right Subtitle", codex)
            .bottom_center_subtitle("Bottom Subtitle", codex)
            .with_beautify_border_breaks()
            .with_fat_border()
            .with_bg_fill();

        block2.style(style2);

        block2.render(canvas, area2, codex);
        let area3 = Rect::new(10, 30, 80, 10);
        
        let style3 = Style::builder()
            .set_fg(Colour::Normal(Normal::Magenta))
            .set_bg(Colour::Normal(Normal::Black))
            .build();

        let mut block3: Block = Block::new()
            .title(" Hello Talos ", codex, false)
            .top_subtitle("Top Subtitle", codex)
            .bottom_left_subtitle("Bottom Left Subtitle", codex)
            .bottom_right_subtitle("Bottom Right Subtitle", codex)
            .bottom_center_subtitle("Bottom Subtitle", codex)
            .with_bg_fill();

        block3.style(style3);

        block3.render(canvas, area3, codex);
        // Lets add some styled text to the block
        let block_inner = block.inner(area);

        let text_style = Style::builder()
            .set_bg(Colour::Normal(Normal::White))
            .set_fg(Colour::Normal(Normal::Blue))
            .set_bold(true)
            .build();

        let mut text = Text::new("Look mom! Text inside a block!", codex)
            .align_center()
            .align_vertically();

        text.style(text_style);
        text.render(canvas, block_inner, codex);

        // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
