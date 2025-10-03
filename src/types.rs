use bon::Builder;
use clap::{Parser, Subcommand, command};
use serde::{Deserialize, Serialize};

use crate::popup::PopupKind;

pub const KITSHELL_IPC_CLIENT_SPEC_VER: u32 = 2025_10_02;

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcMessage {
    /// IPC content
    pub content: IpcContent,

    /// Kitshell IPC specification version
    pub version: u32,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
pub struct IpcContent {
    opt1: String,
    opt2: Option<String>,
    opt3: Option<String>,
    opt4: Option<String>,
    opt5: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcReply {
    pub received_successfully: bool,
    pub server_spec_version: u32,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Send command to Kitshell UI
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Open specified popup (if the specified popup is already opened,
    /// it will close)
    #[command(subcommand)]
    Popup(PopupKind),
}
