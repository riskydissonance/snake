use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use crate::{Vector2};

pub(crate) struct Rectangle {
    min_x: i32,
    min_y: i32,
    width: i32,
    height: i32,
    color: Color,
}

impl Rectangle {
    pub(crate) const fn new(min_x: i32, min_y: i32, width: i32, height: i32, color: Color) -> Rectangle {
        Rectangle {
            min_x,
            min_y,
            width,
            height,
            color
        }
    }
    pub(crate) fn intersects(&self, point: &Vector2) -> bool {
        (point.x > self.min_x && point.x <= (self.min_x + self.width)) && (point.y > self.min_y && point.y <= (self.min_y + self.height))
    }

    pub(crate) fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.min_x, self.min_y, self.width, self.height, self.color);
    }
}