// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

mod key;
mod keybinding;

use super::app::{App, AppMode};
use crate::visibility_info::VisibilityInfo;
use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::fmt::Write;
use std::fs;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    let key = key_event.into();
    match key {
        keybinding::common::QUIT_1 | keybinding::common::QUIT_2 => {
            app.running = false;
            return Ok(());
        }
        keybinding::common::RELOAD => {
            app.debug_info = fs::read_to_string(&app.debug_info_file)
                .with_context(|| format!("failed to read {}", app.debug_info_file.display()))?;
            app.visibility_info = VisibilityInfo::try_new(&app.visibility_info_file)?
                .update_by_debug_info(&app.debug_info)?;
            app.visibility_hash_map = app.visibility_info.clone().into();
            return Ok(());
        }
        _ => (),
    }
    match app.mode {
        AppMode::ViewDebug => match key {
            keybinding::view::EDIT_MODE => app.mode_change(AppMode::EditVisibility),
            keybinding::common::UP => app.scroll_up(1),
            keybinding::common::DOWN => app.scroll_down(1),
            keybinding::common::UP_FAST => app.scroll_up(20),
            keybinding::common::DOWN_FAST => app.scroll_down(20),
            keybinding::common::TOP => app.scroll_up(u16::MAX),
            keybinding::common::BOTTOM => app.scroll_down(u16::MAX),
            _ => (),
        },
        AppMode::EditVisibility => match key {
            keybinding::visibility::VIEW_MODE => app.mode_change(AppMode::ViewDebug),
            keybinding::visibility::SEARCH_MODE => app.mode_change(AppMode::SearchVisibility),
            keybinding::common::UP => app.idx_visibility_prev(1),
            keybinding::common::DOWN => app.idx_visibility_next(1),
            keybinding::common::UP_FAST => app.idx_visibility_prev(20),
            keybinding::common::DOWN_FAST => app.idx_visibility_next(20),
            keybinding::common::TOP => app.idx_visibility_prev(usize::MAX),
            keybinding::common::BOTTOM => app.idx_visibility_next(usize::MAX),
            keybinding::visibility::TOGGLE => app.update_visibility(),
            _ => (),
        },
        AppMode::SearchVisibility => {
            match key {
                keybinding::search::EDIT_MODE => {
                    app.mode_change(AppMode::EditVisibility);
                    return Ok(());
                }
                keybinding::search::DEL_CHAR_1 | keybinding::search::DEL_CHAR_2 => {
                    if app.search_string.is_empty() {
                        app.mode_change(AppMode::EditVisibility);
                    } else {
                        app.search_string.pop();
                        app.update_search_regex();
                    }
                    return Ok(());
                }
                _ => (),
            }
            if let KeyCode::Char(c) = key.code {
                if key.modifiers == KeyModifiers::NONE || key.modifiers == KeyModifiers::SHIFT {
                    app.idx_visibility = 0;
                    app.search_string.push(c);
                    app.update_search_regex();
                }
            }
        }
    }
    Ok(())
}

pub fn set_guidance(app_mode: AppMode, guidance: &mut String) {
    guidance.clear();
    write!(
        guidance,
        " Quit [{}, {}],",
        keybinding::common::QUIT_1,
        keybinding::common::QUIT_2
    )
    .unwrap();
    write!(guidance, " Reload [{}],", keybinding::common::RELOAD,).unwrap();

    match app_mode {
        AppMode::ViewDebug => {
            writeln!(
                guidance,
                " Open Visibility Editor [{}]",
                keybinding::view::EDIT_MODE,
            )
            .unwrap();
        }
        AppMode::EditVisibility => {
            write!(
                guidance,
                " Close Visibility Editor [{}],",
                keybinding::visibility::VIEW_MODE,
            )
            .unwrap();
            writeln!(
                guidance,
                " Search [{}]",
                keybinding::visibility::SEARCH_MODE,
            )
            .unwrap();
        }
        AppMode::SearchVisibility => {
            write!(
                guidance,
                " Finish searching [{}]",
                keybinding::search::EDIT_MODE,
            )
            .unwrap();
        }
    }

    match app_mode {
        AppMode::ViewDebug | AppMode::EditVisibility => {
            write!(
                guidance,
                " Up/Down [{}/{}],",
                keybinding::common::UP,
                keybinding::common::DOWN
            )
            .unwrap();
            write!(
                guidance,
                " Up/Down Fast [{}/{}],",
                keybinding::common::UP_FAST,
                keybinding::common::DOWN_FAST
            )
            .unwrap();
            write!(
                guidance,
                " Top/Bottom [{}/{}]",
                keybinding::common::TOP,
                keybinding::common::BOTTOM
            )
            .unwrap();
        }
        _ => (),
    }

    if app_mode == AppMode::EditVisibility {
        write!(
            guidance,
            ", Change Visibility [{}]",
            keybinding::visibility::TOGGLE,
        )
        .unwrap();
    }
}
