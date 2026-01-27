use crate::render::{
    Colour,
    colour::{
        BG_PRE_DIGIT_BRIGHT, BG_PRE_DIGIT_NORMAL, COLOURMODE_SIGNAL_BIT, EXTENDED_BG_BIT,
        EXTENDED_FG_BIT, Extended, FG_PRE_DIGIT_BRIGHT, FG_PRE_DIGIT_NORMAL,
        TRUE_COLOURMODE_SIGNAL_BIT,
    },
};

pub fn handle_fg(colour: Colour, output_buffer: &mut Vec<u8>) {
    handle_colour(colour, true, output_buffer)
}

pub fn handle_bg(colour: Colour, output_buffer: &mut Vec<u8>) {
    handle_colour(colour, false, output_buffer)
}

fn handle_colour(colour: Colour, fg: bool, output_buffer: &mut Vec<u8>) {
    match colour {
        Colour::Normal(n) => {
            if fg {
                output_buffer.push(FG_PRE_DIGIT_NORMAL);
            } else {
                output_buffer.push(BG_PRE_DIGIT_NORMAL);
            }
            output_buffer.push(n.decode());
        }
        Colour::Bright(b) => {
            if fg {
                output_buffer.push(FG_PRE_DIGIT_BRIGHT);
            } else {
                output_buffer.push(BG_PRE_DIGIT_BRIGHT);
            }
            output_buffer.push(b.decode());
        }
        Colour::Extended(e) => match e {
            Extended::ColourMode(cm) => {
                if fg {
                    output_buffer.push(EXTENDED_FG_BIT);
                } else {
                    output_buffer.push(EXTENDED_BG_BIT);
                }
                output_buffer.extend_from_slice(b";");
                output_buffer.push(COLOURMODE_SIGNAL_BIT);
                output_buffer.extend_from_slice(b";");
                output_buffer.push(cm.decode());
            }
            Extended::TrueColour(tc) => {
                let (r, g, b) = tc.decode();
                if fg {
                    output_buffer.push(EXTENDED_FG_BIT);
                } else {
                    output_buffer.push(EXTENDED_BG_BIT);
                }
                output_buffer.extend_from_slice(b";");
                output_buffer.push(TRUE_COLOURMODE_SIGNAL_BIT);
                output_buffer.extend_from_slice(b";");
                output_buffer.push(r);
                output_buffer.extend_from_slice(b";");
                output_buffer.push(g);
                output_buffer.extend_from_slice(b";");
                output_buffer.push(b);
            }
        },
    }
}
