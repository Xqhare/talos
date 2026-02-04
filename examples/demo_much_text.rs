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

        let big_area_left = Rect::new(1, 1, canvas.max_width() / 2, canvas.max_height());

        let big_area_right = Rect::new(canvas.max_width() / 2 + 1, 1, canvas.max_width() / 2, canvas.max_height());

        let style_left = Style::builder()
            .set_fg(Colour::Normal(Normal::Yellow))
            .set_bg(Colour::Normal(Normal::Blue))
            .build();
        let style_right = Style::builder()
            .set_fg(Colour::Normal(Normal::Red))
            .set_bg(Colour::Normal(Normal::Green))
            .build();

        let mut large_block_left: Block = Block::new()
            .title("", codex, false)
            .with_bg_fill();

        large_block_left.style(style_left);

        let mut large_block_right: Block = Block::new()
            .title("", codex, false)
            .with_bg_fill();

        large_block_right.style(style_right);

        large_block_left.render(canvas, big_area_left, codex);

        let left_block_inner = large_block_left.inner(big_area_left);

        let left_text_style = Style::builder()
            .set_fg(Colour::Normal(Normal::Black))
            .set_bg(Colour::Normal(Normal::Blue))
            .set_bold(true)
            .build();

        let mut left_text = Text::new("Centered Text on the Left! \n \n  Duis vitae erat quis massa faucibus maximus. Suspendisse efficitur sem cursus, euismod augue ultrices, interdum tortor. Maecenas non erat quis nunc finibus sodales eu non quam. Duis auctor lectus vitae quam ullamcorper porta.\n Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur sed maximus eros. Maecenas id placerat eros, in ultrices lectus. Fusce accumsan pellentesque mi, aliquam fermentum ipsum consectetur eu. \n \n
        Donec tempor consectetur tortor, ut vestibulum tortor rhoncus vel. In hac habitasse platea dictumst. Curabitur ut diam molestie, placerat massa accumsan, consectetur neque.\n Etiam accumsan hendrerit tellus, et dapibus nunc aliquam posuere. Duis ac nibh libero. Nunc sapien urna, luctus a augue vel, efficitur hendrerit ligula.\n Donec et maximus nunc, a ultrices augue. Quisque viverra purus nisi, dictum congue mi aliquet sit amet. Fusce eu dignissim elit.\n Etiam urna massa, hendrerit vel commodo non, dictum at augue. Pellentesque dapibus diam ut rutrum mattis. Quisque eu mi arcu. Vestibulum bibendum leo et leo pellentesque, vel finibus lectus ornare. Aenean tincidunt facilisis ipsum, a mattis nisl accumsan efficitur. Duis vitae erat quis massa faucibus maximus. Suspendisse efficitur sem cursus, euismod augue ultrices, interdum tortor. Maecenas non erat quis nunc finibus sodales eu non quam. Duis auctor lectus vitae quam ullamcorper porta.\n Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur sed maximus eros. Maecenas id placerat eros, in ultrices lectus. Fusce accumsan pellentesque mi, aliquam fermentum ipsum consectetur eu. \n \n
        Donec tempor consectetur tortor, ut vestibulum tortor rhoncus vel. In hac habitasse platea dictumst. Curabitur ut diam molestie, placerat massa accumsan, consectetur neque.\n Etiam accumsan hendrerit tellus, et dapibus nunc aliquam posuere. Duis ac nibh libero. Nunc sapien urna, luctus a augue vel, efficitur hendrerit ligula.\n Donec et maximus nunc, a ultrices augue. Quisque viverra purus nisi, dictum congue mi aliquet sit amet. Fusce eu dignissim elit.\n Etiam urna massa, hendrerit vel commodo non, dictum at augue. Pellentesque dapibus diam ut rutrum mattis. Quisque eu mi arcu. Vestibulum bibendum leo et leo pellentesque, vel finibus lectus ornare. Aenean tincidunt facilisis ipsum, a mattis nisl accumsan efficitur.", codex)
            .align_center();

        left_text.style(left_text_style);

        left_text
            .render(canvas, left_block_inner, codex);

        large_block_right.render(canvas, big_area_right, codex);

        let right_block_inner = large_block_right.inner(big_area_right);
        
        let right_text_style = Style::builder()
            .set_bg(Colour::Normal(Normal::Green))
            .set_fg(Colour::Normal(Normal::Yellow))
            .set_bold(true)
            .build();
        let mut right_text = Text::new("Vertically centered Text on the Right!\n \nQuisque molestie, nisi eget imperdiet varius, lorem ante rhoncus metus, sed convallis lacus dolor a lectus. Duis facilisis, ligula eget sodales vehicula, erat mauris luctus est, vitae blandit ex dui sit amet ligula.\n  Donec et posuere tellus. Nunc volutpat ipsum sit amet tristique sodales. Morbi ac convallis metus. Etiam et ante et lacus lobortis luctus. Nullam id varius dolor.\n  Vestibulum dolor neque, dictum elementum est luctus, malesuada vestibulum arcu. Maecenas in mi ut arcu iaculis feugiat eget a erat. Curabitur id nisl id est porta suscipit sollicitudin in erat. Suspendisse blandit ligula orci, sit amet faucibus neque blandit eget. Phasellus eget fermentum libero.
        \n \n
        In quis laoreet lorem. Cras mattis, lacus in consequat bibendum, felis enim dapibus tellus, sit amet posuere nunc nisi quis arcu. Praesent eros lorem, hendrerit ac libero sed, ullamcorper suscipit turpis.\n  Vestibulum accumsan sit amet lectus sed pharetra.\n  Sed et justo eget metus condimentum fringilla vel ac nisi. Duis aliquet tortor eget urna feugiat tincidunt. Ut at elementum augue. In tristique euismod orci, non aliquet dui efficitur volutpat. Vestibulum et libero luctus velit dignissim mollis. Aenean felis nulla, fermentum ut bibendum sed, cursus id turpis.\n  Donec mauris velit, placerat quis consectetur a, iaculis a eros. Donec finibus massa vitae elit laoreet, in mollis ante luctus. Quisque molestie, nisi eget imperdiet varius, lorem ante rhoncus metus, sed convallis lacus dolor a lectus. Duis facilisis, ligula eget sodales vehicula, erat mauris luctus est, vitae blandit ex dui sit amet ligula.\n  Donec et posuere tellus. Nunc volutpat ipsum sit amet tristique sodales. Morbi ac convallis metus. Etiam et ante et lacus lobortis luctus. Nullam id varius dolor.\n  Vestibulum dolor neque, dictum elementum est luctus, malesuada vestibulum arcu. Maecenas in mi ut arcu iaculis feugiat eget a erat. Curabitur id nisl id est porta suscipit sollicitudin in erat. Suspendisse blandit ligula orci, sit amet faucibus neque blandit eget. Phasellus eget fermentum libero. \n In quis laoreet lorem. Cras mattis, lacus in consequat bibendum, felis enim dapibus tellus, sit amet posuere nunc nisi quis arcu. Praesent eros lorem, hendrerit ac libero sed, ullamcorper suscipit turpis.", codex)
            .align_vertically();

        right_text.style(right_text_style);

        right_text
            .render(canvas, right_block_inner, codex);

        // Let's draw a white & black block in the middle
        let area = Rect::new(70, 10, 50, 30);
        
        let style = Style::builder()
            .set_fg(Colour::Normal(Normal::Black))
            .set_bg(Colour::Normal(Normal::White))
            .build();

        let mut block: Block = Block::new()
            .title("Center Block Title", codex, true)
            .with_beautify_border_breaks()
            .with_bg_fill();

        block.style(style);

        block.render(canvas, area, codex);

        // Lets add some styled text to the black and white block
        let block_inner = block.inner(area);

        let text_style = Style::builder()
            .set_bg(Colour::Normal(Normal::White))
            .set_fg(Colour::Normal(Normal::Blue))
            .set_bold(true)
            .build();

        let mut text = Text::new("Look mom! Text inside a block! \n Incredible! \n It's centered and justified! \n It supports several lines, of varying lengths as well! \n \n Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut vel mauris nec nulla porta dictum quis sit amet eros. Praesent mattis odio ac malesuada ultricies. Aenean accumsan dolor ac est eleifend, ac commodo lacus dictum. Vestibulum egestas porttitor convallis. Pellentesque consequat metus turpis. \n \n Suspendisse imperdiet orci eu mi lacinia, id aliquam enim venenatis. Cras aliquet ut lectus vitae blandit. Cras ac tortor consectetur, dictum diam in, volutpat nulla. Nunc aliquet lacinia vulputate. Aliquam efficitur massa sed neque mollis, nec fermentum dolor blandit. Proin finibus tortor at varius gravida. Ut risus mauris, tempor et enim sed, gravida varius ante. Nulla convallis bibendum nulla, ut semper metus eleifend vel. Donec porta nisi in lorem consectetur, eu cursus justo sollicitudin. Vivamus interdum tincidunt diam sit amet luctus.", codex)
            .align_center()
            .align_vertically();

        text.style(text_style);

        text
            .render(canvas, block_inner, codex);

                // 4. Present to Terminal
        talos.present()?;

        // Cap framerate slightly to save CPU
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
