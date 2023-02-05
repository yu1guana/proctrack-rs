// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use super::handler::guidance_string;
use crate::visibility_info::VisibilityInfo;
use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Wrap};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AppMode {
    ViewDebug,
    EditVisibility,
    SearchVisibility,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    pub mode: AppMode,

    guidance: String,
    pub search_string: String,
    search_regex: Regex,
    search_regex_error: String,
    pub visibility_hash_map: HashMap<String, bool>,
    regex_value_line: Regex,

    scroll: (u16, u16),
    pub idx_visibility: usize,

    pub debug_info_file: PathBuf,
    pub visibility_info_file: PathBuf,
    pub debug_info: String,
    pub visibility_info: VisibilityInfo,

    string_buffer: String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(
        debug_info_file: PathBuf,
        visibility_info_file: PathBuf,
        debug_info: String,
        visibility_info: VisibilityInfo,
    ) -> Self {
        Self {
            running: true,
            mode: AppMode::ViewDebug,
            guidance: guidance_string(AppMode::ViewDebug),
            search_string: String::new(),
            search_regex: Regex::new("").unwrap(),
            search_regex_error: String::new(),
            visibility_hash_map: visibility_info.clone().into(),
            regex_value_line: Regex::new(r"\[DEBUG:value\(.*\)] ").unwrap(),
            scroll: (0, 0),
            idx_visibility: 0,
            debug_info_file,
            visibility_info_file,
            debug_info,
            visibility_info,
            string_buffer: String::new(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) -> Result<()> {
        Ok(())
    }

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let guidance_height = 3 + (self.guidance.len() / frame.size().width as usize) as u16;
        let search_box_height = 3;
        let visibility_info_width = match self.mode {
            AppMode::ViewDebug => 0,
            _ => frame.size().width / 2,
        };
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(guidance_height),
                    Constraint::Length(frame.size().height - guidance_height),
                ]
                .as_ref(),
            )
            .split(frame.size());
        let chunks_1 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(frame.size().width - visibility_info_width),
                    Constraint::Length(visibility_info_width),
                ]
                .as_ref(),
            )
            .split(chunks[1]);
        let chunks_1_1 = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(search_box_height),
                    Constraint::Length(frame.size().height - search_box_height),
                ]
                .as_ref(),
            )
            .split(chunks_1[1]);

        let chunk_guidance = chunks[0];
        let chunk_debug_info = chunks_1[0];
        let chunk_search_box = chunks_1_1[0];
        let chunk_visibility_info = chunks_1_1[1];

        self.render_guidance(frame, chunk_guidance);
        self.render_debug_info(frame, chunk_debug_info);
        self.render_search_box(frame, chunk_search_box);
        self.render_visibility_info(frame, chunk_visibility_info);
    }

    fn render_guidance<B: Backend>(&self, frame: &mut Frame<B>, chunk: Rect) {
        frame.render_widget(
            Paragraph::new(self.guidance.as_ref())
                .block(Block::default().borders(Borders::ALL))
                .wrap(Wrap { trim: false }),
            chunk,
        );
    }

    fn render_debug_info<B: Backend>(&mut self, frame: &mut Frame<B>, chunk: Rect) {
        macro_rules! write_depth_representation {
            ($string_buffer:expr, $depth:expr) => {
                for _ in 0..$depth {
                    write!($string_buffer, "| ").unwrap();
                }
            };
        }
        let mut displayed_debug_info = Vec::new();
        let mut display = true;
        let mut depth = 0;
        let mut display_depth = depth;
        for line in self.debug_info.lines() {
            if line.starts_with("[DEBUG:func_enter") {
                let func_name = line.split_ascii_whitespace().last().unwrap();
                if !self
                    .visibility_hash_map
                    .get(func_name)
                    .map_or(false, |visibility| *visibility)
                {
                    display = false;
                } else if display {
                    display_depth = depth
                }
                if display {
                    self.string_buffer.clear();
                    write_depth_representation!(&mut self.string_buffer, depth);
                    displayed_debug_info.push(Spans::from(vec![
                        Span::raw(" "),
                        Span::styled(
                            self.string_buffer.clone(),
                            Style::default().fg(Color::DarkGray),
                        ),
                        Span::raw(func_name),
                    ]));
                }
                depth += 1;
            } else if line.starts_with("[DEBUG:func_exit") {
                depth -= 1;
                if display_depth == depth {
                    display = true;
                }
            } else if display && line.starts_with("[DEBUG:value") {
                self.string_buffer.clear();
                write_depth_representation!(&mut self.string_buffer, depth);
                displayed_debug_info.push(Spans::from(vec![
                    Span::raw(" "),
                    Span::styled(
                        self.string_buffer.clone(),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(
                        format!("{}", self.regex_value_line.replace(line, ""),),
                        Style::default().fg(Color::Gray),
                    ),
                ]));
            }
        }
        frame.render_widget(
            Paragraph::new(displayed_debug_info)
                .block(Block::default().borders(Borders::ALL).title("DebugInfo"))
                .scroll(self.scroll),
            chunk,
        );
    }

    fn render_search_box<B: Backend>(&self, frame: &mut Frame<B>, chunk: Rect) {
        frame.render_widget(
            Paragraph::new(vec![Spans::from(vec![
                Span::raw(" "),
                Span::styled(
                    &self.search_string,
                    if self.mode == AppMode::SearchVisibility {
                        Style::default()
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ),
                Span::styled(
                    " ",
                    if self.mode == AppMode::SearchVisibility {
                        Style::default().bg(Color::Gray)
                    } else {
                        Style::default()
                    },
                ),
                if self.search_regex_error.is_empty() {
                    Span::raw(" ")
                } else {
                    Span::styled(
                        format!("   [Error: {}]", self.search_regex_error),
                        Style::default().fg(Color::Red),
                    )
                },
            ])])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Search")
                    .border_style(
                        Style::default().fg(if self.mode == AppMode::SearchVisibility {
                            Color::Reset
                        } else {
                            Color::DarkGray
                        }),
                    ),
            ),
            chunk,
        );
    }

    fn render_visibility_info<B: Backend>(&mut self, frame: &mut Frame<B>, chunk: Rect) {
        let rows = self
            .visibility_info
            .iter()
            .filter_map(|entry| {
                if self.search_regex.is_match(&entry.func_name) {
                    let style = if entry.visibility {
                        Style::default()
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };
                    Some(Row::new(vec![
                        Cell::from(entry.func_name.as_ref()).style(style)
                    ]))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let widths = vec![Constraint::Length(chunk.width)];
        let table = Table::new(rows).widths(&widths).highlight_symbol(" > ");
        let mut table_state = TableState::default();
        table_state.select(Some(self.idx_visibility));
        frame.render_stateful_widget(
            table.block(Block::default().borders(Borders::ALL).title("Visibility")),
            chunk,
            &mut table_state,
        );
    }

    fn num_displayed_debug_info_lines(&self) -> u16 {
        let mut count = 0;
        let mut display = true;
        let mut depth = 0;
        let mut display_depth = depth;
        for line in self.debug_info.lines() {
            if line.starts_with("[DEBUG:func_enter") {
                let func_name = line.split_ascii_whitespace().last().unwrap();
                if !self
                    .visibility_hash_map
                    .get(func_name)
                    .map_or(false, |visibility| *visibility)
                {
                    display = false;
                } else if display {
                    display_depth = depth
                }
                if display {
                    count += 1;
                }
                depth += 1;
            } else if line.starts_with("[DEBUG:func_exit") {
                depth -= 1;
                if display_depth == depth {
                    display = true;
                }
            } else if display && line.starts_with("[DEBUG:value") {
                count += 1;
            }
        }
        count
    }

    fn num_displayed_visibility_entries(&self) -> usize {
        self.visibility_info
            .iter()
            .filter(|entry| self.search_regex.is_match(&entry.func_name))
            .count()
    }

    pub fn update_visibility_info_file(&mut self) -> Result<()> {
        self.visibility_info
            .write_toml_file(&self.visibility_info_file)
    }

    pub fn update_visibility(&mut self) {
        if let Some(entry) = self
            .visibility_info
            .iter_mut()
            .filter(|entry| self.search_regex.is_match(&entry.func_name))
            .nth(self.idx_visibility)
        {
            entry.visibility ^= true;
        }
        self.visibility_hash_map = self.visibility_info.clone().into();
    }

    pub fn update_guidance(&mut self) {
        self.guidance = guidance_string(self.mode);
    }

    pub fn update_search_regex(&mut self) {
        self.search_regex_error.clear();
        match Regex::new(&self.search_string) {
            Ok(regex) => self.search_regex = regex,
            Err(err) => match err {
                regex::Error::Syntax(msg) => self.search_regex_error = msg,
                regex::Error::CompiledTooBig(_) => {
                    self.search_regex_error.push_str("compiled too big")
                }
                _ => self.search_regex_error.push_str("Some error"),
            },
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll.0 != 0 {
            self.scroll.0 -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.scroll.0 < self.num_displayed_debug_info_lines() - 1 {
            self.scroll.0 += 1;
        }
    }

    pub fn idx_visibility_up(&mut self) {
        if self.idx_visibility != 0 {
            self.idx_visibility -= 1;
        }
    }

    pub fn idx_visibility_down(&mut self) {
        if self.idx_visibility < self.num_displayed_visibility_entries() - 1 {
            self.idx_visibility += 1;
        }
    }
}
