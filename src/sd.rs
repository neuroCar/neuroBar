use zbus::*;
use zbus_systemd::*;

pub async fn systemd_fns(cmd_type: &str) -> zbus::Result<()> {
    let connection = Connection::system().await?;
    let manager = zbus_systemd::login1::ManagerProxy::new(&connection).await?;

    match cmd_type {
        "sleep" => {
            manager.suspend(true).await?;
        },
        "logout" => {
            let session_path = manager
                .get_session_by_pid(std::process::id())
                .await?;
            let session = zbus_systemd::login1::SessionProxy::builder(&connection)
                .path(session_path)?
                .build()
                .await?;
            let session_id = session.id().await?;
            manager.terminate_session(session_id).await?;
        },
        "reboot" => {
            manager.reboot(true).await?;
        },
        "shutdown" => {
            manager.power_off(true).await?;
        },
        _ => { return Err(zbus::Error::Failure("shutdown failed".into())) }
    }
    Ok(())
}