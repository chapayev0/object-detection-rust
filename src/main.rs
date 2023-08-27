extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gtk::{Builder, TextView, TextBuffer};

fn perform_object_detection(image_data: Vec<f32>, model_path: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    // Initialize TensorFlow
    let mut tf = tf::Session::new()?;

    // Load a pre-trained object detection model
    let graph = tf::Graph::new();
    let graph_def = std::fs::read(model_path)?;
    graph.import_graph_def(&graph_def, &tf)?;

    // Prepare input data (e.g., an image)

    // Run inference
    let input_op = graph.operation_by_name_required("input_tensor")?;
    let output_op = graph.operation_by_name_required("output_tensor")?;

    let mut args = tf::SessionRunArgs::new();
    args.add_feed(&input_op, 0, &image_data);
    let output = args.request_fetch(&output_op, 0);

    tf.run(&mut args)?;

    let result: Vec<f32> = args.fetch(output)?;

    // Process the detection results

    Ok(result)
}

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Load Glade UI file
    let glade_src = include_str!("ui/ui.glade");
    let builder = Builder::from_string(glade_src);

    let image_data: Vec<f32> = "/image/img1.jpg"
    let model_path = "/model";

    let detection_results = perform_object_detection(image_data, model_path)?;


    // Get main window object from the Glade UI
    let main_window: gtk::Window = builder
        .object("main_window")
        .expect("Couldn't get main_window");

    // Get the button object from the Glade UI
    let det_btn: gtk::Button = builder
        .object("det_img_btn")
        .expect("Couldn't get my_button");


    let details_box: TextView = builder
        .object("detail_box")
        .expect("Couldn't get my_textview");


    // Connect the "destroy" signal to quit the application
    main_window.connect_destroy(|_| {
        gtk::main_quit();
    });

    // Show the main window and start the GTK main loop
    main_window.show_all();
    gtk::main();
}

fn image_load(text_view: &TextView){

    println!("shashika");

    if let Some(buffer) = text_view.buffer() {
        buffer.set_text("clicked");
    }




}
