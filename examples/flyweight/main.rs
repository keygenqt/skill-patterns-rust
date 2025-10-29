use crate::factory::TextFactory;

struct Point {
    value: String,
    // Many other data for type
}

impl Point {
    fn new(value: String) -> Self {
        Self { value }
    }
}

// Context object
struct Text {
    value: String,
    point_index: usize,
}

impl Text {
    fn new(value: String, point_index: usize) -> Self {
        Self { value, point_index }
    }

    fn print(&self) {
        println!("{}{}", self.value, factory::TextFactory::points().get(self.point_index).unwrap().value)
    }
}

mod factory {
    use std::sync::{LazyLock, Mutex, MutexGuard};
    use crate::{Point, Text};

    static TYPES: LazyLock<Mutex<Vec<Point>>> = LazyLock::new(|| Mutex::new(vec![]));

    pub struct TextFactory;

    impl TextFactory {
        pub fn points() -> MutexGuard<'static, Vec<Point>> {
            TYPES.lock().unwrap()
        }

        pub fn len() -> usize {
            TYPES.lock().unwrap().len()
        }

        pub fn get_text(value: &str, point: &str) -> Text {
            let mut types = TYPES.lock().unwrap();
            match types.iter().position(|p| &p.value == point) {
                Some(obj) => {
                    Text::new(value.to_string(), obj)
                },
                None => {
                    types.push(Point::new(point.to_string()));
                    Text::new(value.to_string(), types.len() - 1)
                },
            }
        }
    }
}

fn main() {
    let texts: Vec<Text> = vec![
        TextFactory::get_text("My best text", "."),
        TextFactory::get_text("My best text", "."),
        TextFactory::get_text("My best text", "."),
        TextFactory::get_text("My best text", "."),
        TextFactory::get_text("My best text", "."),
        TextFactory::get_text("我最好的标题", "。"),
        TextFactory::get_text("我最好的标题", "。"),
        TextFactory::get_text("我最好的标题", "。"),
        TextFactory::get_text("我最好的标题", "。"),
        TextFactory::get_text("我最好的标题", "。"),
    ];

    for item in texts {
        item.print();
    }

    println!("================");
    println!("Size factory: {}", TextFactory::len());
}
