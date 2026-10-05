use std::result::Result;
use std::error::Error;
use gtk4::gio::prelude::*;
use gtk4::gio;

pub async fn get_icon() -> Result<String, Box<dyn Error>> {
    let conn: gio::DBusConnection = gio::bus_get_future(gio::BusType::System).await?;

    let battery: gio::DBusProxy = gio::DBusProxy::new_future(
        &conn,
        gio::DBusProxyFlags::NONE,
        None,
        Some("org.freedesktop.UPower"),
        "/org/freedesktop/UPower/devices/DisplayDevice",
        "org.freedesktop.UPower.Device",
    ).await?;

    let icon_name = battery.cached_property("IconName").ok_or("Icon name not found")?.get::<String>().ok_or("Icon name has wrong type")?;
    Ok(icon_name)
}

pub async fn get_percentage() -> Result<f64, Box<dyn Error>> {
    let conn: gio::DBusConnection = gio::bus_get_future(gio::BusType::System).await?;

    let battery: gio::DBusProxy = gio::DBusProxy::new_future(
        &conn,
        gio::DBusProxyFlags::NONE,
        None,
        Some("org.freedesktop.UPower"),
        "/org/freedesktop/UPower/devices/DisplayDevice",
        "org.freedesktop.UPower.Device",
    ).await?;

    let percentage: f64 = battery.cached_property("Percentage").ok_or("Percentage not found")?.get::<f64>().ok_or("Percentage has wrong type")?;
    Ok(percentage)
}