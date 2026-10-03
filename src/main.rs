use gtk4::{self as gtk};
use gtk::{glib, prelude::*};
use gtk4_layer_shell::*;
use gtk_blueprint::*;

mod sd;
mod loading;
mod battery;

fn activate(app: &gtk::Application) {
    let builder = gtk::Builder::from_string(include_blp!("src/main.blp"));
    let win = builder.object::<gtk::ApplicationWindow>("window").expect("Window not found");
    let menu_btn = builder.object::<gtk::Button>("menuBtn").expect("Menu button not found");
    menu_btn.connect_clicked({
        let pop = builder.object::<gtk::Popover>("start").expect("Start not found");
        let popover = pop.clone();
        move |_| {
            popover.popup();
        }
    });
 
    win.init_layer_shell(); win.set_layer(Layer::Top); win.auto_exclusive_zone_enable();
    for anchor in [Edge::Bottom, Edge::Left, Edge::Right] { win.set_anchor(anchor, true); }

    loading::load_css();
    loading::load_apps(builder.clone());
    loading::load_session_buttons(builder.clone());
    loading::load_tray(builder.clone());

    win.set_application(Some(app));
    win.present();
}

fn main() -> glib::ExitCode {
    gtk::init().unwrap();
    let app = gtk::Application::builder()
        .application_id("org.neuro.neuroBar")
        .build();

    app.connect_activate(|app| {
        activate(app);
    });

    app.run()
}