// TODO: Consider to create to a `fn csi(command: &str, args: &[u16]) -> String` helper function to
// construct ANSI control sequences
//
// I would still keep the constants here though. - They are easier to find and clearer to read

pub const CONTROL_SEQUENCE_INTRO: &str = "\x1B[";

pub const CLEAR_ALL: &str = "\x1b[2J";
pub const TO_TOP_LEFT: &str = "\x1b[H";

pub const HIDE_CURSOR: &str = "\x1b[?25l";
pub const SHOW_CURSOR: &str = "\x1b[?25h";
pub const ENTER_ALT_SCREEN: &str = "\x1b[?1049h";
pub const EXIT_ALT_SCREEN: &str = "\x1b[?1049l";
