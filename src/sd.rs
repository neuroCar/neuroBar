use zbus::*;

pub async fn systemd_fns(cmd_type: &str) -> zbus::Result<()> {
    let connection = Connection::system().await?;
    let proxy_m = Proxy::new(
        &connection,
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        "org.freedesktop.login1.Manager",
    ).await?;

    match cmd_type {
        "sleep" => {
            let _: () = proxy_m.call("Suspend", &(false,)).await?;
        },
        "logout" => {
            let session_id: String = proxy_m.call("GetSessionByPID", &(std::process::id(),)).await?;
            let _: () = proxy_m.call("TerminateSession", &(session_id,)).await?;
        },
        "reboot" => {
            let _: () = proxy_m.call("Reboot", &(false,)).await?;
        },
        "shutdown" => {
            let _: () = proxy_m.call("PowerOff", &(false,)).await?;
        },
        _ => { return Err(zbus::Error::Failure("Command not found".into())) }
    }
    Ok(())
}