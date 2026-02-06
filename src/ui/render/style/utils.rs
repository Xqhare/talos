use crate::{
    render::{
        Colour,
        colour::{
            BG_PRE_DIGIT_BRIGHT, BG_PRE_DIGIT_NORMAL, COLOURMODE_SIGNAL_BIT, EXTENDED_BG_BIT,
            EXTENDED_FG_BIT, Extended, FG_PRE_DIGIT_BRIGHT, FG_PRE_DIGIT_NORMAL,
            TRUE_COLOURMODE_SIGNAL_BIT,
        },
    },
    utils::push_u16_as_ascii,
};

pub fn handle_fg(colour: Colour, output_buffer: &mut Vec<u8>) {
    handle_colour(colour, true, output_buffer);
}

pub fn handle_bg(colour: Colour, output_buffer: &mut Vec<u8>) {
    handle_colour(colour, false, output_buffer);
}

fn handle_colour(colour: Colour, fg: bool, output_buffer: &mut Vec<u8>) {
    match colour {
        Colour::Normal(n) => {
            let base = if fg {
                FG_PRE_DIGIT_NORMAL
            } else {
                BG_PRE_DIGIT_NORMAL
            };
            let code = u16::from(base.saturating_mul(10) + n.decode());
            push_u16_as_ascii(output_buffer, code);
        }
        Colour::Bright(b) => {
            let base = if fg {
                FG_PRE_DIGIT_BRIGHT
            } else {
                BG_PRE_DIGIT_BRIGHT
            };
            let code = u16::from(base.saturating_mul(10) + b.decode());
            push_u16_as_ascii(output_buffer, code);
        }
        Colour::Extended(e) => match e {
            Extended::ColourMode(cm) => {
                // e.g. 38;5;255
                let mode_code = if fg { EXTENDED_FG_BIT } else { EXTENDED_BG_BIT };
                push_u16_as_ascii(output_buffer, u16::from(mode_code));

                output_buffer.extend_from_slice(b";");
                push_u16_as_ascii(output_buffer, u16::from(COLOURMODE_SIGNAL_BIT));

                output_buffer.extend_from_slice(b";");
                push_u16_as_ascii(output_buffer, u16::from(cm.decode()));
            }
            Extended::TrueColour(tc) => {
                // e.g. 38;2;255;0;0
                let mode_code = if fg { EXTENDED_FG_BIT } else { EXTENDED_BG_BIT };
                push_u16_as_ascii(output_buffer, u16::from(mode_code));

                output_buffer.extend_from_slice(b";");
                push_u16_as_ascii(output_buffer, u16::from(TRUE_COLOURMODE_SIGNAL_BIT));

                let (r, g, b) = tc.decode();
                output_buffer.extend_from_slice(b";");
                push_u16_as_ascii(output_buffer, u16::from(r));
                output_buffer.extend_from_slice(b";");
                push_u16_as_ascii(output_buffer, u16::from(g));
                output_buffer.extend_from_slice(b";");
                push_u16_as_ascii(output_buffer, u16::from(b));
            }
        },
    }
}
