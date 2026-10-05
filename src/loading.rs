use gtk4 as gtk;
use gtk::{glib, gdk, gio, prelude::*};
use std::ffi::OsStr;
use std::time::Duration;
use std::collections::HashMap;

use crate::sd;
use crate::battery;
use crate::parser;
use crate::volume;

fn set_time(clock: gtk::Label) {
    glib::timeout_add_local(Duration::from_secs(1), move || {
        let time = chrono::Local::now(); 
        let time_str = time.format("%H:%M").to_string();
        clock.set_label(&time_str);

        glib::ControlFlow::Continue
    });
}

fn set_battery_label(battery: gtk::Label, battery_icon: gtk::Image) {
    let battery_label = battery.clone();
    let bat_ico = battery_icon.clone();
    glib::MainContext::default().spawn_local(async move {
        loop {
            match battery::get_icon().await {
                Ok(icon_name) => {
                    bat_ico.set_icon_name(Some(&icon_name));
                }
                Err(err) => {
                    eprintln!("Failed to get battery percentage: {err}");
                }
            }
            match battery::get_percentage().await {
                Ok(percentage) => {
                    battery_label.set_text(&format!("{percentage}%"));
                }
                Err(err) => {
                    eprintln!("Failed to get battery percentage: {err}");
                }
            }
            glib::timeout_future_seconds(5).await;
        }});
}

fn set_ppd_icon(ppd_icon: gtk::Image) {
    let ppd_icon = ppd_icon.clone();
    glib::MainContext::default().spawn_local(async move {
        loop {
            match sd::power_profile_fns("current_profile", None).await {
                Ok(profile) => {
                    let icons = HashMap::from([(String::from("power-saver"), "power-profile-power-saver-symbolic"), (String::from("balanced"), "power-profile-balanced-symbolic"), (String::from("performance"), "power-profile-performance-symbolic")]);
                    ppd_icon.set_icon_name(Some(icons[&profile]));
                }
                _ => { eprintln!("Failed to set icon"); }
            }
            glib::timeout_future_seconds(1).await;
        }
    });
}

fn load_volume(builder: gtk::Builder) {
    let audio_ico = builder.object::<gtk::Image>("audioIco").expect("Could not load icon");
    let mute_btn = builder.object::<gtk::Button>("muteBtn").expect("Could not load btn");
    let _mute_btn_ico = builder.object::<gtk::Image>("muteBtnIco").expect("Could not load icon");
    let vol_slider = builder.object::<gtk::Scale>("volSlider").expect("Could not load slider");

    mute_btn.connect_clicked(move |_| {
        let _ = gio::Subprocess::newv(&[OsStr::new("wpctl"), OsStr::new("set-mute"), OsStr::new("@DEFAULT_SINK@"), OsStr::new("toggle")], gio::SubprocessFlags::STDOUT_PIPE).expect("Failed to run wpctl");
    });

    audio_ico.set_icon_name(Some(volume::get_volume_icon().expect("Volume not found")));

    vol_slider.set_value(volume::get_volume());
    vol_slider.set_range(0.00, 100.00);
    vol_slider.connect_value_changed(move |scale| {
        volume::set_volume(scale.value());
        audio_ico.set_icon_name(Some(volume::get_volume_icon().expect("Volume not found")));
    });
}

pub fn load_tray(builder: gtk::Builder) {
    let clock = builder.object::<gtk::Label>("clock").expect("Label not found");
    
    let battery = builder.object::<gtk::Label>("battery").expect("Label not found");
    let battery_icon = builder.object::<gtk::Image>("batteryIco").expect("Icon not found");
    
    let ppd_icon = builder.object::<gtk::Image>("profileIco").expect("Icon not found");
    let ppd_btn_list = ["powerSaving", "balanced", "performance"];
    let profiles = ["power-saver", "balanced", "performance"];
    for i in 0..profiles.len() {
        let btn = builder.object::<gtk::Button>(ppd_btn_list[i]).expect("Check Button not found");
        btn.connect_clicked(move |_| {
            let i = i.clone();
            let profiles = profiles.clone();
            glib::MainContext::default().spawn_local(async move {
                match sd::power_profile_fns("set_profile", Some(profiles[i])).await {
                    Ok(_) => { println!("Sucess"); }
                    _ => { println!("Failed"); }
                }
            });
        });
    }
    set_time(clock.clone());
    set_battery_label(battery.clone(), battery_icon.clone());
    set_ppd_icon(ppd_icon.clone());
    load_volume(builder.clone());
}

pub fn load_shortcuts(builder: gtk::Builder) {
    let shortcut_box = builder.object::<gtk::Box>("shortcuts").expect("Box not found");
    let pinned = parser::parse_config().unwrap();
    for app in pinned {
        let icon = parser::get_app_icon(app.clone());
        icon.set_pixel_size(24);
        let btn = gtk::Button::builder()
            .child(&icon)
            .css_classes(["widget"])
            .build();
        btn.connect_clicked(move |_| {
            parser::launch_app(app.clone());
        });
        shortcut_box.append(&btn);
    }
}

pub fn load_session_buttons(builder: gtk::Builder) {
    // TODO: Add settings app
    let cmd_list = ["sleep", "logout", "reboot", "shutdown"];
    
    for i in 0..cmd_list.len() {
        let btn = builder.object::<gtk::Button>(cmd_list[i]).expect("Button not found");
        btn.connect_clicked(move |_| {
            glib::MainContext::default().spawn_local(async move { if let Err(e) = sd::systemd_session_fns(cmd_list[i]).await {
                eprintln!("Failed to {}: {e}", cmd_list[i]);
            }});
        });
    }
}

pub fn load_apps(builder: gtk::Builder) {
    let app_list_widget = builder.object::<gtk::FlowBox>("apps").expect("Apps not available");
    let mut apps = gio::AppInfo::all();

    apps.retain(|app| app.should_show());
    apps.sort_by_key(|app| app.name().to_lowercase());

    for app in apps {
        if !app.should_show() {
            continue;
        }

        let button = gtk::Button::new();
        button.add_css_class("widget");
        let box_ = gtk::Box::new(gtk::Orientation::Vertical, 6);

        if let Some(icon) = app.icon() {
            let img = gtk::Image::from_gicon(&icon);
            img.set_pixel_size(32);
            box_.append(&img);
        }

        let lbl = gtk::Label::new(Some(&app.name()));
        lbl.set_wrap(true);
        lbl.set_max_width_chars(12);

        box_.append(&lbl);
        button.set_child(Some(&box_));

        let app = app.clone();
        button.connect_clicked(move |_| {
            if let Err(error) = app.launch(&[], None::<&gio::AppLaunchContext>,) {
                eprintln!("Failed to launch {}: {error}", app.name());
            }
        });
        app_list_widget.insert(&button, -1);
    }
}

pub fn load_css() {
    let disp = gdk::Display::default().expect("No display");
    let settings = gtk::Settings::default().unwrap();

    let css_provider = gtk::CssProvider::new();
    css_provider.set_prefers_color_scheme(settings.gtk_interface_color_scheme());
    css_provider.load_from_path("src/main.css");
    gtk::style_context_add_provider_for_display(
        &disp,
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}