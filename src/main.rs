use anyhow::Error;
use clap::Parser;
use kitshell_cmd::{
    popup::{self},
    types::{Cli, Commands},
};

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.cmd {
        Commands::Popup(popup) => popup::handle_popup(&popup)?,
    }

    Ok(())
}
