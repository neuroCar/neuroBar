use gtk4::{gio};
use std::error::Error;
use std::ffi::OsStr;

pub fn set_volume(volume: f64) {
    let volume_string = volume.to_string();
    let volume = volume_string + "%";
    let _ = gio::Subprocess::newv(&[OsStr::new("wpctl"), OsStr::new("set-volume"), OsStr::new("@DEFAULT_SINK@"), OsStr::new(&volume)], gio::SubprocessFlags::NONE);
}

pub fn get_volume() -> f64 {
    let subprocess = gio::Subprocess::newv(&[OsStr::new("wpctl"), OsStr::new("get-volume"), OsStr::new("@DEFAULT_SINK@")], gio::SubprocessFlags::STDOUT_PIPE).expect("Failed to run wpctl");
    let (stdout, _) = subprocess.communicate_utf8(None, gio::Cancellable::NONE).expect("Failed to return a value");
    let vol = stdout.unwrap();
    let vol_no_prefix = vol.get(8..12).expect("Failed to get volume");
    let mut volume = vol_no_prefix.parse::<f64>().expect("Unable to parse");
    volume *= 100.0;
    return volume
}

pub fn get_volume_icon() -> Result<&'static str, Box<dyn Error>> {
    let volume = get_volume();

    if volume <= 33.00 {
        return Ok("audio-volume-low-symbolic")
    } else if volume > 33.00 && volume <= 66.00 {
        return Ok("audio-volume-medium-symbolic")
    } else {
        return Ok("audio-volume-high-symbolic")
    }
}