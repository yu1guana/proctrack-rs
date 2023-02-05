// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use anyhow::{bail, Context, Result};
use proctrack::funclog::{methodlog, methodlog_move, methodlog_static};
use proctrack::typename_derive::TypeName;
use serde_derive::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::fs::{self, File};
use std::ops::{Deref, DerefMut};
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, TypeName)]
pub struct VisibilityEntry {
    pub func_name: String,
    pub visibility: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, TypeName)]
pub struct VisibilityInfo {
    entries: Vec<VisibilityEntry>,
}

impl From<(String, bool)> for VisibilityEntry {
    fn from((func_name, visibility): (String, bool)) -> Self {
        Self {
            func_name,
            visibility,
        }
    }
}

impl Deref for VisibilityInfo {
    type Target = Vec<VisibilityEntry>;
    fn deref(&self) -> &Vec<VisibilityEntry> {
        &self.entries
    }
}

impl DerefMut for VisibilityInfo {
    fn deref_mut(&mut self) -> &mut Vec<VisibilityEntry> {
        &mut self.entries
    }
}

impl From<VisibilityInfo> for HashMap<String, bool> {
    fn from(visible_info: VisibilityInfo) -> Self {
        Self::from_iter(
            visible_info
                .entries
                .into_iter()
                .map(|entry| (entry.func_name, entry.visibility)),
        )
    }
}
impl From<Vec<VisibilityEntry>> for VisibilityInfo {
    fn from(entries: Vec<VisibilityEntry>) -> Self {
        Self { entries }
    }
}

impl From<HashMap<String, bool>> for VisibilityInfo {
    fn from(hash_map: HashMap<String, bool>) -> Self {
        Self {
            entries: hash_map
                .into_iter()
                .map(|(func_name, visibility)| VisibilityEntry {
                    func_name,
                    visibility,
                })
                .collect(),
        }
    }
}

impl PartialOrd for VisibilityEntry {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.func_name.partial_cmp(&rhs.func_name)
    }
}

impl Ord for VisibilityEntry {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.func_name.cmp(&rhs.func_name)
    }
}

impl VisibilityInfo {
    pub fn try_new(file: &Path) -> Result<Self> {
        if file.is_file() {
            if File::open(file)
                .with_context(|| format!("failed to open {}", file.display()))?
                .metadata()
                .with_context(|| format!("failed to get metadata of {}", file.display()))?
                .permissions()
                .readonly()
            {
                bail!("visibility_info file must be writable.")
            } else {
                VisibilityInfo::read_toml_file(file)
            }
        } else {
            Ok(VisibilityInfo::default())
        }
    }

    #[methodlog]
    pub fn write_toml_file(&self, file: &Path) -> Result<()> {
        fs::write(
            file,
            toml::to_string(&self).context("failed to change toml into string")?,
        )
        .with_context(|| format!("failed to write toml into {}", file.display()))
    }

    pub fn read_toml_file(file: &Path) -> Result<Self> {
        toml::de::from_str(
            &fs::read_to_string(file)
                .with_context(|| format!("failed to read {}", file.display()))?,
        )
        .map_err(Into::into)
    }

    pub fn update_by_debug_info(self, debug_info: &str) -> Result<Self> {
        let mut old_visibility_info = HashMap::from(self);
        let mut new_visibility_info = HashMap::with_capacity(old_visibility_info.len());
        for func_name_result in debug_info.lines().enumerate().filter_map(|(i_line, line)| {
            if line.starts_with("[DEBUG:func_enter") {
                Some(
                    line.split_ascii_whitespace()
                        .last()
                        .map(str::to_string)
                        .with_context(|| {
                            format!("the {}-th line of debug_info is invalid format", i_line + 1)
                        }),
                )
            } else {
                None
            }
        }) {
            let func_name = func_name_result?;
            let entry = old_visibility_info
                .remove_entry(&func_name)
                .map_or((func_name, true), |entry| entry);
            new_visibility_info.insert(entry.0, entry.1);
        }
        let mut new_visibility_info = VisibilityInfo::from(new_visibility_info);
        new_visibility_info.sort();
        Ok(new_visibility_info)
    }
}
