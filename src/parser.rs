use std::fs;
use std::error::Error;
use std::env;
use gtk4::gio;
use gtk4::gio::prelude::*;

pub fn parse_config() -> Result<Vec<String>, Box<dyn Error>> {
    let path = env::home_dir().expect("Could not find home").join(".config/neuroBar/conf.neu");
    let conf = fs::read_to_string(path)?;
    let pinned: Vec<String> = conf.trim().strip_prefix("pinned: ").unwrap().split(",").map(|s| s.trim().to_string()).collect();

    Ok(pinned)
}

pub fn get_app_icon(name: String) -> gtk4::Image {
    let apps = gio::AppInfo::all();

    for app in apps {
        if app.name().to_lowercase() == name.to_lowercase() {
            if let Some(icon) = app.icon() {
                return gtk4::Image::from_gicon(&icon);
            }
        } else {
            continue;
        }
    }
    panic!("{name} not found or has no icon!");
}

pub fn launch_app(name: String) {
    let apps = gio::AppInfo::all();

    for app in apps {
        if app.name() == name {
            app.launch(&[], None::<&gio::AppLaunchContext>,).expect("App not launched");
            break
        }
    }
}