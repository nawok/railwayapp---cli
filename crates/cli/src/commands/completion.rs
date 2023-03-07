use std::io;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};

/// Generate completion script
#[derive(Parser)]
pub struct Args {
    shell: Shell,
}

pub async fn command(args: Args, _json: bool) -> Result<()> {
    generate(
        args.shell,
        &mut crate::Args::command(),
        "railway",
        &mut io::stdout(),
    );
    Ok(())
}
