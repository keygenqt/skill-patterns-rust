use std::sync::{LazyLock, Mutex, atomic::{AtomicUsize, Ordering}};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
static HISTORY: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(vec![]));

#[derive(Default)]
pub struct Editor {
    text: String,
    subscribes: Vec<fn(&str)>,
}

impl Editor {
    pub fn change(&mut self, text: &str) {
        self.text = text.to_string();
        for subscribe in &self.subscribes {
            (subscribe)(&self.text)
        }
    }

    pub fn subscribe(&mut self, subscribe: fn(&str)) {
        self.subscribes.push(subscribe);
    }

    pub fn unsubscribe(&mut self) {
        self.subscribes.clear();
    }

    pub fn is_subscribe(&self) -> bool {
        !self.subscribes.is_empty()
    }
}

fn main() {
    let mut editor = Editor::default();

    // Subscribe for change text
    editor.subscribe(|text: &str| {
        COUNTER.fetch_add(1, Ordering::Relaxed);
        HISTORY.lock().unwrap().push(text.to_string());
    });

    // Change text
    editor.change("> Set my best text1!");
    editor.change("> Set my best text2!");
    editor.change("> Set my best text3!");

    // Unsubscribe
    editor.unsubscribe();

    // Not save history after unsubscribe
    editor.change("> Text after unsubscribe.");

    println!("== Events ===============-");
    println!("Events call count: {}", COUNTER.load(Ordering::Relaxed));

    println!("== History ===============");
    for history in HISTORY.lock().unwrap().iter() {
        println!("{}", history);
    }
}
