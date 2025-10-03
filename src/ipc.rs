use anyhow::{Error, anyhow};
use linux_ipc::IpcChannel;

use crate::types::{IpcContent, IpcMessage, KITSHELL_SPEC_VER};

pub fn send(content: IpcContent) -> Result<Option<IpcMessage>, Error> {
    let mut channel = connect()?;

    let message = IpcMessage {
        content,
        version: KITSHELL_SPEC_VER,
    };
    let response = channel.send::<_, IpcMessage>(message)?;

    if let Some(response) = &response {
        println!("Kitshell IPC: Received {:?}", &response);
    }

    Ok(response)
}

fn connect() -> Result<IpcChannel, Error> {
    match IpcChannel::connect("/tmp/kitshell.sock") {
        Ok(val) => Ok(val),
        Err(_) => Err(anyhow!("Cannot create socket")),
    }
}
