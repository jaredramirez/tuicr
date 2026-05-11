use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::InputMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    // Navigation
    CursorDown(usize),
    CursorUp(usize),
    HalfPageDown,
    HalfPageUp,
    PageDown,
    PageUp,
    GoToTop,
    GoToBottom,
    Digit(u8),
    // Chord prefixes (set state in main.rs; the following key dispatches)
    PendingZCommand,
    PendingShiftZCommand,
    PendingGCommand,
    PendingSpaceCommand,
    PendingBracketLeftCommand,
    PendingBracketRightCommand,
    ScrollLeft(usize),
    ScrollRight(usize),
    ScrollViewDown(usize),
    ScrollViewUp(usize),
    MouseScrollUp(usize),
    MouseScrollDown(usize),

    // Panel focus
    ToggleFocus,
    ToggleFocusReverse,
    SelectFile,

    // Review actions
    ToggleReviewed,
    AddLineComment,
    AddFileComment,
    EditComment,
    PendingDCommand,
    SearchNext,
    SearchPrev,

    // Visual selection mode
    EnterVisualMode,
    AddRangeComment,

    // Session
    Quit,
    ExportToClipboard,

    // Mode changes
    EnterCommandMode,
    EnterSearchMode,
    ExitMode,
    ToggleHelp,

    // Text input
    InsertChar(char),
    DeleteChar,
    DeleteWord,
    ClearLine,
    SubmitInput,
    TextCursorLeft,
    TextCursorRight,
    TextCursorLineStart,
    TextCursorLineEnd,
    TextCursorWordLeft,
    TextCursorWordRight,

    // Comment type
    CycleCommentType,
    CycleCommentTypeReverse,

    // Confirm dialog
    ConfirmYes,
    ConfirmNo,

    // Commit selection
    CommitSelectUp,
    CommitSelectDown,
    ToggleCommitSelect,
    ConfirmCommitSelect,
    /// Cycle inline commit selector to next individual commit (`)`)
    CycleCommitNext,
    /// Cycle inline commit selector to previous individual commit (`(`)
    CycleCommitPrev,

    ExpandAll,
    CollapseAll,
    SelectFileFull,

    // No-op
    None,
}

pub fn map_key_to_action(key: KeyEvent, mode: InputMode) -> Action {
    match mode {
        InputMode::Normal => map_normal_mode(key),
        InputMode::Command => map_command_mode(key),
        InputMode::Search => map_search_mode(key),
        InputMode::Comment => map_comment_mode(key),
        InputMode::Help => map_help_mode(key),
        InputMode::Confirm => map_confirm_mode(key),
        InputMode::CommitSelect => map_commit_select_mode(key),
        InputMode::VisualSelect => map_visual_mode(key),
        InputMode::FilePicker => map_file_picker_mode(key),
    }
}

