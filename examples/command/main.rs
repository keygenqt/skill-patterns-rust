trait Command {
    fn execute(&self, editor: &mut Editor, value: &str);
}

struct SetCommand;

impl SetCommand {
    fn new() -> Box<Self> {
        Box::new(SetCommand {})
    }
}

impl Command for SetCommand {
    fn execute(&self, editor: &mut Editor, value: &str) {
        editor.text = value.to_string()
    }
}

struct AddCommand;

impl AddCommand {
    fn new() -> Box<Self> {
        Box::new(AddCommand {})
    }
}

impl Command for AddCommand {
    fn execute(&self, editor: &mut Editor, value: &str) {
        editor.text.push_str(value);
    }
}

#[derive(Default)]
struct Editor {
    text: String
}

impl Editor {
    fn execute(&mut self, command: Box<dyn Command>, value: &str) {
        command.execute(self, value);
    }

    fn print(&self) {
        println!("Editor: {}", &self.text)
    }
}

fn main() {
    let mut editor = Editor::default();
    editor.execute(SetCommand::new(), "My best text");
    editor.execute(AddCommand::new(), ".");
    editor.print();
}
