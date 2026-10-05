use gtk4::{self as gtk};
use gtk::{glib, prelude::*};
use gtk4_layer_shell::*;
use gtk_blueprint::*;

mod sd;
mod loading;
mod battery;
mod parser;
mod volume;

fn activate(app: &gtk::Application) {
    let builder: gtk4::Builder = gtk::Builder::from_string(include_blp!("src/main.blp"));
    let win: gtk4::ApplicationWindow = builder.object::<gtk::ApplicationWindow>("window").expect("Window not found");
    win.init_layer_shell(); win.set_layer(Layer::Top); win.auto_exclusive_zone_enable();
    for anchor in [Edge::Bottom, Edge::Left, Edge::Right] { win.set_anchor(anchor, true); }

    loading::load_css();
    loading::load_apps(builder.clone());
    loading::load_session_buttons(builder.clone());
    loading::load_shortcuts(builder.clone());
    loading::load_tray(builder.clone());

    win.set_application(Some(app));
    win.present();
}

fn main() -> glib::ExitCode {
    gtk::init().unwrap();
    let app: gtk4::Application = gtk::Application::builder()
        .application_id("org.neuro.neuroBar")
        .build();

    app.connect_activate(|app| {
        activate(app);
    });

    parser::parse_config().expect("Unable to get config");

    app.run()
}