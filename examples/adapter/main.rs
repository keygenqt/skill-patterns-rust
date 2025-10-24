// Old
struct SquarePeg {
    width: u32,
}

impl SquarePeg {
    fn new(width: u32) -> Self {
        Self { width }
    }
}

// New
struct RoundPeg {
    radius: u32,
}

impl RoundPeg {
    fn new(radius: u32) -> Self {
        Self { radius }
    }
}

// Feature
struct RoundHole {
    radius: u32,
}

impl RoundHole {
    fn new(radius: u32) -> Self {
        Self { radius }
    }

    fn is_fits(&self, peg: RoundPeg) -> bool {
        self.radius >= peg.radius
    }
}

// Adapter
struct SquarePegAdapter {
    peg: RoundPeg,
}

impl SquarePegAdapter {
    fn new(peg: SquarePeg) -> Self {
        Self {
            peg: RoundPeg {
                radius: peg.width * (2 as u32).isqrt() / 2,
            },
        }
    }
}

fn main() {
    let hole = RoundHole::new(10);
    let new_peg = RoundPeg::new(5);
    let old_peg = SquarePeg::new(5);

    println!("1. Fits peg: {}", hole.is_fits(new_peg));

    // not work
    // println!("Fits peg: {}", hole.is_fits(old_peg));

    let adapter = SquarePegAdapter::new(old_peg);
    println!("2. Fits peg: {}", hole.is_fits(adapter.peg));

}
