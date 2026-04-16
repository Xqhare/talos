mod list;
pub use list::{List, ListState};

mod signal_box;
pub use signal_box::{SignalBox, SignalBoxState};

mod fillable_bar;
pub use fillable_bar::{FillableBar, FillableBarState};

mod table;
pub use table::{InnerBorder, Table, TableState};

mod button;
pub use button::{Button, ButtonState};

mod text_box;
pub use text_box::{TextBox, TextBoxState};

mod menu_button;
pub use menu_button::{MenuButton, MenuButtonState};

mod block_box;
pub use block_box::{BlockBox, BlockBoxState};

mod check_box;
pub use check_box::{CheckBox, CheckBoxState};

mod sequence;
pub use sequence::{Sequence, SequenceState};

/// Generic widget state
///
/// Useful for constructing generic stores of States
pub enum States<'a> {
    /// Block Box State
    BlockBox(BlockBoxState<'a>),
    /// Button State
    Button(ButtonState),
    /// Check Box State
    CheckBox(CheckBoxState<'a>),
    /// Fillable Bar State
    FillableBar(FillableBarState),
    /// List State
    List(ListState),
    /// Menu Button State
    MenuButton(MenuButtonState<'a>),
    /// Sequence State
    Sequence(SequenceState),
    /// Signal Box State
    SignalBox(SignalBoxState),
    /// Table State
    Table(TableState),
    /// Text Box State
    TextBox(TextBoxState),
}

impl<'a> States<'a> {
    /// Gets the state of the block box
    pub fn as_block_box(&self) -> Option<&BlockBoxState<'a>> {
        match self {
            States::BlockBox(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the mutable state of the block box
    pub fn as_block_box_mut(&mut self) -> Option<&mut BlockBoxState<'a>> {
        match self {
            States::BlockBox(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the state of the button
    pub fn as_button(&self) -> Option<&ButtonState> {
        match self {
            States::Button(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the mutable state of the button
    pub fn as_button_mut(&mut self) -> Option<&mut ButtonState> {
        match self {
            States::Button(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the state of the checkbox
    pub fn as_check_box(&self) -> Option<&CheckBoxState<'a>> {
        match self {
            States::CheckBox(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the mutable state of the checkbox
    pub fn as_check_box_mut(&mut self) -> Option<&mut CheckBoxState<'a>> {
        match self {
            States::CheckBox(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the state of the fillable bar
    pub fn as_fillable_bar(&self) -> Option<&FillableBarState> {
        match self {
            States::FillableBar(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the mutable state of the fillable bar
    pub fn as_fillable_bar_mut(&mut self) -> Option<&mut FillableBarState> {
        match self {
            States::FillableBar(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the state of the list
    pub fn as_list(&self) -> Option<&ListState> {
        match self {
            States::List(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the mutable state of the list
    pub fn as_list_mut(&mut self) -> Option<&mut ListState> {
        match self {
            States::List(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the state of the menu button
    pub fn as_menu_button(&self) -> Option<&MenuButtonState<'a>> {
        match self {
            States::MenuButton(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the mutable state of the menu button
    pub fn as_menu_button_mut(&mut self) -> Option<&mut MenuButtonState<'a>> {
        match self {
            States::MenuButton(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the state of the sequence
    pub fn as_sequence(&self) -> Option<&SequenceState> {
        match self {
            States::Sequence(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the mutable state of the sequence
    pub fn as_sequence_mut(&mut self) -> Option<&mut SequenceState> {
        match self {
            States::Sequence(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the state of the signal box
    pub fn as_signal_box(&self) -> Option<&SignalBoxState> {
        match self {
            States::SignalBox(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the mutable state of the signal box
    pub fn as_signal_box_mut(&mut self) -> Option<&mut SignalBoxState> {
        match self {
            States::SignalBox(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the state of the table
    pub fn as_table(&self) -> Option<&TableState> {
        match self {
            States::Table(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the mutable state of the table
    pub fn as_table_mut(&mut self) -> Option<&mut TableState> {
        match self {
            States::Table(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the state of the text box
    pub fn as_text_box(&self) -> Option<&TextBoxState> {
        match self {
            States::TextBox(state) => Some(state),
            _ => None,
        }
    }

    /// Gets the mutable state of the text box
    pub fn as_text_box_mut(&mut self) -> Option<&mut TextBoxState> {
        match self {
            States::TextBox(state) => Some(state),
            _ => None,
        }
    }
}

impl<'a> From<BlockBoxState<'a>> for States<'a> {
    fn from(state: BlockBoxState<'a>) -> Self {
        States::BlockBox(state)
    }
}

impl From<ButtonState> for States<'_> {
    fn from(state: ButtonState) -> Self {
        States::Button(state)
    }
}

impl<'a> From<CheckBoxState<'a>> for States<'a> {
    fn from(state: CheckBoxState<'a>) -> Self {
        States::CheckBox(state)
    }
}

impl From<FillableBarState> for States<'_> {
    fn from(state: FillableBarState) -> Self {
        States::FillableBar(state)
    }
}

impl From<ListState> for States<'_> {
    fn from(state: ListState) -> Self {
        States::List(state)
    }
}

impl<'a> From<MenuButtonState<'a>> for States<'a> {
    fn from(state: MenuButtonState<'a>) -> Self {
        States::MenuButton(state)
    }
}

impl From<SequenceState> for States<'_> {
    fn from(state: SequenceState) -> Self {
        States::Sequence(state)
    }
}

impl From<SignalBoxState> for States<'_> {
    fn from(state: SignalBoxState) -> Self {
        States::SignalBox(state)
    }
}

impl From<TableState> for States<'_> {
    fn from(state: TableState) -> Self {
        States::Table(state)
    }
}

impl From<TextBoxState> for States<'_> {
    fn from(state: TextBoxState) -> Self {
        States::TextBox(state)
    }
}
