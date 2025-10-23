#[derive(Debug)]
enum Color {
    NONE,
    RED,
}

#[derive(Debug)]
struct Car {
    engine: String,
    color: Color,
    is_gps: bool,
}

impl Car {
    fn default() -> Car {
        Car {
            engine: "None".to_string(),
            color: Color::NONE,
            is_gps: false,
        }
    }
}

struct CarBuilder {
    car: Car
}

impl CarBuilder {
    fn default() -> CarBuilder {
        CarBuilder {
            car: Car::default()
        }
    }

    fn result(&self) -> &Car {
        &self.car
    }
}

#[derive(Debug)]
struct ManualCar {
    engine: String,
    color: Color,
    is_gps: bool,
}

impl ManualCar {
    fn default() -> ManualCar {
        ManualCar {
            engine: "None".to_string(),
            color: Color::NONE,
            is_gps: false,
        }
    }
}

struct ManualCarBuilder {
    manual_car: ManualCar
}

impl ManualCarBuilder {
    fn default() -> ManualCarBuilder {
        ManualCarBuilder {
            manual_car: ManualCar::default()
        }
    }

    fn result(&self) -> &ManualCar {
        &self.manual_car
    }
}

trait Builder {
    fn reset(&mut self);
    fn engine(&mut self, engine: String);
    fn color(&mut self, color: Color);
    fn gps(&mut self, is_gps: bool);
}

impl Builder for CarBuilder {
    fn reset(&mut self) {
        self.car = Car::default();
    }

    fn engine(&mut self, engine: String) {
        self.car.engine = engine;
    }

    fn color(&mut self, color: Color) {
        self.car.color = color;
    }

    fn gps(&mut self, is_gps: bool) {
        self.car.is_gps = is_gps;
    }
}

impl Builder for ManualCarBuilder {
    fn reset(&mut self) {
        self.manual_car = ManualCar::default();
    }

    fn engine(&mut self, engine: String) {
        self.manual_car.engine = engine;
    }

    fn color(&mut self, color: Color) {
        self.manual_car.color = color;
    }

    fn gps(&mut self, is_gps: bool) {
        self.manual_car.is_gps = is_gps;
    }
}

struct CarDirector;

impl CarDirector {
    fn sports_car<T: Builder>(builder: &mut T) {
        builder.reset();
        builder.engine("The best".to_string());
        builder.color(Color::RED);
        builder.gps(true);
    }
}


fn main() {
    let mut car_builder = CarBuilder::default();
    CarDirector::sports_car(&mut car_builder);
    println!("{:?}", car_builder.result());

    let mut manual_car_builder = ManualCarBuilder::default();
    CarDirector::sports_car(&mut manual_car_builder);
    println!("{:?}", manual_car_builder.result());
}
