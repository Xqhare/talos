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

/// Handles the foreground color.
pub fn handle_fg(colour: Colour, output_buffer: &mut Vec<u8>) {
    handle_colour(colour, true, output_buffer);
}

/// Handles the background color.
pub fn handle_bg(colour: Colour, output_buffer: &mut Vec<u8>) {
    handle_colour(colour, false, output_buffer);
}

/// Low-level color handling logic.
pub(crate) fn handle_colour(colour: Colour, fg: bool, output_buffer: &mut Vec<u8>) {
    match colour {
        Colour::Normal(n) => {
            let base = if fg {
                FG_PRE_DIGIT_NORMAL
            } else {
                BG_PRE_DIGIT_NORMAL
            };
            let code = [0x1b, b'[', base, b'0' + n.decode(), b'm'];
            output_buffer.extend_from_slice(&code);
        }
        Colour::Bright(n) => {
            let base = if fg {
                FG_PRE_DIGIT_BRIGHT
            } else {
                BG_PRE_DIGIT_BRIGHT
            };
            let code = [0x1b, b'[', base, b'0' + n.decode(), b'm'];
            output_buffer.extend_from_slice(&code);
        }
        Colour::Extended(e) => {
            let base = if fg { EXTENDED_FG_BIT } else { EXTENDED_BG_BIT };
            output_buffer.extend_from_slice(b"\x1b[");
            push_u16_as_ascii(output_buffer, u16::from(base));
            output_buffer.push(b';');

            match e {
                Extended::ColourMode(cm) => {
                    push_u16_as_ascii(output_buffer, u16::from(COLOURMODE_SIGNAL_BIT));
                    output_buffer.push(b';');
                    push_u16_as_ascii(output_buffer, u16::from(cm.decode()));
                }
                Extended::TrueColour(tc) => {
                    push_u16_as_ascii(output_buffer, u16::from(TRUE_COLOURMODE_SIGNAL_BIT));
                    output_buffer.push(b';');
                    let (r, g, b) = tc.decode();
                    push_u16_as_ascii(output_buffer, u16::from(r));
                    output_buffer.extend_from_slice(b";");
                    push_u16_as_ascii(output_buffer, u16::from(g));
                    output_buffer.extend_from_slice(b";");
                    push_u16_as_ascii(output_buffer, u16::from(b));
                }
            }
            output_buffer.push(b'm');
        }
    }
}
