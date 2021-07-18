#![warn(clippy::all, clippy::pedantic)]
use gio::AppInfoExt;
use gtk::prelude::*;

pub fn run() -> Option<String> {
    let dialog = gtk::AppChooserDialog::for_content_type::<gtk::Window>(
        None,
        gtk::DialogFlags::MODAL,
        "image/svg+xml",
    );
    let chooser = dialog.get_widget();
    let chooser = chooser.downcast::<gtk::AppChooser>().unwrap();
    let res = dialog.run();
    let command: Option<String> = if res == gtk::ResponseType::Ok {
        let app = chooser.get_app_info();
        let cmd = match app {
            Some(a) => a.get_commandline(),
            None => None,
        };
        if cmd.is_some() {
            Some(cmd
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string()
            )
        } else {
            None
        }
    } else {
        None
    };

    unsafe { dialog.destroy(); }
    command
}
