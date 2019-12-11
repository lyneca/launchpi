use rppal::i2c::I2c;
use ssd1306::{Builder, mode::GraphicsMode, interface::DisplayInterface};
use rand::Rng;

struct Star { x: i32, y: i32, z: i32 }
struct Offset { x: i32, y: i32, z: i32 }

impl Star {
    fn new() -> Star {
        let mut rng = rand::thread_rng();
        Star {
            x: rng.gen_range(0, 128),
            y: rng.gen_range(0, 64),
            z: rng.gen_range(1, 20)
        }
    }

    fn render<DI: DisplayInterface>(&self, disp: &mut GraphicsMode<DI>, offset: &Offset) {
        let x = (self.x + (offset.x as f32 * (2.0 / self.z as f32)) as i32) % 128;
        let y = (self.y + (offset.y as f32 * (2.0 / self.z as f32)) as i32) % 64;
        if x > 0 && y > 0 {
            disp.set_pixel(x as u32, y as u32, 1);
        }
    }
}

impl Offset {
    fn new() -> Offset {
        Offset { x: 0, y: 0, z: 0 }
    }

    fn change(&mut self, x: i32, y: i32, z: i32) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

fn main() {
    let mut i2c = I2c::new().expect("Could not create I2C Device");
    i2c.set_slave_address(0x3c).expect("Could not select device");

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
    disp.init();

    let mut offset = Offset::new();

    let mut stars = vec![];
    for i in 0..100 {
        stars.push(Star::new());
    }

    loop {
        disp.clear();
        offset.change(1, 0, 0);
        for star in stars.iter() {
            star.render(&mut disp, &offset);
        }
        disp.flush();
    }
}
