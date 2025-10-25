enum Color {
    RED,
    BLUE,
    YELLOW
}

trait Graphic {
    fn draw(&self) -> String;
}

struct Dot {
    tabs: usize
}

impl Dot {
    fn new(tabs: usize) -> Self {
        Dot { tabs }
    }
}

impl Graphic for Dot {
    fn draw(&self) -> String {
        let tabs = String::from_utf8(vec![b'\t'; self.tabs]).unwrap() + ".";
        format!("{}.", tabs)
    }
}

struct DotColor {
    color: Color,
    dot: Dot,
}

impl DotColor {
    fn new(color: Color, tabs: usize) -> Self {
        DotColor {
            color,
            dot: Dot::new(tabs),
         }
    }
}

impl Graphic for DotColor {
    fn draw(&self) -> String {
        match self.color {
            Color::RED => format!("\x1b[1m\x1b[91m{}\x1b[0m", self.dot.draw()),
            Color::BLUE => format!("\x1b[1m\x1b[94m{}\x1b[0m", self.dot.draw()),
            Color::YELLOW => format!("\x1b[1m\x1b[93m{}\x1b[0m", self.dot.draw()),
        }
    }
}

struct GraphicCompound {
    graphics: Vec<Box<dyn Graphic>>
}

impl GraphicCompound {
    fn new() -> Self {
        Self { graphics: vec![] }
    }

    fn add(&mut self, graphic: Box<dyn Graphic>) {
        self.graphics.push(graphic);
    }
}

fn main() {
    let mut compound = GraphicCompound::new();

    let dot1 = Dot::new(0);
    let dot2 = Dot::new(1);
    let dot3 = Dot::new(2);

    let dot_color1 = DotColor::new(Color::RED, 0);
    let dot_color2 = DotColor::new(Color::BLUE, 1);
    let dot_color3 = DotColor::new(Color::YELLOW, 2);

    compound.add(Box::new(dot1));
    compound.add(Box::new(dot2));
    compound.add(Box::new(dot3));

    compound.add(Box::new(dot_color1));
    compound.add(Box::new(dot_color2));
    compound.add(Box::new(dot_color3));

    for a in compound.graphics {
        println!("{}", a.draw());
    }
}
