use std::{collections::HashMap, sync::mpsc::Sender};
use tracing::debug;
use zbus::zvariant;

#[derive(Clone, Debug, zvariant::Type)]
pub struct ExecParams {
    pub cmd: String,
    pub args: Vec<String>,
    pub env: zvariant::Optional<HashMap<String, String>>,
}

pub struct Executor {
    pub sender: Sender<ExecParams>,
}

#[zbus::interface(name = "net.arusahni.DbusExecutor.Exec")]
impl Executor {
    /// Execute a command without args
    fn cmd(&self, cmd: &str) -> zbus::fdo::Result<()> {
        debug!(?cmd, "Received command");
        self.sender
            .send(ExecParams {
                cmd: cmd.to_string(),
                args: Vec::new(),
                env: None.into(),
            })
            .map_err(|_| zbus::Error::Failure("Failed to send command".into()))?;
        Ok(())
    }

    /// Execute a command with args
    fn cmd_args(&self, cmd: &str, args: Vec<String>) -> zbus::fdo::Result<()> {
        debug!(%cmd, ?args, "Received command with args");
        self.sender
            .send(ExecParams {
                cmd: cmd.to_string(),
                args,
                env: None.into(),
            })
            .map_err(|_| zbus::Error::Failure("Failed to send command".into()))?;
        Ok(())
    }

    /// Execute a command with args and an environment
    fn cmd_args_env(
        &self,
        cmd: &str,
        args: Vec<String>,
        env: HashMap<String, String>,
    ) -> zbus::fdo::Result<()> {
        debug!(%cmd, ?args, ?env, "Received command with args and env");
        self.sender
            .send(ExecParams {
                cmd: cmd.to_string(),
                args,
                env: Some(env).into(),
            })
            .map_err(|_| zbus::Error::Failure("Failed to send command".into()))?;
        Ok(())
    }
}
