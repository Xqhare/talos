// TODO: Consider to move to a `fn csi(command: &str, args: &[u16]) -> String` helper function to
// construct ANSI control sequences
//
// HOWEVER: I fear that the codebase will become harder to read - Right now its very clear what
// sequence is called. Moving away from clearly defined constants could make the code less readable
// e.g: 
// current code: `write_all_bytes(&mut self.output_buffer, TO_TOP_LEFT.as_bytes())?;`
// new code: `write_all_bytes(&mut self.output_buffer, self.get_csi("\x1b[H", &[]))?;`
// This does not seem like a great change.

pub const CONTROL_SEQUENCE_INTRO: &str = "\x1B[";

pub const CLEAR_ALL: &str = "\x1b[2J";
pub const TO_TOP_LEFT: &str = "\x1b[H";

pub const HIDE_CURSOR: &str = "\x1b[?25l";
pub const SHOW_CURSOR: &str = "\x1b[?25h";
pub const ENTER_ALT_SCREEN: &str = "\x1b[?1049h";
pub const EXIT_ALT_SCREEN: &str = "\x1b[?1049l";
