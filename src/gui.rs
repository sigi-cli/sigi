use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};

pub fn run() -> i32 {
    let app = Application::builder()
        .application_id("so.dang.cool.sigi-gtk")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |button| {
        button.set_label("Hello World!");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Sigi")
        .child(&button)
        .build();

    window.present();
}