fn map_normal_mode(key: KeyEvent) -> Action {
    match (key.code, key.modifiers) {
        // Cursor movement (Helix-style: cursor moves, scroll follows when needed)
        (KeyCode::Char('j') | KeyCode::Down, KeyModifiers::NONE) => Action::CursorDown(1),
        (KeyCode::Char('k') | KeyCode::Up, KeyModifiers::NONE) => Action::CursorUp(1),
        (KeyCode::Char('e'), KeyModifiers::CONTROL) => Action::ScrollViewDown(1),
        (KeyCode::Char('y'), KeyModifiers::CONTROL) => Action::ScrollViewUp(1),
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => Action::HalfPageDown,
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => Action::HalfPageUp,
        (KeyCode::Char('f'), KeyModifiers::CONTROL) => Action::PageDown,
        (KeyCode::Char('b'), KeyModifiers::CONTROL) => Action::PageUp,
        (KeyCode::PageDown, KeyModifiers::NONE) => Action::PageDown,
        (KeyCode::PageUp, KeyModifiers::NONE) => Action::PageUp,

        // Chord prefixes (Helix-style leaders)
        (KeyCode::Char('g'), KeyModifiers::NONE) => Action::PendingGCommand,
        (KeyCode::Char('z'), KeyModifiers::NONE) => Action::PendingZCommand,
        (KeyCode::Char('Z'), _) => Action::PendingShiftZCommand,
        (KeyCode::Char(' '), KeyModifiers::NONE) => Action::PendingSpaceCommand,
        (KeyCode::Char('['), _) => Action::PendingBracketLeftCommand,
        (KeyCode::Char(']'), _) => Action::PendingBracketRightCommand,

        // Inline commit cycle (kept from original)
        (KeyCode::Char(')'), _) => Action::CycleCommitNext,
        (KeyCode::Char('('), _) => Action::CycleCommitPrev,

        // Panel focus
        (KeyCode::Tab, KeyModifiers::NONE) => Action::ToggleFocus,
        (KeyCode::BackTab, _) => Action::ToggleFocusReverse,
        (KeyCode::Enter, KeyModifiers::NONE) => Action::SelectFile,
        (KeyCode::Enter, KeyModifiers::SHIFT) => Action::SelectFileFull,

        // Horizontal scrolling
        (KeyCode::Char('h') | KeyCode::Left, KeyModifiers::NONE) => Action::ScrollLeft(4),
        (KeyCode::Char('l') | KeyCode::Right, KeyModifiers::NONE) => Action::ScrollRight(4),

        // Review actions
        (KeyCode::Char('r'), KeyModifiers::NONE) => Action::ToggleReviewed,
        (KeyCode::Char('c'), KeyModifiers::NONE) => Action::AddLineComment,
        (KeyCode::Char('C'), _) => Action::AddFileComment,
        (KeyCode::Char('i'), KeyModifiers::NONE) => Action::EditComment,
        (KeyCode::Char('d'), KeyModifiers::NONE) => Action::PendingDCommand,
        (KeyCode::Char('v') | KeyCode::Char('V'), _) => Action::EnterVisualMode,
        (KeyCode::Char('y'), KeyModifiers::NONE) => Action::ExportToClipboard,
        (KeyCode::Char('n'), KeyModifiers::NONE) => Action::SearchNext,
        (KeyCode::Char('N'), _) => Action::SearchPrev,

        // Mode changes (use _ for shifted characters like : and ?)
        (KeyCode::Char(':'), _) => Action::EnterCommandMode,
        (KeyCode::Char('/'), _) => Action::EnterSearchMode,
        (KeyCode::Char('?'), _) => Action::ToggleHelp,
        (KeyCode::Esc, KeyModifiers::NONE) => Action::ExitMode,

        // Quick quit
        (KeyCode::Char('q'), KeyModifiers::NONE) => Action::Quit,

        // Tree expansion (file list panel)
        (KeyCode::Char('o'), KeyModifiers::NONE) => Action::ExpandAll,
        (KeyCode::Char('O'), _) => Action::CollapseAll,

        (KeyCode::Char(c @ '0'..='9'), KeyModifiers::NONE) => Action::Digit(c as u8 - b'0'),

        _ => Action::None,
    }
}

