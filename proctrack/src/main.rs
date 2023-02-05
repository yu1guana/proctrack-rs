// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

mod system;
mod visibility_info;

use self::visibility_info::VisibilityInfo;
use anyhow::{Context, Result};
use clap::{Parser, ValueHint};
use proctrack::funclog::{funclog, methodlog_static};
use proctrack::typename_derive::TypeName;
use std::fs;
use std::path::PathBuf;

#[funclog]
fn main() -> Result<()> {
    Cli::run()
}
#[derive(Parser, TypeName)]
#[clap(author, version, about, after_help = concat!("Repository: ", env!("CARGO_PKG_REPOSITORY")))]
pub(crate) struct Cli {
    #[clap(
        value_hint(ValueHint::FilePath),
        help = "Debug information written by using proctrack crate."
    )]
    debug_info: PathBuf,
    #[clap(
        value_hint(ValueHint::FilePath),
        help = "TOML file which saves visibility information. If this file does not exsist, a new file is created."
    )]
    visibility_info: PathBuf,
}

impl Cli {
    #[methodlog_static]
    fn run() -> Result<()> {
        let args = Cli::parse();
        let debug_info = fs::read_to_string(&args.debug_info)
            .with_context(|| format!("failed to read {}", args.debug_info.display()))?;
        let visibility_info =
            VisibilityInfo::try_new(&args.visibility_info)?.update_by_debug_info(&debug_info)?;
        system::activate(
            args.debug_info,
            args.visibility_info,
            debug_info,
            visibility_info,
        )
    }
}
