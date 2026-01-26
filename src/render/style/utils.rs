use crate::render::{colour::{Extended, BG_PRE_DIGIT_BRIGHT, BG_PRE_DIGIT_NORMAL, COLOURMODE_SIGNAL_BIT, EXTENDED_BG_BIT, EXTENDED_FG_BIT, FG_PRE_DIGIT_BRIGHT, FG_PRE_DIGIT_NORMAL, TRUE_COLOURMODE_SIGNAL_BIT}, Colour};



pub fn handle_fg(colour: Colour) -> String {
    handle_colour(colour, true)
}

pub fn handle_bg(colour: Colour) -> String {
    handle_colour(colour, false)
}

fn handle_colour(colour: Colour, fg: bool) -> String {
    // There has to be a way to construct this without heap allocating
    match colour {
        Colour::Normal(n) => {
            let last = n.decode();
            if fg {
                format!("{}{}", FG_PRE_DIGIT_NORMAL, last)
            } else {
                format!("{}{}", BG_PRE_DIGIT_NORMAL, last)
            }
        },
        Colour::Bright(b) => {
            let last = b.decode();
            if fg {
                format!("{}{}", FG_PRE_DIGIT_BRIGHT, last)
            } else {
                format!("{}{}", BG_PRE_DIGIT_BRIGHT, last)
            }
        },
        Colour::Extended(e) => {
            match e {
                Extended::ColourMode(cm) => {
                    let rgb = cm.decode();
                    if fg {
                        format!("{};{};{}", EXTENDED_FG_BIT, COLOURMODE_SIGNAL_BIT, rgb )
                    } else {
                        format!("{};{};{}", EXTENDED_BG_BIT, COLOURMODE_SIGNAL_BIT, rgb )
                    }
                },
                Extended::TrueColour(tc) => {
                    let (r, g, b) = tc.decode();
                    if fg {
                        format!("{};{};{};{};{}", EXTENDED_FG_BIT, TRUE_COLOURMODE_SIGNAL_BIT, r, g, b )
                    } else {
                        format!("{};{};{};{};{}", EXTENDED_BG_BIT, TRUE_COLOURMODE_SIGNAL_BIT, r, g, b )
                    }
                }
            }
        },
    }
}
