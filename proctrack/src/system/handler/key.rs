// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MediaKeyCode, ModifierKeyCode};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Key {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        Self {
            code: event.code,
            modifiers: event.modifiers,
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            write!(f, "Shift-")?;
        };
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            write!(f, "Ctrl-")?;
        };
        if self.modifiers.contains(KeyModifiers::ALT) {
            write!(f, "Alt-")?;
        };
        if self.modifiers.contains(KeyModifiers::SUPER) {
            write!(f, "Super-")?;
        };
        if self.modifiers.contains(KeyModifiers::HYPER) {
            write!(f, "Hyper-")?;
        };
        if self.modifiers.contains(KeyModifiers::META) {
            write!(f, "Meta-")?;
        };
        match self.code {
            KeyCode::Backspace => write!(f, "BS")?,
            KeyCode::Enter => write!(f, "Enter")?,
            KeyCode::Left => write!(f, "Left")?,
            KeyCode::Right => write!(f, "Right")?,
            KeyCode::Up => write!(f, "Up")?,
            KeyCode::Down => write!(f, "Down")?,
            KeyCode::Home => write!(f, "Home")?,
            KeyCode::End => write!(f, "End")?,
            KeyCode::PageUp => write!(f, "PageUp")?,
            KeyCode::PageDown => write!(f, "PageDown")?,
            KeyCode::Tab => write!(f, "Tab")?,
            KeyCode::BackTab => write!(f, "BackTab")?,
            KeyCode::Delete => write!(f, "Del")?,
            KeyCode::Insert => write!(f, "Insert")?,
            KeyCode::F(x) => write!(f, "F{}", x)?,
            KeyCode::Char(c) => write!(f, "{}", c)?,
            KeyCode::Null => write!(f, "Null")?,
            KeyCode::Esc => write!(f, "Esc")?,
            KeyCode::CapsLock => write!(f, "CapsLock")?,
            KeyCode::ScrollLock => write!(f, "ScrollLock")?,
            KeyCode::NumLock => write!(f, "NumLock")?,
            KeyCode::PrintScreen => write!(f, "PrintScreen")?,
            KeyCode::Pause => write!(f, "Pause")?,
            KeyCode::Menu => write!(f, "Menu")?,
            KeyCode::KeypadBegin => write!(f, "KeypadBegin")?,
            KeyCode::Media(code) => match code {
                MediaKeyCode::Play => write!(f, "Play")?,
                MediaKeyCode::Pause => write!(f, "Pause")?,
                MediaKeyCode::PlayPause => write!(f, "PlayPause")?,
                MediaKeyCode::Reverse => write!(f, "Reverse")?,
                MediaKeyCode::Stop => write!(f, "Stop")?,
                MediaKeyCode::FastForward => write!(f, "FastForward")?,
                MediaKeyCode::Rewind => write!(f, "Rewind")?,
                MediaKeyCode::TrackNext => write!(f, "TrackNext")?,
                MediaKeyCode::TrackPrevious => write!(f, "TrackPrevious")?,
                MediaKeyCode::Record => write!(f, "Record")?,
                MediaKeyCode::LowerVolume => write!(f, "LowerVolume")?,
                MediaKeyCode::RaiseVolume => write!(f, "RaiseVolume")?,
                MediaKeyCode::MuteVolume => write!(f, "MuteVolume")?,
            },
            KeyCode::Modifier(code) => match code {
                ModifierKeyCode::LeftShift => write!(f, "LeftShift")?,
                ModifierKeyCode::LeftControl => write!(f, "LeftControl")?,
                ModifierKeyCode::LeftAlt => write!(f, "LeftAlt")?,
                ModifierKeyCode::LeftSuper => write!(f, "LeftSuper")?,
                ModifierKeyCode::LeftHyper => write!(f, "LeftHyper")?,
                ModifierKeyCode::LeftMeta => write!(f, "LeftMeta")?,
                ModifierKeyCode::RightShift => write!(f, "RightShift")?,
                ModifierKeyCode::RightControl => write!(f, "RightControl")?,
                ModifierKeyCode::RightAlt => write!(f, "RightAlt")?,
                ModifierKeyCode::RightSuper => write!(f, "RightSuper")?,
                ModifierKeyCode::RightHyper => write!(f, "RightHyper")?,
                ModifierKeyCode::RightMeta => write!(f, "RightMeta")?,
                ModifierKeyCode::IsoLevel3Shift => write!(f, "IsoLevel3Shift")?,
                ModifierKeyCode::IsoLevel5Shift => write!(f, "IsoLevel5Shift")?,
            },
        }
        Ok(())
    }
}
