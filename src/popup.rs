use clap::Subcommand;

use crate::{ipc, types::IpcContent};

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

pub fn handle_popup(popup: &PopupKind) {
    let result = IpcContent::builder()
        .opt1("popup".to_owned())
        .maybe_opt2(match popup {
            PopupKind::Notif => Some("notif".to_owned()),
            PopupKind::Quicksettings => Some("quicksettings".to_owned()),
            PopupKind::Appmenu => Some("appmenu".to_owned()),
            PopupKind::Mpris => Some("mpris".to_owned()),
        })
        .build();

    let _ = ipc::send(result);
}
