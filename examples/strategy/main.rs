trait Strategy {
    fn execute(&self, a: i32, b: i32) -> String;
}

#[derive(Default)]
struct StrategyAdd;

#[derive(Default)]
struct StrategySubtract;

impl Strategy for StrategyAdd {
    fn execute(&self, a: i32, b: i32) -> String {
        format!("{}", a + b)
    }
}

impl Strategy for StrategySubtract {
    fn execute(&self, a: i32, b: i32) -> String {
        format!("{}", a - b)
    }
}

struct Context {
    strategy: Option<Box<dyn Strategy>>,
}

impl Context {
    fn new() -> Self {
        Self { strategy: None }
    }

    fn add(&mut self, strategy: Box<dyn Strategy>) {
        self.strategy = Some(strategy)
    }

    fn execute_strategy(&self, a: i32, b: i32) -> Result<String, Box<dyn std::error::Error>> {
        match &self.strategy {
            Some(strategy) => Ok(strategy.execute(a, b)),
            None => Err("strategy not add")?,
        }
    }
}

fn main() {
    let mut context = Context::new();
    if rand::random_bool(0.5) {
        context.add(Box::new(StrategyAdd::default()));
    } else {
        context.add(Box::new(StrategySubtract::default()));
    }
    println!("Strategy result: {:?}", context.execute_strategy(5, 4))
}
