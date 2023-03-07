use std::io;

use clap::CommandFactory;
use clap_complete::generate;
use clap_complete::Shell;

use super::*;

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
