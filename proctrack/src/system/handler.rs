// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use super::app::{App, AppMode};
use crate::visibility_info::VisibilityInfo;
use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::fs;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    match app.mode {
        AppMode::ViewDebug => {
            match key_event.code {
                // Exit application on `ESC`
                KeyCode::Esc => {
                    app.running = false;
                }
                // Exit application on `q`
                KeyCode::Char('q') => {
                    if key_event.modifiers == KeyModifiers::NONE {
                        app.running = false;
                    }
                }
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.running = false;
                    }
                }
                // Reload debug info file on `R`
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    if key_event.modifiers == KeyModifiers::SHIFT {
                        app.debug_info =
                            fs::read_to_string(&app.debug_info_file).with_context(|| {
                                format!("failed to read {}", app.debug_info_file.display())
                            })?;
                        app.visibility_info = VisibilityInfo::try_new(&app.visibility_info_file)?
                            .update_by_debug_info(&app.debug_info)?;
                        app.visibility_hash_map = app.visibility_info.clone().into();
                    }
                }
                // Open visibility info editor on `v`
                KeyCode::Char('v') => {
                    if key_event.modifiers == KeyModifiers::NONE {
                        app.mode = AppMode::EditVisibility;
                        app.update_guidance();
                    }
                }
                KeyCode::Char(keybinding::UP) => {
                    if key_event.modifiers == KeyModifiers::NONE {
                        app.scroll_up()
                    }
                }
                KeyCode::Char(keybinding::DOWN) => {
                    if key_event.modifiers == KeyModifiers::NONE {
                        app.scroll_down()
                    }
                }
                // Other handlers you could add here.
                _ => {}
            }
        }
        AppMode::EditVisibility => {
            match key_event.code {
                // Exit application on `ESC`
                KeyCode::Esc => {
                    app.running = false;
                }
                // Exit application on `q`
                KeyCode::Char('q') => {
                    if key_event.modifiers == KeyModifiers::NONE {
                        app.running = false;
                    }
                }
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.running = false;
                    }
                }
                // Reload debug info file on `R`
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    if key_event.modifiers == KeyModifiers::SHIFT {
                        app.debug_info =
                            fs::read_to_string(&app.debug_info_file).with_context(|| {
                                format!("failed to read {}", app.debug_info_file.display())
                            })?;
                        app.visibility_info = VisibilityInfo::try_new(&app.visibility_info_file)?
                            .update_by_debug_info(&app.debug_info)?;
                        app.visibility_hash_map = app.visibility_info.clone().into();
                    }
                }
                // Close visibility info editor on `v`
                KeyCode::Char('v') => {
                    if key_event.modifiers == KeyModifiers::NONE {
                        app.mode = AppMode::ViewDebug;
                        app.update_guidance();
                    }
                }
                KeyCode::Char(keybinding::UP) => {
                    if key_event.modifiers == KeyModifiers::NONE {
                        app.idx_visibility_up()
                    }
                }
                KeyCode::Char(keybinding::DOWN) => {
                    if key_event.modifiers == KeyModifiers::NONE {
                        app.idx_visibility_down()
                    }
                }
                KeyCode::Enter => {
                    app.update_visibility();
                }
                KeyCode::Char('f') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.mode = AppMode::SearchVisibility;
                        app.update_guidance();
                    }
                }
                // Other handlers you could add here.
                _ => {}
            }
        }
        AppMode::SearchVisibility => match key_event.code {
            // Exit application on `ESC`
            KeyCode::Esc => {
                app.running = false;
            }
            KeyCode::Enter => {
                app.mode = AppMode::EditVisibility;
                app.update_guidance();
            }
            KeyCode::Char(c) => match key_event.modifiers {
                KeyModifiers::CONTROL => {
                    if c == 'c' {
                        app.running = false;
                    }
                }
                KeyModifiers::NONE | KeyModifiers::SHIFT => {
                    app.idx_visibility = 0;
                    app.search_string.push(c);
                    app.update_search_regex();
                }
                _ => {}
            },
            KeyCode::Backspace => {
                app.search_string.pop();
                app.update_search_regex();
            }
            _ => {}
        },
    }

    Ok(())
}

pub fn guidance_string(app_mode: AppMode) -> String {
    match app_mode {
        AppMode::ViewDebug => format!(
            " Quit [q, Esc, Ctrl-c], Up/Down [{}/{}], Reload Debug Info File [r], Open Visibility Editor [v]",
            keybinding::UP,
            keybinding::DOWN
        ),
        AppMode::EditVisibility => format!(
            " Quit [q, Esc, Ctrl-c], Up/Down [{}/{}], Reload Debug Info File [r], Close Visibility Editor [v], Change Editor Visibility[Enter], Search [Ctrl-f]",
            keybinding::UP,
            keybinding::DOWN
        ),
        AppMode::SearchVisibility => " Quit [Esc, Ctrl-c], Finish searching [Enter]".to_string(),
    }
}

#[cfg(not(feature = "alternative_keybinding"))]
mod keybinding {
    pub const UP: char = 'k';
    pub const DOWN: char = 'j';
    // const LEFT: char = 'h';
    // const RIGHT: char = 'l';
}

#[cfg(feature = "alternative_keybinding")]
mod keybinding {
    pub const UP: char = 'i';
    pub const DOWN: char = 'k';
    // const LEFT: char = 'l';
    // const RIGHT: char = 'j';
}
