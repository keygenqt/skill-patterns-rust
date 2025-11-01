trait Flower {}

trait FlowerCare {

    fn fertilizer(&self);
    fn water(&self);

    fn care(&self) {
        self.fertilizer();
        self.water();
    }
}

#[derive(Default)]
struct Rose;

impl Flower for Rose {}

impl FlowerCare for Rose {
    fn fertilizer(&self) {
        println!("> Rose: add fertilizer number 1.")
    }

    fn water(&self) {
        println!("> Rose: water with 200 milliliters.")
    }
}

#[derive(Default)]
struct Lily;

impl Flower for Lily {}

impl FlowerCare for Lily {
    fn fertilizer(&self) {
        println!("> Lily: add fertilizer number 2.")
    }

    fn water(&self) {
        println!("> Lily: water with 100 milliliters.")
    }
}

trait CommonFlower: Flower + FlowerCare {}

impl CommonFlower for Rose  {}
impl CommonFlower for Lily  {}

struct Flowers {
    items: Vec<Box<dyn CommonFlower>>
}

impl Flowers {
    fn new(items: Vec<Box<dyn CommonFlower>>) -> Self {
        Self { items }
    }

    fn care(&self) {
        for flower in &self.items {
            flower.care();
        }
    }
}

fn main() {
    let flowers = Flowers::new(vec![
        Box::new(Rose::default()),
        Box::new(Lily::default()),
        Box::new(Rose::default()),
        Box::new(Lily::default()),
    ]);

    flowers.care();
}
