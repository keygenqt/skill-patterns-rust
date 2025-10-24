#[allow(dead_code)]
trait Device {
    fn enable(&mut self);
    fn disable(&mut self);
    fn volume(&mut self, volume: u8);
}

#[derive(Debug)]
#[derive(Clone, Copy)]
struct Radio {
    is_enable: bool,
    volume: u8,
}

impl Default for Radio {
    fn default() -> Self {
        Self {
            is_enable: false,
            volume: 30,
        }
    }
}

impl Device for Radio {
    fn enable(&mut self) {
        self.is_enable = true
    }

    fn disable(&mut self) {
        self.is_enable = false
    }

    fn volume(&mut self, volume: u8) {
        self.volume = volume
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
struct Tv {
    is_enable: bool,
    volume: u8,
}

impl Default for Tv {
    fn default() -> Self {
        Self {
            is_enable: false,
            volume: 20,
        }
    }
}

impl Device for Tv {
    fn enable(&mut self) {
        self.is_enable = true
    }

    fn disable(&mut self) {
        self.is_enable = false
    }

    fn volume(&mut self, volume: u8) {
        self.volume = volume
    }
}

#[derive(Debug)]
struct DeviceAdvanced<T: Device> {
    device: T,
}

impl<T: Device> DeviceAdvanced<T> {
    pub fn new(device: T) -> Self {
        Self { device }
    }

    pub fn mute(&mut self) {
        self.device.volume(0);
    }
}

fn main() {
    let mut radio = Radio::default();
    let mut tv = Tv::default();

    println!("--------------");
    println!("Radio enable: {}", radio.is_enable);
    println!("Tv is enable: {}", tv.is_enable);

    device_enable(&mut radio);
    device_enable(&mut tv);

    println!("--------------");
    println!("Radio enable: {}", radio.is_enable);
    println!("Tv is enable: {}", tv.is_enable);

    let mut radio_advanced = DeviceAdvanced::new(radio);
    radio_advanced.mute();

    let mut tv_advanced = DeviceAdvanced::new(tv);
    tv_advanced.mute();

    println!("--------------");
    println!("{:?}", radio_advanced);
    println!("{:?}", tv_advanced);

}

fn device_enable<T: Device>(device: &mut T) {
    device.enable();
}
