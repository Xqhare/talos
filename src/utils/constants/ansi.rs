//! This module contains ANSI escape sequences used by the Talos library.

/// CSI (Control Sequence Introducer)
pub const CONTROL_SEQUENCE_INTRO: &str = "\x1B[";

/// Clear the entire screen.
pub const CLEAR_ALL: &str = "\x1b[2J";
/// Move the cursor to the top left corner (1,1).
pub const TO_TOP_LEFT: &str = "\x1b[H";

/// Hide the terminal cursor.
pub const HIDE_CURSOR: &str = "\x1b[?25l";
/// Show the terminal cursor.
pub const SHOW_CURSOR: &str = "\x1b[?25h";
/// Enter the alternate screen buffer.
pub const ENTER_ALT_SCREEN: &str = "\x1b[?1049h";
/// Exit the alternate screen buffer.
pub const EXIT_ALT_SCREEN: &str = "\x1b[?1049l";

/// Enable mouse reporting.
pub const MOUSE_REPORTING_CODE: &str = "\x1b[?1000h";
/// Enable SGR mouse formatting.
pub const MOUSE_FORMATTING_CODE: &str = "\x1b[?1006h";
/// Disable mouse reporting.
pub const DISABLE_MOUSE_REPORTING_CODE: &str = "\x1b[?1000l";
/// Disable SGR mouse formatting.
pub const DISABLE_MOUSE_FORMATTING_CODE: &str = "\x1b[?1006l";

/// Begin synchronized terminal update.
pub const BEGIN_SYNC_UPDATE: &str = "\x1b[?2026h";
/// End synchronized terminal update.
pub const END_SYNC_UPDATE: &str = "\x1b[?2026l";
