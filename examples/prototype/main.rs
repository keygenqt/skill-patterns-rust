#[allow(dead_code)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Clone, Copy)]
struct Shape {
    pub x: u32,
    pub y: u32,
    pub radius: u32,
}

fn main() {
    let shape = Shape::default();
    // Copy
    let mut shape2 = shape;
    // Clone
    let mut shape3 = shape.clone();
    // Change
    shape2.radius = 45;
    shape3.radius = 90;
    // Output
    println!("Shape 1: {:?}", shape);
    println!("Shape 2: {:?}", shape2);
    println!("Shape 3: {:?}", shape3);
}
