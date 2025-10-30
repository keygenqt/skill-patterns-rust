use patterns_rust::ComponentHelp;

trait Component {
    fn print_info(&self);
}

pub trait ComponentHelp {
    fn print_help(&self);
}

#[allow(dead_code)]
#[derive(ComponentHelp)]
struct Button {
    name: String
    // other data
}

impl Default for Button {
    fn default() -> Self {
        Self { name: "Info: my button.".to_string() }
    }
}

impl Component for Button {
    fn print_info(&self) {
        println!("{}", self.name)
    }
}

#[allow(dead_code)]
struct Dialog {
    name: String,
    size: usize,
    // other data
}

impl Default for Dialog {
    fn default() -> Self {
        Self { name: "Info: modal dialog.".to_string(), size: 100 }
    }
}

impl Component for Dialog {
    fn print_info(&self) {
        println!("{}", self.name)
    }
}

impl ComponentHelp for Dialog {
    fn print_help(&self) {
        println!("Help: it's component with custom help!")
    }
}

trait ContainerComponent: ComponentHelp + Component {}

impl ContainerComponent for Button  {}
impl ContainerComponent for Dialog  {}

struct Container {
    children: Vec<Box<dyn ContainerComponent>>
}

impl Container {
    fn new(children: Vec<Box<dyn ContainerComponent>>) -> Self {
        Self {children}
    }

    fn print_info(&self) {
        for component in &self.children {
            component.print_info();
        }
    }

    fn print_help(&self) {
        for component in &self.children {
            component.print_help();
        }
    }
}

fn main() {
    let container = Container::new(vec![
        Box::new(Button::default()),
        Box::new(Button::default()),
        Box::new(Dialog::default()),
        Box::new(Dialog::default()),
    ]);

    container.print_info();
    println!("================================");
    container.print_help();
}
