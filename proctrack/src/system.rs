// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

use self::app::App;
use self::event::{Event, EventHandler};
use self::handler::handle_key_events;
use self::tui::Tui;
use crate::visibility_info::VisibilityInfo;
use ::tui::backend::CrosstermBackend;
use ::tui::Terminal;
use anyhow::Result;
use std::io;
use std::path::PathBuf;

pub fn activate(
    debug_info_file: PathBuf,
    visibility_info_file: PathBuf,
    debug_info: String,
    visibility_info: VisibilityInfo,
) -> Result<()> {
    // Create an application.
    let mut app = App::new(
        debug_info_file,
        visibility_info_file,
        debug_info,
        visibility_info,
    );

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick()?,
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;

    app.update_visibility_info_file()
}
