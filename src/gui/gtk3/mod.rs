use gtk::{self};
use gtk::prelude::*;

pub fn launch() {
    gtk::init().unwrap_or_else(|_| panic!("panic!"));

    let builder = gtk::Builder::new_from_string(include_str!("main.ui"));
    let window: gtk::Window =builder.get_object("app_window").unwrap();

    window.show_all();

    // And then kill the event:

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();

}
