// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use super::key::Key;
use crossterm::event::{KeyCode, KeyModifiers};

pub mod common {
    use super::*;

    pub const QUIT_1: Key = Key {
        code: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    };

    pub const QUIT_2: Key = Key {
        code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    };

    pub const RELOAD: Key = Key {
        code: KeyCode::Char('r'),
        modifiers: KeyModifiers::CONTROL,
    };

    pub const TOP: Key = Key {
        code: KeyCode::Char('g'),
        modifiers: KeyModifiers::NONE,
    };
    pub const BOTTOM: Key = Key {
        code: KeyCode::Char('G'),
        modifiers: KeyModifiers::SHIFT,
    };

    #[cfg(not(feature = "alternative_keybinding"))]
    pub const UP: Key = Key {
        code: KeyCode::Char('k'),
        modifiers: KeyModifiers::NONE,
    };
    #[cfg(feature = "alternative_keybinding")]
    pub const UP: Key = Key {
        code: KeyCode::Char('i'),
        modifiers: KeyModifiers::NONE,
    };

    #[cfg(not(feature = "alternative_keybinding"))]
    pub const DOWN: Key = Key {
        code: KeyCode::Char('j'),
        modifiers: KeyModifiers::NONE,
    };
    #[cfg(feature = "alternative_keybinding")]
    pub const DOWN: Key = Key {
        code: KeyCode::Char('k'),
        modifiers: KeyModifiers::NONE,
    };

    #[cfg(not(feature = "alternative_keybinding"))]
    pub const UP_FAST: Key = Key {
        code: KeyCode::Char('k'),
        modifiers: KeyModifiers::CONTROL,
    };
    #[cfg(feature = "alternative_keybinding")]
    pub const UP_FAST: Key = Key {
        code: KeyCode::Tab,
        modifiers: KeyModifiers::NONE,
        // code: KeyCode::Char('i'),
        // modifiers: KeyModifiers::CONTROL,
    };

    #[cfg(not(feature = "alternative_keybinding"))]
    pub const DOWN_FAST: Key = Key {
        code: KeyCode::Char('j'),
        modifiers: KeyModifiers::CONTROL,
    };
    #[cfg(feature = "alternative_keybinding")]
    pub const DOWN_FAST: Key = Key {
        code: KeyCode::Char('k'),
        modifiers: KeyModifiers::CONTROL,
    };
}

pub mod view {
    use super::*;

    pub const EDIT_MODE: Key = Key {
        code: KeyCode::Char('v'),
        modifiers: KeyModifiers::NONE,
    };
}

pub mod visibility {
    use super::*;

    pub const TOGGLE: Key = Key {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    };

    pub const VIEW_MODE: Key = Key {
        code: KeyCode::Char('v'),
        modifiers: KeyModifiers::NONE,
    };

    pub const SEARCH_MODE: Key = Key {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
    };
}

pub mod search {
    use super::*;

    pub const EDIT_MODE: Key = Key {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    };

    pub const DEL_CHAR_1: Key = Key {
        code: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
    };

    pub const DEL_CHAR_2: Key = Key {
        code: KeyCode::Char('h'),
        modifiers: KeyModifiers::CONTROL,
    };
}
