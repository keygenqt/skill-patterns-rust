use std::env;

trait Button {
    fn render(&self) -> &'static str;
}

struct MacosButton;
struct LinuxButton;

impl Button for MacosButton {
    fn render(&self) -> &'static str {
        "> Macos button ---------------"
    }
}

impl Button for LinuxButton {
    fn render(&self) -> &'static str {
        "> Linux button ---------------"
    }
}

trait Dialog {
    fn create_button(&self) -> Box<dyn Button>;
}

struct MacosDialog;
struct LinuxDialog;

impl MacosDialog {
    fn default() -> MacosDialog {
        MacosDialog {}
    }
}

impl LinuxDialog {
    fn default() -> LinuxDialog {
        LinuxDialog {}
    }
}

impl Dialog for MacosDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacosButton {})
    }
}

impl Dialog for LinuxDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(LinuxButton {})
    }
}

struct Application {
    dialog: Box<dyn Dialog>,
}

impl Application {
    fn default() -> Application {
        match env::consts::OS {
            "macos" => Application {
                dialog: Box::new(MacosDialog::default()),
            },
            "linux" => Application {
                dialog: Box::new(LinuxDialog::default()),
            },
            _ => panic!("OS not supported"),
        }
    }

    fn dialog_render(&self) -> String {
        let button = self.dialog.create_button();
        let render = format!("{}\n{}", "> Dialog ---------------------", button.render());
        render
    }
}

fn main() {
    let application = Application::default();
    println!("{}", application.dialog_render());
}
