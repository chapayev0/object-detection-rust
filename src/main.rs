extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gtk::Builder;

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Load Glade UI file
    let glade_src = include_str!("ui/ui.glade");
    let builder = Builder::from_string(glade_src);

    // Get main window object from the Glade UI
    let main_window: gtk::Window = builder
        .object("main_window")
        .expect("Couldn't get main_window");

    // Connect the "destroy" signal to quit the application
    main_window.connect_destroy(|_| {
        gtk::main_quit();
    });

    // Show the main window and start the GTK main loop
    main_window.show_all();
    gtk::main();
}
