use std::{sync::Arc, path::PathBuf};
use tokio::runtime::Runtime;
use adw::prelude::AdwApplicationWindowExt;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt, FileChooserExt, FileExt};
use relm4::{send, adw, Sender, AppUpdate, RelmApp, RelmComponent};

mod watch;
mod scanner;


enum Message {
    SetView(View),
    DeviceSelected(bluer::Address),
    DeviceConnected(bluer::Address),
    FirmwareUpdate(PathBuf),
    Notification(String),
}


struct Components {
    watch: RelmComponent<watch::Model, Model>,
    scanner: RelmComponent<scanner::Model, Model>,
}

impl relm4::Components<Model> for Components {
    fn init_components(parent_model: &Model, parent_sender: Sender<Message>) -> Self {
        Self {
            watch: RelmComponent::new(parent_model, parent_sender.clone()),
            scanner: RelmComponent::new(parent_model, parent_sender),
        }
    }

    fn connect_parent(&mut self, parent_widgets: &Widgets) {
        self.watch.connect_parent(parent_widgets);
        self.scanner.connect_parent(parent_widgets);
    }
}


struct Model {
    // UI state
    active_view: View,
    is_connected: bool,
    // Non-UI state
    runtime: Runtime,
    adapter: Arc<bluer::Adapter>,
    toast_overlay: adw::ToastOverlay,
}

impl Model {
    fn notify(&self, message: &str) {
        self.toast_overlay.add_toast(&adw::Toast::new(message));
    }
}

impl relm4::Model for Model {
    type Msg = Message;
    type Widgets = Widgets;
    type Components = Components;
}

impl AppUpdate for Model {
    fn update(&mut self, msg: Message, components: &Components, sender: Sender<Message>) -> bool {
        match msg {
            Message::SetView(view) => {
                self.active_view = view;
            }
            Message::DeviceSelected(address) => {
                match self.adapter.device(address) {
                    Ok(device) => {
                        self.runtime.spawn(async move {
                            match device.connect().await {
                                Ok(()) => sender.send(Message::DeviceConnected(address)).unwrap(),
                                Err(error) => eprintln!("Error: {}", error),
                            }
                        });
                    }
                    Err(error) => self.notify(&format!("Error: {}", error)),
                }
            }
            Message::DeviceConnected(address) => {
                println!("Connected: {}", address.to_string());
                self.is_connected = true;
                self.active_view = View::Main;
                match self.adapter.device(address) {
                    Ok(device) => components.watch.send(watch::Message::Connected(device)).unwrap(),
                    Err(error) => self.notify(&format!("Error: {}", error)),
                }
            }
            Message::FirmwareUpdate(filename) => {
                components.watch.send(watch::Message::FirmwareUpdate(filename));
                sender.send(Message::SetView(View::FirmwareUpdate));
            }
            Message::Notification(message) => {
                self.notify(&message);
            }
        }
        true
    }
}

#[relm4::widget]
impl relm4::Widgets<Model, ()> for Widgets {
    view! {
        adw::ApplicationWindow {
            set_default_width: 480,
            set_default_height: 640,
            set_content = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                append = &adw::HeaderBar {
                    set_title_widget = Some(&gtk::Label) {
                        set_label: watch!(match model.active_view {
                            View::Main => "WatchMate",
                            View::Scan => "Devices",
                            View::FileChooser => "Choose DFU file",
                            View::FirmwareUpdate => "Firmware Upgrade",
                        }),
                    },
                    pack_start = &gtk::Button {
                        set_label: "Back",
                        set_icon_name: "go-previous-symbolic",
                        set_visible: watch!(model.active_view != View::Main),
                        connect_clicked(sender) => move |_| {
                            send!(sender, Message::SetView(View::Main));
                        },
                    },
                    pack_start = &gtk::Button {
                        set_label: "Devices",
                        set_icon_name: watch!(if model.is_connected {
                            "bluetooth-symbolic"
                        } else {
                            "bluetooth-disconnected-symbolic"
                        }),
                        set_visible: watch!(model.active_view == View::Main),
                        connect_clicked(sender) => move |_| {
                            send!(sender, Message::SetView(View::Scan));
                        },
                    },
                    pack_start = &gtk::Button {
                        set_label: "Open",
                        set_icon_name: "document-send-symbolic",
                        // set_sensitive: watch!(file_chooser.file().is_some()),
                        set_visible: watch!(model.active_view == View::FileChooser),
                        connect_clicked(sender, file_chooser) => move |_| {
                            if let Some(file) = file_chooser.file() {
                                send!(sender, Message::FirmwareUpdate(file.path().unwrap()));
                            }
                        },
                    }
                },
                append = &Clone::clone(&model.toast_overlay) -> adw::ToastOverlay {
                    set_child = Some(&gtk::Stack) {
                        add_named(Some("main_view")) = &adw::Clamp {
                            set_maximum_size: 400,
                            // set_visible: watch!(components.watch.model.device.is_some()),
                            set_child: Some(components.watch.root_widget()),
                        },
                        add_named(Some("scan_view")) = &adw::Clamp {
                            set_maximum_size: 400,
                            set_child: Some(components.scanner.root_widget()),
                        },
                        add_named(Some("file_view")): file_chooser = &gtk::FileChooserWidget {
                            set_action: gtk::FileChooserAction::Open,
                            set_filter = &gtk::FileFilter {
                                add_pattern: "*.zip"
                            },
                        },
                        add_named(Some("fwupd_view")) = &adw::Clamp {
                            set_maximum_size: 400,
                        },
                        set_visible_child_name: watch!(match model.active_view {
                            View::Main => "main_view",
                            View::Scan => "scan_view",
                            View::FileChooser => "file_view",
                            View::FirmwareUpdate => "fwupd_view",
                        }),
                    },
                },
            },
        }
    }
}


#[derive(Debug, PartialEq)]
enum View {
    Main,
    Scan,
    FileChooser,
    FirmwareUpdate,
}


pub fn run(runtime: Runtime, adapter: Arc<bluer::Adapter>) {
    // Init GTK before libadwaita (ToastOverlay)
    gtk::init().unwrap();

    // Init model
    let model = Model {
        // UI state
        active_view: View::Scan,
        is_connected: false,
        // System
        runtime,
        adapter,
        // Widget handles
        toast_overlay: adw::ToastOverlay::new(),
    };

    // Run app
    let app = RelmApp::new(model);
    app.run();
}
