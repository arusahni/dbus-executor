mod executor;

use executor::{ExecParams, Executor};
use nix::sys::wait::waitpid;
use nix::unistd::{execve, fork, ForkResult};
use std::collections::HashMap;
use std::{error::Error, ffi::CString, sync::mpsc::channel};
use tracing::{debug, error, info, trace};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use zbus::blocking::connection;

/// Execute the provided command
fn exec_command(params: ExecParams) {
    let cmd = CString::new(params.cmd).expect("Invalid command");

    // The first element of an args array should be the name of the command
    let mut args: Vec<CString> = vec![cmd.clone()];
    args.extend(
        params
            .args
            .iter()
            .map(|arg| CString::new(arg.as_str()).expect("Invalid argument")),
    );

    let mut envvars: HashMap<_, _> = std::env::vars().collect();
    if let Some(vars) = params.env.as_ref() {
        // Overrwrite any system envvars with ones set by the caller
        envvars.extend(vars.clone());
    }
    let env: Vec<CString> = envvars
        .iter()
        .map(|(k, v)| CString::new(format!("{}={}", k, v)).expect("Invalid environment variable"))
        .collect();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            trace!("Fork parent: {:?}", child);
            waitpid(child, None).expect("Failed to wait for child process");
        }
        Ok(ForkResult::Child) => {
            execve(&cmd, &args, &env).expect("Failed to execute commands");
        }
        Err(err) => error!(%err, "fork failed"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::try_from_env("DBUS_EXEC_LOG")
                .unwrap_or_else(|_| EnvFilter::new("dbus_executor=info")),
        )
        .init();
    let (sender, receiver) = channel();
    let executor = Executor { sender };
    let _connection = connection::Builder::session()?
        .name("net.arusahni.DbusExecutor")?
        .serve_at("/net/arusahni/DbusExecutor", executor)?
        .build()?;
    info!("Service started");
    if let Ok(params) = receiver.recv() {
        debug!(?params, "received params");
        exec_command(params);
    };
    Ok(())
}
