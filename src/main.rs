use rppal::i2c::I2c;
use ssd1306::{Builder, mode::GraphicsMode, interface::DisplayInterface};
use rppal::gpio::Gpio;
use rppal::gpio::InputPin;
use rppal::gpio::Level;

mod launchkey;
mod menu;

struct Button {
    pin: Box<InputPin>,
    last_state: Level,
    has_been_pressed: bool
}

impl Button {
    fn new(pin: InputPin) -> Button {
        let v = pin.read();
        Button {
            pin: Box::new(pin),
            last_state: v,
            has_been_pressed: v == Level::High
        }
    }

    fn poll(&mut self) {
        if self.pin.is_low() && self.last_state == Level::High {
            self.has_been_pressed = true;
            self.last_state = Level::Low;
        } else if self.pin.is_high() {
            self.has_been_pressed = false;
            self.last_state = Level::High;
        }
    }

    fn was_pressed(&mut self) -> bool {
        if self.has_been_pressed {
            self.has_been_pressed = false;
            return true;
        } else {
            return false;
        }
    }
}

struct ButtonSet {
    a: Button,
    b: Button,
    c: Button,
    up: Button,
    down: Button,
    left: Button,
    right: Button
}

impl ButtonSet {
    fn poll_all(&mut self) {
        self.a.poll();
        self.b.poll();
        self.c.poll();
        self.up.poll();
        self.down.poll();
        self.left.poll();
        self.right.poll();
    }
}

fn main() {
    let mut last_pin = false;
    let mut i2c = I2c::new().expect("Could not create I2C Device");
    i2c.set_slave_address(0x3c).expect("Could not select device");

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().expect("Could not init device");

    let launchkey = launchkey::LaunchKey::new();

    let mut main_menu = menu::Menu::new();

    main_menu.add_entry("Terminal");
    main_menu.add_entry("Network");
    main_menu.add_entry("USBs");
    main_menu.add_entry("Shutdown");

    let gpio = Gpio::new().expect("Could not init board");

    let get_button = |n: u8| {
        return Button::new(gpio.get(n)
            .expect("Could not get pin")
            .into_input_pullup());
    };

    let mut buttons: ButtonSet = ButtonSet {
        a: get_button(5),
        b: get_button(6),
        c: get_button(4),
        up: get_button(17),
        down: get_button(22),
        left: get_button(23),
        right: get_button(27) // ???
    };

    loop {
        buttons.poll_all();


        if buttons.a.was_pressed() { }
        if buttons.b.was_pressed() { }
        if buttons.c.was_pressed() { }
        if buttons.left.was_pressed() { }
        if buttons.right.was_pressed() { }

        if buttons.up.was_pressed() { main_menu.prev_entry() }
        if buttons.down.was_pressed() { main_menu.next_entry() }

        disp.clear();
        main_menu.render(&mut disp);
        disp.flush();
    }
}
