trait Button {
    fn paint(&self) -> &'static str;
}

trait Checkbox {
    fn paint(&self) -> &'static str;
}

trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

struct MacButton;

impl Button for MacButton {
    fn paint(&self) -> &'static str {
        "> macOS button -----------------"
    }
}

struct LinuxButton;

impl Button for LinuxButton {
    fn paint(&self) -> &'static str {
        "> Linux button -----------------"
    }
}

struct MacCheckbox;

impl Checkbox for MacCheckbox {
    fn paint(&self) -> &'static str {
        "> macOS checkbox ---------------"
    }
}

struct LinuxCheckbox;

impl Checkbox for LinuxCheckbox {
    fn paint(&self) -> &'static str {
        "> Linux checkbox ---------------"
    }
}

struct MacosFactory;

impl GUIFactory for MacosFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton {})
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox {})
    }
}

struct LinuxFactory;

impl GUIFactory for LinuxFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(LinuxButton {})
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(LinuxCheckbox {})
    }
}

#[allow(dead_code)]
enum GUIFactories {
    MACOS(MacosFactory),
    LINUX(LinuxFactory),
}

#[allow(dead_code)]
impl GUIFactories {
    fn new_macos() -> GUIFactories {
        GUIFactories::MACOS(MacosFactory {})
    }

    fn new_linux() -> GUIFactories {
        GUIFactories::LINUX(LinuxFactory {})
    }

    fn create_button(&self) -> Box<dyn Button> {
        match self {
            GUIFactories::MACOS(macos_factory) => macos_factory.create_button(),
            GUIFactories::LINUX(linux_factory) => linux_factory.create_button(),
        }
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        match self {
            GUIFactories::MACOS(macos_factory) => macos_factory.create_checkbox(),
            GUIFactories::LINUX(linux_factory) => linux_factory.create_checkbox(),
        }
    }
}

struct Application {
    factory: GUIFactories,
}

impl Application {
    pub fn new(factory: GUIFactories) -> Application {
        Application { factory }
    }

    pub fn button(&self) -> Box<dyn Button> {
        self.factory.create_button()
    }

    pub fn checkbox(&self) -> Box<dyn Checkbox> {
        self.factory.create_checkbox()
    }
}

fn main() {
    let factory = GUIFactories::new_macos();

    match factory {
        GUIFactories::MACOS(_) => println!("- Application macOS ------------"),
        GUIFactories::LINUX(_) => println!("- Application Linux ------------"),
    };

    let app = Application::new(factory);

    println!("{}", app.button().paint());
    println!("{}", app.checkbox().paint());
}
