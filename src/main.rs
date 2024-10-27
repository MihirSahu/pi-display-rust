mod utils;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Mutex, Arc};

use std::collections::HashMap;
use std::sync::OnceLock;

use glib::clone;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Label, Box, Orientation, CssProvider};
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}
fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let data: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let data_order: Vec<String> = vec!["cat_fact".to_string(), "temperature".to_string()];
    let mut pointer = 0;
    let label_text = Rc::new(RefCell::new(String::from("Temp")));

    data.lock().unwrap().insert("temperature".to_string(), "Temperature".to_string());
    data.lock().unwrap().insert("cat_fact".to_string(), "Cat Fact".to_string());

    let label = Label::builder()
        .label(label_text.borrow().to_string())
        .build();

    // Add css classes
    //button.add_css_class("button1");
    label.add_css_class("label1");
    
    // Create a vertical box container to hold multiple widgets
    let vbox = Box::new(Orientation::Vertical, 10);  // 5 is the spacing between widgets

    vbox.set_halign(gtk::Align::Center);
    vbox.set_valign(gtk::Align::Center);

    vbox.append(&label);

    let data_clone = data.clone();
    runtime().spawn(clone!(
        async move {
            loop {
                //update_variables(data_clone.clone(), sender.clone()).await;
                update_variables(data_clone.clone()).await;
            }
        }
    ));

    // Create channel that can hold at most 1 message at a time
    let (sender, receiver) = async_channel::bounded(1);
    runtime().spawn(clone!(
        #[strong]
        sender,
        async move {
            loop {
                sender
                    .send(true)
                    .await
                    .expect("The channel needs to be open.");
                sleep(Duration::from_secs(5)).await;
            }
        }
    ));
    
    // The main loop executes the asynchronous block
    glib::spawn_future_local(async move {
            while let Ok(_response) = receiver.recv().await {
                //println!("Updating label");
                label.set_label(data.lock().unwrap().get(&data_order[pointer]).unwrap());
                pointer = (pointer + 1) % data_order.len();
            }
        }
    );

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pi Display")
        .child(&vbox)
        .build();

    window.fullscreen();

    // Present window
    window.present();
}

//async fn update_variables(data: Arc<Mutex<HashMap<String, String>>>, sender: Sender<Arc<Mutex<HashMap<String, String>>>>) {
async fn update_variables(data: Arc<Mutex<HashMap<String, String>>>) {
    //println!("Updating variables");
    let cat_fact = utils::get_cat_fact().await;
    let temperature = utils::get_temperature().await;

    data.lock().unwrap().insert("cat_fact".to_string(), cat_fact);
    data.lock().unwrap().insert("temperature".to_string(), temperature);

    sleep(Duration::from_secs(10)).await;
}