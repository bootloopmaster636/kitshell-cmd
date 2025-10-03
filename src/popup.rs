use anyhow::{Error, bail};
use clap::Subcommand;

use crate::{
    ipc,
    types::{IpcContent, KITSHELL_IPC_CLIENT_SPEC_VER},
};

#[derive(Subcommand)]
pub enum PopupKind {
    /// Notification and clock popup
    Notif,

    /// Quick settings popup
    Quicksettings,

    /// Appmenu popup
    Appmenu,

    /// Media player popup
    Mpris,
}

pub fn handle_popup(popup: &PopupKind) -> Result<(), Error> {
    let result = IpcContent::builder()
        .opt1("popup".to_owned())
        .maybe_opt2(match popup {
            PopupKind::Notif => Some("notif".to_owned()),
            PopupKind::Quicksettings => Some("quicksettings".to_owned()),
            PopupKind::Appmenu => Some("appmenu".to_owned()),
            PopupKind::Mpris => Some("mpris".to_owned()),
        })
        .build();

    match ipc::send(result) {
        Ok(val) => match val {
            Some(reply) => {
                if reply.server_spec_version != KITSHELL_IPC_CLIENT_SPEC_VER {
                    println!("kitshell-ipc version and Kitshell version does not match");
                    println!(
                        "kitshell-ipc spec version is {}, and kitshell spec version is {}",
                        KITSHELL_IPC_CLIENT_SPEC_VER, reply.server_spec_version
                    );
                    println!("Please upgrade both to the latest version!");
                } else {
                    if reply.received_successfully {
                        println!("Received successfully")
                    }
                }
            }
            None => {}
        },
        Err(_) => bail!("Failed to send IPC command"),
    }

    Ok(())
}
