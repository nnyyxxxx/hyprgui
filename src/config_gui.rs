use gtk::{
    prelude::*, Application, ApplicationWindow, Box, Button, FileChooserAction, FileChooserDialog,
    FileFilter, HeaderBar, Orientation, ResponseType,
};

pub struct ConfigGUI {
    pub window: ApplicationWindow,
    box_: Box,
    header_bar: HeaderBar,
    pub open_button: Button,
}

impl ConfigGUI {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hyprland Configuration GUI")
            .default_width(800)
            .default_height(600)
            .build();

        let box_ = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .build();

        let header_bar = HeaderBar::builder()
            .show_title_buttons(false)
            .decoration_layout("")
            .build();

        let open_button = Button::builder().label("Open Configuration File").build();

        header_bar.set_title_widget(Some(&open_button));

        window.set_titlebar(Some(&header_bar));
        window.set_child(Some(&box_));

        Self {
            window,
            box_,
            header_bar,
            open_button,
        }
    }

    pub fn open_config_file(&self) {
        let dialog = FileChooserDialog::builder()
            .title("Open Hyprland Configuration File")
            .action(FileChooserAction::Open)
            .transient_for(&self.window)
            .modal(true)
            .build();

        dialog.add_button("Cancel", ResponseType::Cancel);
        dialog.add_button("Open", ResponseType::Accept);

        let filter = FileFilter::new();
        filter.set_name(Some("Configuration files"));
        filter.add_pattern("*.conf");

        dialog.add_filter(&filter);

        dialog.connect_response(move |dialog, response| {
            match response {
                ResponseType::Accept => {
                    if let Some(filename) = dialog.file() {
                        println!("{:?}", filename);
                    }
                }
                ResponseType::Cancel => {
                    println!("canceled");
                }
                _ => {}
            }
            dialog.close();
        });

        dialog.show();
    }
}
