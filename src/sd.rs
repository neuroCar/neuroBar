use std::collections::HashMap;
use zbus::*;
use zvariant;

pub async fn power_profile_fns(cmd: &str, opt: Option<&str>) -> zbus::Result<String> {
    let connection = Connection::system().await?;
    let proxy_ppd = Proxy::new(
        &connection,
        "org.freedesktop.UPower.PowerProfiles",
        "/org/freedesktop/UPower/PowerProfiles",
        "org.freedesktop.UPower.PowerProfiles",
    ).await?;

    match cmd {
        "get_profiles" => {
            let profiles: Vec<HashMap<String, zvariant::OwnedValue>> = proxy_ppd.get_property("Profiles").await?;
            for profile in profiles {
                if let Some(value) = profile.get("Profile") {
                    let name: String = value.clone().try_into()?;
                    println!("Profile: {name}");
                }
            }
        }
        "current_profile" => {
            let profile: String = proxy_ppd.get_property("ActiveProfile").await?;
            return Ok(profile)
        }
        "set_profile" => {
            proxy_ppd.set_property("ActiveProfile", opt.unwrap()).await?;
        }
        _ => { println!("Function does not exist!"); }
    }
    Ok(String::from(""))
}

pub async fn systemd_session_fns(cmd_type: &str) -> zbus::Result<()> {
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