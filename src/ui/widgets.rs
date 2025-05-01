pub fn vbox() -> gtk::Box {
    gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(0)
        .build()
}

pub fn input_bar() -> gtk::Entry {
    gtk::Entry::builder()
        .placeholder_text("Type your message here...")
        .build()
}