fn map_file_picker_mode(key: KeyEvent) -> Action {
    match (key.code, key.modifiers) {
        (KeyCode::Esc, KeyModifiers::NONE) => Action::ExitMode,
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => Action::ExitMode,
        (KeyCode::Enter, KeyModifiers::NONE) => Action::SubmitInput,
        (KeyCode::Up, _) => Action::CursorUp(1),
        (KeyCode::Down, _) => Action::CursorDown(1),
        // Tab / Shift-Tab cycle the result list (Helix picker convention)
        (KeyCode::Tab, KeyModifiers::NONE) => Action::CursorDown(1),
        (KeyCode::BackTab, _) => Action::CursorUp(1),
        (KeyCode::Char('k'), KeyModifiers::CONTROL) => Action::CursorUp(1),
        (KeyCode::Char('j'), KeyModifiers::CONTROL) => Action::CursorDown(1),
        (KeyCode::Char('p'), KeyModifiers::CONTROL) => Action::CursorUp(1),
        (KeyCode::Char('n'), KeyModifiers::CONTROL) => Action::CursorDown(1),
        (KeyCode::Backspace, mods) if mods.contains(KeyModifiers::ALT) => Action::DeleteWord,
        (KeyCode::Backspace, KeyModifiers::NONE) => Action::DeleteChar,
        (KeyCode::Char('w'), KeyModifiers::CONTROL) => Action::DeleteWord,
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => Action::ClearLine,
        (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => Action::InsertChar(c),
        _ => Action::None,
    }
}

fn map_command_mode(key: KeyEvent) -> Action {
    match (key.code, key.modifiers) {
        (KeyCode::Esc, KeyModifiers::NONE) => Action::ExitMode,
        (KeyCode::Enter, KeyModifiers::NONE) => Action::SubmitInput,
        (KeyCode::Backspace, mods) if mods.contains(KeyModifiers::ALT) => Action::DeleteWord,
        (KeyCode::Backspace, KeyModifiers::NONE) => Action::DeleteChar,
        (KeyCode::Char('w'), KeyModifiers::CONTROL) => Action::DeleteWord,
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => Action::ClearLine,
        (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => Action::InsertChar(c),
        _ => Action::None,
    }
}

fn map_search_mode(key: KeyEvent) -> Action {
    match (key.code, key.modifiers) {
        (KeyCode::Esc, KeyModifiers::NONE) => Action::ExitMode,
        (KeyCode::Enter, KeyModifiers::NONE) => Action::SubmitInput,
        (KeyCode::Backspace, mods) if mods.contains(KeyModifiers::ALT) => Action::DeleteWord,
        (KeyCode::Backspace, KeyModifiers::NONE) => Action::DeleteChar,
        (KeyCode::Char('w'), KeyModifiers::CONTROL) => Action::DeleteWord,
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => Action::ClearLine,
        (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => Action::InsertChar(c),
        _ => Action::None,
    }
}

fn map_comment_mode(key: KeyEvent) -> Action {
    match (key.code, key.modifiers) {
        // Cancel: Esc, Ctrl+C
        (KeyCode::Esc, KeyModifiers::NONE) => Action::ExitMode,
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => Action::ExitMode,
        // Submit: Enter without shift (Ctrl+Enter and Ctrl+S also work)
        (KeyCode::Enter, KeyModifiers::NONE) => Action::SubmitInput,
        (KeyCode::Enter, KeyModifiers::CONTROL) => Action::SubmitInput,
        (KeyCode::Char('s'), KeyModifiers::CONTROL) => Action::SubmitInput,
        // Newline: Shift+Enter (modern terminals) or Ctrl+J (universal fallback)
        (KeyCode::Enter, mods) if mods.contains(KeyModifiers::SHIFT) => Action::InsertChar('\n'),
        (KeyCode::Char('j'), KeyModifiers::CONTROL) => Action::InsertChar('\n'),
        // Comment type: Tab to cycle
        (KeyCode::Tab, KeyModifiers::NONE) => Action::CycleCommentType,
        (KeyCode::BackTab, _) => Action::CycleCommentTypeReverse,
        // Cursor movement
        (KeyCode::Char('a'), KeyModifiers::CONTROL) => Action::TextCursorLineStart,
        (KeyCode::Char('e'), KeyModifiers::CONTROL) => Action::TextCursorLineEnd,
        (KeyCode::Left, mods)
            if mods.contains(KeyModifiers::ALT) || mods.contains(KeyModifiers::CONTROL) =>
        {
            Action::TextCursorWordLeft
        }
        (KeyCode::Right, mods)
            if mods.contains(KeyModifiers::ALT) || mods.contains(KeyModifiers::CONTROL) =>
        {
            Action::TextCursorWordRight
        }
        (KeyCode::Home, _) => Action::TextCursorLineStart,
        (KeyCode::End, _) => Action::TextCursorLineEnd,
        (KeyCode::Left, mods)
            if mods.contains(KeyModifiers::SUPER) || mods.contains(KeyModifiers::META) =>
        {
            Action::TextCursorLineStart
        }
        (KeyCode::Right, mods)
            if mods.contains(KeyModifiers::SUPER) || mods.contains(KeyModifiers::META) =>
        {
            Action::TextCursorLineEnd
        }
        (KeyCode::Left, KeyModifiers::NONE) => Action::TextCursorLeft,
        (KeyCode::Right, KeyModifiers::NONE) => Action::TextCursorRight,
        // Editing
        (KeyCode::Backspace, mods)
            if mods.contains(KeyModifiers::ALT)
                || mods.contains(KeyModifiers::SUPER)
                || mods.contains(KeyModifiers::META) =>
        {
            Action::DeleteWord
        }
        (KeyCode::Backspace, KeyModifiers::NONE) => Action::DeleteChar,
        (KeyCode::Char('w'), KeyModifiers::CONTROL) => Action::DeleteWord,
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => Action::ClearLine,
        (KeyCode::Char(c), _) => Action::InsertChar(c),
        _ => Action::None,
    }
}

fn map_help_mode(key: KeyEvent) -> Action {
    match (key.code, key.modifiers) {
        // Close help
        (KeyCode::Esc, KeyModifiers::NONE)
        | (KeyCode::Char('q'), KeyModifiers::NONE)
        | (KeyCode::Char('?'), _) => Action::ToggleHelp,
        // Scroll navigation
        (KeyCode::Char('j') | KeyCode::Down, KeyModifiers::NONE) => Action::CursorDown(1),
        (KeyCode::Char('k') | KeyCode::Up, KeyModifiers::NONE) => Action::CursorUp(1),
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => Action::HalfPageDown,
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => Action::HalfPageUp,
        (KeyCode::Char('f'), KeyModifiers::CONTROL) => Action::PageDown,
        (KeyCode::Char('b'), KeyModifiers::CONTROL) => Action::PageUp,
        (KeyCode::PageDown, KeyModifiers::NONE) => Action::PageDown,
        (KeyCode::PageUp, KeyModifiers::NONE) => Action::PageUp,
        (KeyCode::Char('g'), KeyModifiers::NONE) => Action::GoToTop,
        (KeyCode::Char('G'), _) => Action::GoToBottom,
        _ => Action::None,
    }
}

fn map_confirm_mode(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => Action::ConfirmYes,
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => Action::ConfirmNo,
        _ => Action::None,
    }
}

fn map_commit_select_mode(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Action::CommitSelectDown,
        KeyCode::Char('k') | KeyCode::Up => Action::CommitSelectUp,
        KeyCode::Char(' ') => Action::ToggleCommitSelect,
        KeyCode::Enter => Action::ConfirmCommitSelect,
        KeyCode::Esc => Action::ExitMode,
        KeyCode::Char('q') => Action::Quit,
        _ => Action::None,
    }
}

fn map_visual_mode(key: KeyEvent) -> Action {
    match (key.code, key.modifiers) {
        // Extend selection
        (KeyCode::Char('j') | KeyCode::Down, KeyModifiers::NONE) => Action::CursorDown(1),
        (KeyCode::Char('k') | KeyCode::Up, KeyModifiers::NONE) => Action::CursorUp(1),
        (KeyCode::Char('c'), KeyModifiers::NONE) => Action::AddRangeComment,
        (KeyCode::Enter, KeyModifiers::NONE) => Action::AddRangeComment,
        (KeyCode::Char('y'), KeyModifiers::NONE) => Action::ExportToClipboard,
        (KeyCode::Esc, KeyModifiers::NONE) => Action::ExitMode,
        (KeyCode::Char('v') | KeyCode::Char('V'), _) => Action::ExitMode,
        (KeyCode::Char('q'), KeyModifiers::NONE) => Action::Quit,
        _ => Action::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn key_shift(c: char) -> KeyEvent {
        KeyEvent::new(KeyCode::Char(c), KeyModifiers::SHIFT)
    }

    #[test]
    fn should_map_digit_keys_to_digit_action_in_normal_mode() {
        for d in 0..=9u8 {
            let c = (b'0' + d) as char;
            let action = map_normal_mode(key(KeyCode::Char(c)));
            assert_eq!(
                action,
                Action::Digit(d),
                "digit key '{c}' should map to Digit({d})"
            );
        }
    }

    #[test]
    fn should_map_lowercase_g_to_pending_g_chord_in_normal_mode() {
        let action = map_normal_mode(key(KeyCode::Char('g')));
        assert_eq!(action, Action::PendingGCommand);
    }

    #[test]
    fn should_map_space_to_pending_space_chord_in_normal_mode() {
        let action = map_normal_mode(key(KeyCode::Char(' ')));
        assert_eq!(action, Action::PendingSpaceCommand);
    }

    #[test]
    fn should_map_bracket_chords_in_normal_mode() {
        let action = map_normal_mode(key(KeyCode::Char('[')));
        assert_eq!(action, Action::PendingBracketLeftCommand);
        let action = map_normal_mode(key(KeyCode::Char(']')));
        assert_eq!(action, Action::PendingBracketRightCommand);
    }

    #[test]
    fn shift_g_is_no_longer_a_bottom_alias() {
        // GoToBottom now reached via `ge`, dispatched in main.rs chord state.
        let action = map_normal_mode(key_shift('G'));
        assert_ne!(action, Action::GoToBottom);
    }

    #[test]
    fn should_not_map_digits_in_command_mode() {
        for d in 0..=9u8 {
            let c = (b'0' + d) as char;
            let action = map_command_mode(key(KeyCode::Char(c)));
            assert_eq!(
                action,
                Action::InsertChar(c),
                "digit '{c}' in command mode should be InsertChar"
            );
        }
    }

    #[test]
    fn should_not_map_digits_in_search_mode() {
        for d in 0..=9u8 {
            let c = (b'0' + d) as char;
            let action = map_search_mode(key(KeyCode::Char(c)));
            assert_eq!(
                action,
                Action::InsertChar(c),
                "digit '{c}' in search mode should be InsertChar"
            );
        }
    }

    #[test]
    fn should_not_map_shifted_digits_to_digit_action() {
        // Shift+digit produces characters like !, @, #, etc. on most layouts,
        // but if a terminal sends the raw digit with SHIFT modifier it must not
        // be treated as Action::Digit.
        for d in 0..=9u8 {
            let c = (b'0' + d) as char;
            let action = map_normal_mode(key_shift(c));
            assert_ne!(
                action,
                Action::Digit(d),
                "Shift+'{c}' in normal mode must not produce Digit({d})"
            );
        }
    }

    #[test]
    fn should_map_backtab_to_reverse_focus_in_normal_mode() {
        let action = map_normal_mode(KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT));
        assert_eq!(action, Action::ToggleFocusReverse);
    }

    #[test]
    fn should_map_backtab_to_reverse_comment_type_in_comment_mode() {
        let action = map_comment_mode(KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT));
        assert_eq!(action, Action::CycleCommentTypeReverse);
    }

    #[test]
    fn should_map_alt_backspace_to_delete_word_in_text_input_modes() {
        let alt_backspace = KeyEvent::new(KeyCode::Backspace, KeyModifiers::ALT);
        assert_eq!(map_comment_mode(alt_backspace), Action::DeleteWord);
        assert_eq!(map_command_mode(alt_backspace), Action::DeleteWord);
        assert_eq!(map_search_mode(alt_backspace), Action::DeleteWord);
        assert_eq!(map_file_picker_mode(alt_backspace), Action::DeleteWord);
    }

    #[test]
    fn file_picker_mode_maps_text_input_and_navigation() {
        assert_eq!(map_file_picker_mode(key(KeyCode::Esc)), Action::ExitMode);
        assert_eq!(map_file_picker_mode(key(KeyCode::Enter)), Action::SubmitInput);
        assert_eq!(
            map_file_picker_mode(key(KeyCode::Char('a'))),
            Action::InsertChar('a')
        );
        assert_eq!(
            map_file_picker_mode(key(KeyCode::Backspace)),
            Action::DeleteChar
        );
        assert_eq!(map_file_picker_mode(key(KeyCode::Up)), Action::CursorUp(1));
        assert_eq!(
            map_file_picker_mode(key(KeyCode::Down)),
            Action::CursorDown(1)
        );
        let ctrl_j = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::CONTROL);
        assert_eq!(map_file_picker_mode(ctrl_j), Action::CursorDown(1));

        // Tab / Shift-Tab cycle the result list (Helix picker convention).
        assert_eq!(map_file_picker_mode(key(KeyCode::Tab)), Action::CursorDown(1));
        let shift_tab = KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT);
        assert_eq!(map_file_picker_mode(shift_tab), Action::CursorUp(1));
    }

    #[test]
    fn no_key_should_produce_mouse_scroll_actions() {
        let codes = [
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::PageDown,
            KeyCode::PageUp,
            KeyCode::Char('j'),
            KeyCode::Char('k'),
            KeyCode::Char('e'),
            KeyCode::Char('y'),
        ];
        let mod_sets = [
            KeyModifiers::NONE,
            KeyModifiers::CONTROL,
            KeyModifiers::ALT,
            KeyModifiers::SHIFT,
        ];
        for code in codes {
            for mods in mod_sets {
                let ev = KeyEvent::new(code, mods);
                for action in [
                    map_normal_mode(ev),
                    map_command_mode(ev),
                    map_search_mode(ev),
                    map_comment_mode(ev),
                    map_help_mode(ev),
                ] {
                    assert!(
                        !matches!(
                            action,
                            Action::MouseScrollUp(_) | Action::MouseScrollDown(_)
                        ),
                        "key {code:?} + {mods:?} produced a mouse-scroll action"
                    );
                }
            }
        }
    }
}
