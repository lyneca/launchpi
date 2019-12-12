extern crate embedded_graphics;

use std::cmp;

use ssd1306::{mode::GraphicsMode, interface::DisplayInterface};

use embedded_graphics::fonts::{Font,Font6x8};
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::image::Image1BPP;
use embedded_graphics::pixelcolor::PixelColorU8;
use embedded_graphics::primitives::Rect;
use embedded_graphics::Drawing;

pub struct Menu {
    entries: Vec<String>,
    selected: u8
}

const ARROW: &[u8] = &[
    0b0100_0000,
    0b1111_1100,
    0b1111_1100,
    0b0100_0000,
];

impl Menu {
    pub fn new() -> Menu {
        Menu {
            entries: vec![],
            selected: 0
        }
    }

    pub fn render<DI: DisplayInterface>(&self, disp: &mut GraphicsMode<DI>) {
        for (i, entry) in self.entries.iter().enumerate() {
            disp.draw(Rect::new(
                    Coord::new(0, i as i32 * 11),
                    Coord::new(117, (i + 1) as i32 * 11),
            ).with_stroke(Some(PixelColorU8(1))).into_iter());
            disp.draw(Font6x8::render_str(entry).translate(Coord::new(2, i as i32 * 11 + 2)).into_iter());
            if i as u8 == self.selected {
                disp.draw(Image1BPP::<PixelColorU8>::new(ARROW, 6, 4)
                    .translate(Coord::new(120, i as i32 * 11 + 4))
                    .into_iter()
                );
            }
        }
        let size = self.entries.len();
    }

    pub fn add_entry(&mut self, text: &str) {
        self.entries.push(String::from(&text[..cmp::min(19, text.len())]));
    }

    pub fn next_entry(&mut self) {
        self.selected += 1;
        self.selected %= self.entries.len() as u8;
    }

    pub fn prev_entry(&mut self) {
        if self.selected == 0 {
            self.selected = self.entries.len() as u8 - 1;
        } else {
            self.selected -= 1;
        }
    }
}
