trait Memento<T> {
    fn restore(self) -> T;
}

#[derive(Default, Debug)]
struct MyData {
    state: u32,
}

impl MyData {
    fn up(&mut self) {
        self.state = self.state + 1
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            state: self.state
        }
    }
}

struct Snapshot {
    state: u32,
}

impl Memento<MyData> for Snapshot {
    fn restore(self) -> MyData {
        MyData {
            state: self.state,
        }
    }
}

fn main() {
    let mut data = MyData::default();
    let mut history = Vec::<Snapshot>::new();

    history.push(data.snapshot());
    println!("== State init");
    println!("{:?}", data);

    data.up();

    history.push(data.snapshot());
    println!("== State up");
    println!("{:?}", data);

    println!("== History");
    while let Some(snapshot) = history.pop() {
        println!("{:?}", snapshot.restore());
    }
}
