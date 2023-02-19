use glib::ExitCode;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("org.olle.NeovimWizard")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Neovim Wizard")
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));

        window.show();
    });

    app.run()
}
